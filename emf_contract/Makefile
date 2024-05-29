fmt:
	cargo +nightly fmt

lint: fmt
	cargo clippy --tests --all-features -- -D warnings

test:
	cargo test --all-features --jobs 1 -- --nocapture --test-threads 1 $(name)

build:
	cargo contract build

deploy: build
	cargo contract upload --suri //Alice -x
	cargo contract instantiate --suri //Alice \
		--args 10 --args 2 --args 1 --args 1 --args 360 --args 2 \
		-x --skip-confirm
	subxt metadata -f bytes > ./emf.metadata.scale
	subxt codegen --file ./emf.metadata.scale \
		--no-docs | \
		rustfmt --edition=2021 --emit=stdout > ../agent/src/emf_contract.rs
	cp target/ink/emf_contract.json ../agent/assets/emf_contract.metadata.json
	cp target/ink/emf_contract.json ../emf-ui/src/assets/emf_contract.metadata.json
	mv emf.metadata.scale ../agent/assets/emf_contract.metadata.scale
	cp ../agent/src/emf_contract.rs ../agent/tests/emf_contract.rs

run_substrate:
	rm -rf data-substrate ; substrate-contracts-node \
		--no-telemetry --dev -d data-substrate -l info \
		--unsafe-rpc-external --rpc-external --rpc-methods unsafe
