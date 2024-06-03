# EMF

The goal of this project is to digitize and automate certification of cell towers’ compliance with regulations related to electromagnetic field (EMF) radiation levels.

Proliferation of mobile 4G and 5G networks increased people’s concerns about the negative influence of these networks on people’s health. As a result European policy makers created regulations that limit the maximum level of EMF radiation produced by wireless antennas. Since a cell tower usually hosts multiple antennas, the EMF compliance certification lies on the shoulders of cell tower companies. The current certification process is mostly manual and involves subcontractors going to each tower and making measurements using specialized equipment. This is inefficient in terms of time and money especially when you need to repeat the certification periodically, i.e. when the new antennas are mounted on the tower.

We want to automate this process using industry-grade EMF sensors that save their measurements in the blockchain. The sensors are perpetually mounted on the towers and are connected to the single-board computer that periodically (e.g. once a day/month, after each new antenna is installed etc.) sends the data to the blockchain.

Within the smart contracts we would like to store each cell company and their cell towers on-chain and link with them EMF measurements which will be stored on-chain too. From time to time we would like to issue on-chain certificates which will be based on EMF measurements. These certificates can be included into the official documents that the cell tower company submits to the regulatory authorities.

We partner with a cell tower company that is particularly interested in reducing the operational costs associated with EMF certification. The automation of this process via blockchain would substantially reduce these costs.

---

![](./docs/images/emf_general.png)

---

![](./docs/images/emf_precise.png)

---

**[Flow](./docs/render/Flow.svg)**

By flow below you can find how this software works step-by-step.

EMF - electromagnetic field.

Useful links about regulation:
- https://single-market-economy.ec.europa.eu/sectors/electrical-and-electronic-engineering-industries-eei/radio-equipment-directive-red_en
- https://www.sgs.com/en/news/2023/01/cc-2022-q4-finding-the-best-way-to-demonstrate-compliance-with-cybersecurity-regulations
- https://www.tuvsud.com/en/resource-centre/stories/new-eu-security-legislation-under-radio-equipment-directive

Our initial idea is to develop special measurement software in Rust which can be run on IoT devices. This software will accumulate data from the measurement module and store it on-chain using smart contracts. Using stored measurement we can issue on-chain certificates for cell towers companies through the smart contract.

This software contains by three parts:

1. [Smart contract](./emf_contract/) using [ink!](https://use.ink/) language
2. [Agent](./agent/) - software which receives new measurements and store them on-chain
3. [Indexer](./agent/) - part of the agent which listens for on-chain events and store them in the database
3. [User interface](./emf-ui/) - EMF user interface to see cell towers on the map and crete new one

## How to run

```shell
cd emf_contract
# Run substrate node.
make run_substrate

# In separate terminal session.
cd emf_contract
# Deploy smart contract.
make deploy

# In separate terminal session.
cd agent
# Run agent.
# Use smart contract address from deploy stage.
rm -rf emf.indexer.sqlite ; RUST_LOG=trace,agent::indexer=error TIME_TO_ACCUMULATE=0 \
  cargo run -- run 5GPGUPaCzQKHao1bQ5y9BybDzbpsbjAribjTQ3xSe1dcxJxe

# In separate terminal session.
cd emf-ui
# Prepare .env file.
echo -n "VITE_CONTRACT_ADDRESS=5GPGUPaCzQKHao1bQ5y9BybDzbpsbjAribjTQ3xSe1dcxJxe" > .env
# Run user interface.
make run-docker
# Open browser page at: http://localhost:5173/

# In case you need tokens for your wallet (entity or sub-entity).
cd agent
# Do faucet.
cargo run -- faucet 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY

# Initialize entity and sub-entity on UI.

# Start producing measurements.
cat <(echo -n '{"value":5}') | nc 127.0.0.1 3322
```
## Drift

In case you need fully automatically started environment you can use drift mode.

You can start drift mode for demo purposes. In this mode Substrate node starts automatically, smart contract is deployed and agent with indexer is up and running. Also in this mode there are 3 already prepared towers with data and certificates.

This mode has a timeout for an hour after tests passing. Also it starts to generate random measurements for random entities and sub-entities.

```shell
cd agent && make drift
```

## Staex Public Network (SPN)

If you want to provide more secure communication with your IoT devices you can use [Staex](https://staex.io) software and our Staex Public Network (https://staex.io/blog/public-network-for-iot-devices).

[Here](https://docs.staex.io/how-to-guides/quick-start/) you can find precise documentation how to start own node and use it.
