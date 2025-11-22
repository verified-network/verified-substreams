# secondarypool_factory Substreams modules

This package was initialized via `substreams init`, using the `evm-events-calls` template.

## Usage

```bash
substreams build
substreams auth
substreams gui       			  # Get streaming!
```

Optionally, you can publish your Substreams to the [Substreams Registry](https://substreams.dev).

```bash
substreams registry login         # Login to substreams.dev
substreams registry publish       # Publish your Substreams to substreams.dev
```

## Modules

All of these modules produce data filtered by these contracts:
- _secondarypool_factory_ at **0x4519148cc4030c2e3573f1f886ed4071fa35d62b**
- secondary_pool contracts created from _secondarypool_factory_
### `map_events`

This module gets you only events that matched.


