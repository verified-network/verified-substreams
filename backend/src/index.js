import "dotenv/config";
import { networkSubstreamConfig } from "../utils/config";
import { getSubstreamNode } from "../utils/helpers";

export const delay = (ms) => new Promise((res) => setTimeout(res, ms));

export const startSubstreams = async () => {
  const networkConfigs = networkSubstreamConfig[`${process.env.NODE_ENV}`];
  for (const chainId of Object.keys(networkConfigs)) {
    const { primary, secondary, margin } = networkConfigs[chainId];
    console.log("pri: ", primary);

    let primarySubstream, secondarySubstream, marginSubstream;

    if (primary) {
      primarySubstream = getSubstreamNode(primary);
    }

    if (secondary) {
      secondarySubstream = getSubstreamNode(secondary);
    }

    if (margin) {
      marginSubstream = getSubstreamNode(margin);
    }

    const oneHour = 60 * 60 * 1000;
    if (primarySubstream) {
      await primarySubstream?.start();
      logger.log(
        "info",
        `Primary subsream started Successfully for ${chainId} chainId`
      );
      primarySubstream?.startHealthMetricsScheduler(oneHour);
      logger.log(
        "info",
        `Subscribed to hourly health and metrics log for primary substream on ${chainId} chainId`
      );
    }

    if (secondarySubstream) {
      await secondarySubstream?.start();
      logger.log(
        "info",
        `secondary subsream started Successfully for ${chainId} chainId`
      );
      secondarySubstream?.startHealthMetricsScheduler(oneHour);
      logger.log(
        "info",
        `Subscribed to hourly health and metrics log for secondary substream on ${chainId} chainId`
      );
    }

    if (marginSubstream) {
      await marginSubstream?.start();
      logger.log(
        "info",
        `margin subsream started Successfully for ${chainId} chainId`
      );
      marginSubstream?.startHealthMetricsScheduler(oneHour);
      logger.log(
        "info",
        `Subscribed to hourly health and metrics log for margin substream on ${chainId} chainId`
      );
    }

    await delay(15000); //15 seconds delay per chain???
  }

  logger.log("info", `Subsream Started Successfully for ${chainId} chainId `);

  process.on("message", async function (msg) {
    logger.log("warn", `received message from process: ${msg}`);
    if (msg === "shutdown" || msg.includes("failed to kill")) {
      setTimeout(async () => {
        logger.log(
          "error",
          "Process shutdown. will close Substreams for graceful restart"
        );

        process.exit(0);
      }, 1500);
    }
  });

  process
    .on("unhandledRejection", (rsn, prm) => {
      logger.log(
        "error",
        `Unhandled Rejection at Promise: ${prm} with reason: ${rsn}. will close Substreams for graceful restart`
      );
      process.exit(1);
    })
    .on("uncaughtException", (err) => {
      logger.log(
        "error",
        `Uncaught Exception thrown: ${err}. will close Substreams for graceful restart`
      );
      process.exit(1);
    });
};
