# Verified Substreams

## Introduction to Substreams

Substreams is a robust blockchain indexing technology developed for The Graph Network.

Substreams empowers developers to create Rust modules, facilitating the composition of data streams in collaboration with the community. It offers exceptionally high-performance indexing through parallelization, all while embracing a streaming-first approach. Substreams inherits the advantages of StreamingFast Firehose, including cost-effective caching and archiving of blockchain data, high throughput processing, and robust handling of cursor-based reorganizations.

## Documentation

For comprehensive documentation on installing, running, and working with Substreams, please visit: [Substreams Documentation](https://substreams.streamingfast.io).

## Streaming Orders and Trade Data from the Verified Network

This repository contains modules that stream subscription/pricing data from primary issue pools and trade/pricing data from secondary issue pools. There are two modules available: PrimaryPool and SecondaryPool. Substreams stores the data in key/value stores, making it accessible via gRPC. 
More details on modules can be found [Primary](./modules/PrimaryPool/) and [Secondary](./modules/SecondaryPool/)
More details on client interfaces can be found [here](https://github.com/streamingfast/substreams-sink-kv/tree/develop/examples/generic-service) along with an illustrative example.

Verified Substreams support Ethereum endpoints for Mainnet, Polygon, BNB (for production), and Goerli (for testing).

*Ensure you are in the correct directory before running these commands.*

To generate Rust code for connecting to protobuf, navigate to the module folder and execute the following command:
```substreams protogen substreams.yaml --exclude-paths="sf/substreams,google"```

To build the Rust code, run the following command within the module folder:
```cargo build --target wasm32-unknown-unknown --release```

To execute the modules, execute the following command in the module folder:
```substreams run -e <Protocol-Proto model> substreams.yaml <FUNCTION_NAME> --start-block 9561663 --stop-block +20```

##### Replace <Protocol-Proto model> with the desired protocol model. Supported protocol-Proto models are listed below.
1. Ethereum Mainnet: ```mainnet.eth.streamingfast.io:443```
2. Ethereum Görli: ```goerli.eth.streamingfast.io:443```
3. Polygon Mainnet: ```polygon.streamingfast.io:443```
4. BNB: ```bnb.streamingfast.io:443```

To run the client interface on your local system, please refer to [substreams-sink-kv](https://github.com/streamingfast/substreams-sink-kv/tree/develop/examples/generic-service) with a provided example.

## License

[BUSL-1.1](https://github.com/verified-network/verified-substreams/blob/master/LICENSE).







