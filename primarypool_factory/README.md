# primarypool_factory Substreams modules

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
- _primarypool_factory_ at **0xda13bc71fee08ffd523f10458b0e2c2d8427bbd5**
- primary_pool contracts created from _primarypool_factory_
### `map_events`

This module gets you only events that matched.


