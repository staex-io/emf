# EMF UI

## Usage

Prepare `.env` file in the root user interface folder:

```
VITE_CONTRACT_ADDRESS=5GPGUPaCzQKHao1bQ5y9BybDzbpsbjAribjTQ3xSe1dcxJxe
```

In the `.env` file you need to prepare deployed EMF smart contract address.

Then execute following command:

```shell
# Linux
make run
# macOS
make run-docker
```

If you don't want to use Docker to start user interface you can use [bun](https://bun.sh/) directly.

```shell
bun dev
```

### Linter and formatter

```
make lint
```

### Other

```shell
# To add new dependency.
make new_dep name=<name>

# Install dependencies.
make install

# Build.
make build
```

## Structure

`src/public` - static files.
`src/assets` - assets files.
`src/router` - Vue router definitions.
`src/views/EntitiesView` - in this view we create entities and sub-entities, issue certificates.
`src/views/MapView` - in this view we see map and check issued certificates.
`src/signer-extension.js` - in this file located everything about working with Polkadot.js signer extension.
`src/smart-contract.js` - in this file located everything about working with deployed Substrate smart contract 

For formatting and linting we use [prettier](./.prettierrc.json) and [eslint](./eslint.config.js).
