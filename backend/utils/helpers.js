import { SubstreamsNode } from "./substream";

export const getSubstreamNode = (chainConfig) =>
  new SubstreamsNode({
    endpoint: chainConfig?.substreamEndpoint,
    token: chainConfig?.substreamToken,
    spkgUrl: chainConfig?.substreamSpkgUrl,
    module: chainConfig?.substreamModule,
    startBlock: Number(chainConfig?.substreamStartBlock || 0),
    protoPath: chainConfig?.substreamProtoUrl,
    protoMessage: chainConfig?.substreamMessage,
    cursorFilePath: chainConfig?.cursorPath,
    healthMetricFilePath: chainConfig?.healthAndMetricsPath,
    onMessage: (event) => console.log("EVENT:", event),
    onConnected: (status) => console.log("CONNECTED:", status),
    onError: (err) => console.error("ERROR:", err),
  });

// export const getSecondarySubstreamNode = (chainConfig) =>
//   new SubstreamsNode({
//     endpoint: chainConfig?.substreamEndpoint,
//     token: chainConfig?.substreamToken,
//     spkgUrl: chainConfig?.substreamSpkgUrl,
//     module: chainConfig?.substreamModule,
//     startBlock: Number(chainConfig?.substreamStartBlock || 0),
//     protoPath: chainConfig?.substreamProtoUrl,
//     protoMessage: chainConfig?.substreamMessage,
//     cursorFilePath: chainConfig?.cursorPath,
//     healthMetricFilePath: chainConfig?.healthAndMetricsPath,
//     onMessage: (event) => console.log("EVENT:", event),
//     onConnected: (status) => console.log("CONNECTED:", status),
//     onError: (err) => console.error("ERROR:", err),
//   });

// export const getMarginSubstreamNode = (chainConfig) =>
//   new SubstreamsNode({
//     endpoint: chainConfig?.substreamEndpoint,
//     token: chainConfig?.substreamToken,
//     spkgUrl: chainConfig?.substreamSpkgUrl,
//     module: chainConfig?.substreamModule,
//     startBlock: Number(chainConfig?.substreamStartBlock || 0),
//     protoPath: chainConfig?.substreamProtoUrl,
//     protoMessage: chainConfig?.substreamMessage,
//     cursorFilePath: chainConfig?.cursorPath,
//     healthMetricFilePath: chainConfig?.healthAndMetricsPath,
//     onMessage: (event) => console.log("EVENT:", event),
//     onConnected: (status) => console.log("CONNECTED:", status),
//     onError: (err) => console.error("ERROR:", err),
//   });
