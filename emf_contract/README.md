# EMF Substrate smart contract

## Usage

```shell
# To run tests use:
make test name=test_issue_certificate_ok

# To build and deploy smart contract:
make run_substrate
make deploy
```

### Deploy arguments

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
