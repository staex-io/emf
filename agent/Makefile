fmt:
	cargo fmt

lint: fmt
	cargo clippy --tests --all-features -- -D warnings

test:
	cargo build
	#cd ../emf_contract && make build
	rm -rf measurements.json
	cargo test --jobs 1 -- --nocapture --test-threads 1 $(name)

sql_format:
	docker run --rm \
		-v "${PWD}/migrations:/migrations" \
		-v "${PWD}/scripts/sql_format.sh:/sql_format.sh" \
		--entrypoint="" \
		node:20-alpine ./sql_format.sh
