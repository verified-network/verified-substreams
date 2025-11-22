import {
  createRequest,
  streamBlocks,
  createAuthInterceptor,
  createRegistry,
  fetchSubstream,
} from "@substreams/core";
import { createConnectTransport } from "@connectrpc/connect-node";
import fs from "fs";
import path from "path";
import protobuf from "protobufjs";

export class SubstreamsNode {
  constructor({
    endpoint,
    token,
    spkgUrl,
    module,
    startBlock,
    protoPath,
    protoMessage,
    cursorFilePath,
    healthMetricFilePath,
    onMessage = () => {},
    onConnected = () => {},
    onError = () => {},
  }) {
    // Stream config
    this.endpoint = endpoint;
    this.token = token;
    this.spkgUrl = spkgUrl;
    this.module = module;
    this.startBlock = startBlock;
    this.protoPath = protoPath;
    this.protoMessage = protoMessage;

    // Cursor storage
    this.cursorFile = cursorFilePath;
    this.cursor = this.loadCursorFromDisk();

    // Health and Metric
    this.healthMetricFile = healthMetricFilePath;

    // Event handlers
    this.onMessage = onMessage;
    this.onConnected = onConnected;
    this.onError = onError;

    // Streaming state
    this.isStreaming = false;
    this.abortController = null;

    // Backoff settings
    this.retryCount = 0;
    this.baseDelay = 2000; // 2s
    this.maxDelay = 60000; // 60s

    // Metrics + health tracking
    this.startTime = Date.now();
    this.lastBlockTime = null;
    this.blocksProcessed = 0;
    this.eventsProcessed = 0;
    this.bytesProcessed = 0;
    this.totalRestarts = 0;

    // Scheduled health metrics interval
    this.healthInterval = null;
  }

  /** Load saved cursor from disk if exists or create new file for cursor */
  loadCursorFromDisk() {
    try {
      const dir = path.dirname(this.cursorFile);
      if (!fs.existsSync(dir)) {
        fs.mkdirSync(dir, { recursive: true });
      }

      if (!fs.existsSync(this.cursorFile)) {
        fs.writeFileSync(this.cursorFile, "");
        console.log("Cursor file created:", this.cursorFile);
        return null;
      }

      const cursor = fs.readFileSync(this.cursorFile, "utf8").trim();
      if (cursor) {
        console.log("Loaded saved cursor:", cursor);
        return cursor;
      }

      return null;
    } catch (err) {
      console.error("Failed to read or create cursor file:", err);
      return null;
    }
  }

  /** Save cursor to disk */
  saveCursor(cursor) {
    this.cursor = cursor;
    try {
      fs.writeFileSync(this.cursorFile, cursor);
    } catch (err) {
      console.error("Failed to save cursor:", err);
    }
  }

  /** Exponential backoff delay */
  getBackoffDelay() {
    const jitter = Math.floor(Math.random() * 500);
    return Math.min(
      this.baseDelay * 2 ** this.retryCount + jitter,
      this.maxDelay
    );
  }

  /** Health object */
  getHealth() {
    const now = Date.now();
    const timeSinceLastBlock = this.lastBlockTime
      ? now - this.lastBlockTime
      : Infinity;

    const healthy =
      this.isStreaming && this.lastBlockTime && timeSinceLastBlock < 30000; // last block < 30 seconds ago

    return {
      healthy,
      streaming: this.isStreaming,
      last_block_at: this.lastBlockTime
        ? new Date(this.lastBlockTime).toISOString()
        : null,
      time_since_last_block_ms: timeSinceLastBlock,
      cursor: this.cursor,
      retry_count: this.retryCount,
      next_retry_delay_ms: this.retryCount > 0 ? this.getBackoffDelay() : 0,
      reason: healthy
        ? "OK"
        : this.isStreaming
        ? "No blocks received recently"
        : "Not streaming",
    };
  }

