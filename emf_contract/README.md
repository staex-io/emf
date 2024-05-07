# EMF Substrate smart contract

This folder contains everything about ink! smart contract.

## Usage

To run all tests:

```shell
make test
```

To run single test:

```shell
make test name=test_issue_certificate_ok
```

To build and deploy smart contract:

```shell
make run_substrate
make deploy
```

### Deploy

```shell
cargo contract instantiate \
		--args 10 --args 2 --args 1 --args 1 --args 360 --args 2 \
    ...
```

Args:

1. Max measurement value. If some of the measurements are higher this number it means we issue bad certificate for the cell company. It means measurements are not ok for the particular cell tower.
2. Max measurement count. How many measurement we need to save before issue the certificate.
3. Minimal time which should be passed between each measurement to save it to storage.
4. Minimal time which should be passed between each spike to save it to storage.
5. Maximum time between each spike to not spawn an event. If difference in time of two nearest spikes are more than this time, we do not spawn an event.
6. Minimal spikes to spawn an event.

## Smart contract methods

### Create entity (`create_entity`)

With this method cell company can create new on-chain entity for itself. This is required action to process with other smart contract features (methods).

### Create sub-entity (`create_sub_entity`)

With this method entities (cell companies) can initialize sub-entities (cell towers) and link to the main entity. Only cell companies (main entity) can use this method.

Cell tower location as a required parameter to create an on-chain entity. We need it to show this cell tower on the map in user interface. So users can find a cell tower near them and check its measurements.

This method rejects creating a sub-entity in case the main entity was not created before.

### Delete sub-entity (`delete_sub_entity`)

With this method entities (cell companies) can delete linked sub-entities. Only cell companies (main entity) can use this method. This method will not delete sub-entity from smart contract storage to not lose measurement data and certificates, but it will produce an event so indexers can know it to not show on the interactive map

### Store measurement (`store_measurement`)

With this method sub-entity can store their measurements and link to itself. This method can be executed only by sub-entity.

Smart contract stores measurements using u128 Rust type.

In case smart contract reaches enough amount of measurements in the storage, it produces an on-chain event that certificate is ready to be issued. Required amount of measurements can be set during smart contract initialization.

### Store measurement spike (`store_measurement_spike`)

With this method measurement software (or sub-entity or cell tower in other words) can store measurement spikes. Spikes will be stored separately from average values.

As we want to store average values in the smart contract, we still don’t want to miss unexpected values. So we want to store such measurement spikes on-chain too. Spike means when value is more than X% of last previous value and value is more than maximum permitted.

Smart contract stores measurement spikes using u128 Rust type.

Every new spike smart contract produces new on-chain event about it.

In case smart contract reaches enough amount of measurement spikes in the storage, it produces an on-chain event that there are too many spikes for the cell tower (sub-entity). Required amount of measurements can be set during smart contract initialization.

### Check sub-entity (`check_sub_entity`)

With this method users can check that measurements are ok for some cell tower (sub-entity). Currently we are thinking that ok means that last month measurements were in the interval of ok numbers (everything can be changed during smart contract initialization). Month means the last 30 records as we want to store avg measurement once per day. One parameter should be passed: sub-entity (cell tower) public key.

### Issue certificate (`issue_certificate`)

This method can be executed only from the main entity (cell company). One parameter should be passed: sub-entity public key. So this method will check measurements from this sub-entity and issue certificates if measurements will be ok. Otherwise, issuing can be rejected. We will restrict the execution of this method not by entity account (cell tower’s company).

Rejected means we still issue certificate but we point in the certificate body that measurement were not ok.
