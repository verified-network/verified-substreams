import "dotenv/config";
import {
  createLogger,
  format as _format,
  transports as _transports,
} from "winston";
import { Logtail } from "@logtail/node";
import { LogtailTransport } from "@logtail/winston";

const logtail = new Logtail(process.env.LOGTAIL_SOURCE_KEY);

const VerifiedCustomLevels = {
  error: 0,
  warn: 1,
  info: 2,
};

export const logger = createLogger({
  levels: VerifiedCustomLevels,
  level: process.env.LOG_LEVEL || "info", //use highest level to capture all
  format: _format.combine(
    _format.timestamp({
      format: "YYYY-MM-DD hh:mm:ss.SSS A",
    }),
    _format.errors({ stack: true }),
    _format.splat(),
    _format.simple()
  ),
  defaultMeta: { service: "Verified Substreams" },
  transports: [new LogtailTransport(logtail)],
});

if (process.env.NODE_ENV !== "production") {
  logger.add(
    new _transports.Console({
      format: _format.combine(_format.simple()),
    })
  );
}
