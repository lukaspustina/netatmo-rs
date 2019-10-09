check:
	cargo check --all --tests --examples --benches

build:
	cargo build --all --tests --examples --benches

test:
	cargo test --all

clean-package:
	cargo clean -p $$(cargo read-manifest | jq -r .name)

clippy:
	cargo clippy --all --all-targets --all-features -- -D warnings $$(source ".clippy.args")

fmt:
	cargo +nightly fmt

audit:
	cargo audit --deny-warnings

.PHONY:

