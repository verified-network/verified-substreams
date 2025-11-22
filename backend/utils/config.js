import "dotenv/config";

export const networkSubstreamConfig = {
  production: {},
  developement: {
    11155111: {
      primary: {
        substreamEndpoint: "https://sepolia.substreams.pinax.network:443",
        substreamStartBlock: 5492879,
        substreamToken: process.env.REACT_APP_SUBSTREAMS_API_TOKEN,
        substreamSpkgUrl:
          "https://raw.githubusercontent.com/verified-network/verified-substreams/new-update/primarypool_factory/primarypool-factory-v0.1.0.spkg",
        substreamModule: "map_subscriptions_per_pool",
        substreamProtoUrl:
          "https://raw.githubusercontent.com/verified-network/verified-substreams/new-update/primarypool_factory/proto/contract.proto",
        substreamMessage: "contract.v1.PrimaryPoolSubscriptionsList",
        healthAndMetricsPath: "../logs/primaryHealthAndMetric11155111.json",
        cursorPath: "../logs/primaryCursor11155111.txt",
      },
      secondary: {
        substreamEndpoint: "https://sepolia.substreams.pinax.network:443",
        substreamStartBlock: 6589966,
        substreamToken: process.env.REACT_APP_SUBSTREAMS_API_TOKEN,
        substreamSpkgUrl:
          "https://raw.githubusercontent.com/verified-network/verified-substreams/new-update/secondarypool_factory/secondarypool-factory-v0.1.0.spkg",
        substreamModule: "map_trades_per_pool",
        substreamProtoUrl:
          "https://raw.githubusercontent.com/verified-network/verified-substreams/new-update/secondarypool_factory/proto/contract.proto",
        substreamMessage: "contract.v1.SecondaryPoolTradeReportsList",
        healthAndMetricsPath: "../logs/secondaryHealthAndMetric11155111.json",
        cursorPath: "../logs/secondaryCursor11155111.txt",
      },
      margin: {
        substreamEndpoint: "https://sepolia.substreams.pinax.network:443",
        substreamStartBlock: 5492908,
        substreamToken: process.env.REACT_APP_SUBSTREAMS_API_TOKEN,
        substreamSpkgUrl:
          "https://raw.githubusercontent.com/verified-network/verified-substreams/new-update/marginpool_factory/marginpool-factory-v0.1.0.spkg",
        substreamModule: "map_trades_per_pool",
        substreamProtoUrl:
          "https://raw.githubusercontent.com/verified-network/verified-substreams/new-update/marginpool_factory/contract.proto",
        substreamMessage: "contract.v1.MarginPoolTradeReportsList",
        healthAndMetricsPath: "backend/logs/marginHealthAndMetric11155111.json",
        cursorPath: "../logs/marginCursor11155111.txt",
      },
    },
  },
};
