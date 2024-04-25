fmt:
	cargo fmt

lint: fmt
	cargo clippy --tests --all-features -- -D warnings

test:
	cd ../emf_contract && make build
	cargo test --jobs 1 -- --nocapture --test-threads 1 $(name)
