.PHONY: all build contract driver driver-js clean tree

all: build

build: contract driver-js

contract:
	@cargo build -p my-first-contract --release --target wasm32-unknown-unknown

driver:
	@cargo build -p my-first-contract-dd --release --target wasm32-unknown-unknown --features ffi

driver-js:
	@cargo build -p my-first-contract-dd --release --target wasm32-unknown-unknown --features js

clean:
	@cargo clean

tree:
	@echo "Artifacts:" && \	ls -1 contract/target/wasm32-unknown-unknown/release/* 2>/dev/null || true && \	ls -1 data-driver/target/wasm32-unknown-unknown/release/* 2>/dev/null || true
