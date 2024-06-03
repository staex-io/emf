# Agent

Software which receives and accumulate new measurements and store on-chain by interacting with smart contract which is deployed to Substrate node.

## Usage

```shell
Command line utility to interact with EMF agent

Usage: 

Commands:
  run     Run agent
  faucet  Faucet some account with test tokens
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

To run agent you can use following command:

```shell
cargo run -- run
```

Agent configuration can be updated by two environment variables:

1. `SMART_CONTRACT_ADDRESS` - to change smart contract address
2. `TIME_TO_ACCUMULATE` - you can use this env to change interval between first and last measurements which are stored to start saving measurement on-chain

Also using command line arguments for `run` command:

```shell
Run agent

Usage: 

Arguments:
  [CONTRACT_ADDRESS]  Specify smart contract address [default: 5GPGUPaCzQKHao1bQ5y9BybDzbpsbjAribjTQ3xSe1dcxJxe]
  [PHRASE]            Specify cell tower private phrase. Address of the default phrase: 5CwQRPkqmUg5arWuJtw2qoGRL4oRDjguzmrcrSSsv35Cuv3s [default: "corn recipe you dish oil glass found wood weekend above thumb siege"]

Options:
  -h, --help  Print help
```

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

## Faucet

If you are working locally you can faucet some tokens to your account by:

```shell
Faucet some account with test tokens

Usage: 

Arguments:
  [ADDRESS]  Specify address to faucet [default: 5FvLyPSLg9caiZPgdVyXB6uPJXxyC1zfSMR3EthQg1bTwVzR]

Options:
  -h, --help  Print help
```

```shell
cargo run -- faucet <address>
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
