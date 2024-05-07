# Agent

Software which receives and accumulate new measurements and store on-chain by interacting with smart contract which is deployed to Substrate node.

## Usage

To run agent you can use following command:

```shell
cargo run
```

Agent configuration can be updated by two environment variables:

1. `SMART_CONTRACT_ADDRESS` - to change smart contract address
2. `TIME_TO_ACCUMULATE` - you can use this env to change interval between first and last measurements which are stored to start saving measurement on-chain

Agent stores measurements in `measurements.json` file:

```json
{
  "first_measurement": {
    "secs_since_epoch": 0,
    "nanos_since_epoch": 0
  },
  "last_measurement": {
    "secs_since_epoch": 1715007833,
    "nanos_since_epoch": 135056000
  },
  "measurements": [
    6
  ]
}
```

## Indexer

Part of the agent which listens for on-chain events and store them in the database.

As a database we are using sqlite right now with the following [structure](./migrations/20240207111258_init.sql).

You can see indexer [OpenAPI](../openapi/indexer.openapi.yaml).

## Tests

Agent and indexer have end-to-end tests. You can use following command to test agent and indexer simultaneously:

```shell
make test
```

In tests we automatically start substrate contract node, build and deploy smart contract, run agent and indexer.