  /** Metrics object */
  getMetrics() {
    const uptime = (Date.now() - this.startTime) / 1000;

    return {
      timestamp: new Date().toISOString(),
      uptime_seconds: uptime,
      blocks_processed: this.blocksProcessed,
      events_processed: this.eventsProcessed,
      bytes_processed: this.bytesProcessed,
      last_cursor: this.cursor,
      last_block_time: this.lastBlockTime
        ? new Date(this.lastBlockTime).toISOString()
        : null,
      retry_count: this.retryCount,
      total_restarts: this.totalRestarts,
    };
  }

  /** Automatic hourly reporting */
  startHealthMetricsScheduler(intervalMs = 3600000) {
    console.log(
      `Health & Metrics reporting enabled every ${intervalMs / 1000}s`
    );

    this.healthInterval = setInterval(() => {
      const health = this.getHealth();
      const metrics = this.getMetrics();

      console.log("===== HOURLY HEALTH CHECK =====");
      console.log(JSON.stringify(health, null, 2));

      console.log("===== HOURLY METRICS =====");
      console.log(JSON.stringify(metrics, null, 2));

      // Save to file
      try {
        fs.writeFileSync(
          healthMetricFile,
          JSON.stringify(
            {
              timestamp: new Date().toISOString(),
              health,
              metrics,
            },
            null,
            2
          )
        );
      } catch (err) {
        console.error("Failed to write health/metrics file:", err);
      }
    }, intervalMs);
  }

  stopHealthMetricsScheduler() {
    if (this.healthInterval) {
      clearInterval(this.healthInterval);
      console.log("Stopped health & metrics reporting.");
    }
  }

  /** Start streaming */
  async start() {
    if (this.isStreaming) return;

    this.isStreaming = true;
    this.totalRestarts++;

    try {
      const pkg = await fetchSubstream(this.spkgUrl);
      const registry = createRegistry(pkg);

      const PROTO_ROOT = await protobuf.load(this.protoPath);
      const PROTO_EVENTS = PROTO_ROOT.lookupType(this.protoMessage);

      const transport = createConnectTransport({
        baseUrl: this.endpoint,
        interceptors: [createAuthInterceptor(this.token)],
        useBinaryFormat: true,
        jsonOptions: { typeRegistry: registry },
      });

      const request = createRequest({
        substreamPackage: pkg,
        outputModule: this.module,
        productionMode: true,
        startBlockNum: this.cursor ? undefined : this.startBlock,
        startCursor: this.cursor ?? undefined,
      });

      this.onConnected(true);

      this.abortController = new AbortController();

      for await (const response of streamBlocks(transport, request, {
        signal: this.abortController.signal,
      })) {
        // Reset retry counter once healthy stream resumes
        if (this.retryCount !== 0) {
          console.log("Stream stable again — resetting retry counter");
          this.retryCount = 0;
        }

        // Update block timestamp
        this.lastBlockTime = Date.now();
        this.blocksProcessed++;

        // Save cursor
        if (response.progress?.cursor) {
          this.saveCursor(response.progress.cursor);
        }

        // Decode events
        const msg = response?.response?.message;
        if (msg?.case === "blockScopedData") {
          const buf = msg.value?.output?.mapOutput?.value;
          if (!buf) continue;

          this.bytesProcessed += buf.length;

          const decoded = PROTO_EVENTS.decode(buf);
          const event = PROTO_EVENTS.toObject(decoded, {
            longs: String,
            enums: String,
            defaults: true,
          });

          this.eventsProcessed++;
          this.onMessage(event);
        }
      }
    } catch (err) {
      console.error("Streaming error:", err);

      this.onError(err);
      this.onConnected(false);

      this.isStreaming = false;

      const delay = this.getBackoffDelay();
      console.log(
        `Retrying stream in ${delay}ms (retry ${this.retryCount + 1})`
      );

      this.retryCount++;

      setTimeout(() => {
        this.start();
      }, delay);
    }
  }

  stop() {
    this.abortController?.abort();
    this.isStreaming = false;
    console.log("Substreams stopped.");
  }
}
