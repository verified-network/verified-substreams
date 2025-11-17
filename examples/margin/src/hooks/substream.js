import { useEffect, useRef, useState, useCallback } from "react";
import {
  createRequest,
  streamBlocks,
  createAuthInterceptor,
  createRegistry,
  fetchSubstream,
} from "@substreams/core";
import { createConnectTransport } from "@connectrpc/connect-web";
import protobuf from "protobufjs";

/**
 * React hook for streaming live Substreams data
 *
 * @param {string} endpoint - Substreams endpoint URL
 * @param {string} token - Auth token
 * @param {string} spkgUrl - Path url to the .spkg file
 * @param {string} module - Module name inside substream(e.g. map_event)
 * @param {number} startBlock - Starting block number
 * @param {string} protoPath - Path url to .proto file
 * @param {string} protoMessage - Message type name, (e.g. contract.v1.Events)
 * @param {string} storageKey - Key for storing cursor in localStorage
 */
export const useSubstreams = (
  endpoint,
  token,
  spkgUrl,
  module,
  startBlock,
  protoPath,
  protoMessage,
  storageKey = "substreams_cursor_marg"
) => {
  const [messages, setMessages] = useState([]);
  const [connected, setConnected] = useState(false);
  const [cursor, setCursor] = useState(null);

  const cursorRef = useRef(null);
  const abortControllerRef = useRef(null);
  const isStreaming = useRef(false);

  // Load saved cursor from localStorage on mount
  useEffect(() => {
    const savedCursor = localStorage.getItem(storageKey);
    if (savedCursor) {
      cursorRef.current = savedCursor;
      setCursor(savedCursor);
      console.log("Resuming from saved cursor:", savedCursor);
    }
  }, [storageKey]);

  // Save cursor to ref + localStorage + state
  const saveCursor = useCallback(
    (newCursor) => {
      cursorRef.current = newCursor;
      setCursor(newCursor);
      try {
        localStorage.setItem(storageKey, newCursor);
      } catch (err) {
        console.warn("Failed to persist cursor:", err);
      }
    },
    [storageKey]
  );

  const streamSubstreams = useCallback(async () => {
    if (isStreaming.current) return;
    isStreaming.current = true;

    try {
      const pkg = await fetchSubstream(spkgUrl);
      const registry = createRegistry(pkg);
      const PROTO_ROOT = await protobuf.load(protoPath);
      const PROTO_EVENTS = PROTO_ROOT.lookupType(protoMessage);

      const transport = createConnectTransport({
        baseUrl: endpoint,
        interceptors: [createAuthInterceptor(token)],
        useBinaryFormat: true,
        jsonOptions: { typeRegistry: registry },
      });

      const request = createRequest({
        substreamPackage: pkg,
        outputModule: module,
        productionMode: true,
        startBlockNum: startBlock, // only used if no cursor exists
        startCursor: cursorRef.current ?? undefined,
      });

      const controller = new AbortController();
      abortControllerRef.current = controller;

      setConnected(true);

      for await (const statefulResponse of streamBlocks(transport, request, {
        signal: controller.signal,
      })) {
        // Always save the latest cursor immediately
        if (statefulResponse.progress?.cursor) {
          saveCursor(statefulResponse.progress.cursor);
        }

        const msg = statefulResponse?.response?.message;

        if (msg?.case === "blockScopedData") {
          const buf = msg.value?.output?.mapOutput?.value;
          if (!buf) continue;

          const decoded = PROTO_EVENTS.decode(buf);
          const event = PROTO_EVENTS.toObject(decoded, {
            longs: String,
            enums: String,
            defaults: true,
          });

          setMessages((prev) => [...prev, event]);
        }
      }
    } catch (err) {
      console.error("Substream error:", err);
      setConnected(false);

      // Retry automatically after 5 seconds
      setTimeout(() => {
        isStreaming.current = false;
        streamSubstreams();
      }, 5000);
    }
  }, [
    endpoint,
    token,
    spkgUrl,
    module,
    startBlock,
    protoPath,
    protoMessage,
    saveCursor,
  ]);

  useEffect(() => {
    streamSubstreams();

    return () => {
      abortControllerRef.current?.abort();
      isStreaming.current = false;
      setConnected(false);
    };
  }, [streamSubstreams]);

  return { messages, connected, cursor };
};
