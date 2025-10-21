all:
	@cargo build --release

wasm:
	@cargo build --release --target wasm32-unknown-unknown --features ffi

wasm-js:
	@cargo build --release --target wasm32-unknown-unknown --features js

clippy:
	@cargo clippy -- -D warnings
	@cargo clippy --target wasm32-unknown-unknown --features ffi -- -D warnings
	@cargo clippy --target wasm32-unknown-unknown --features js -- -D warnings

.PHONY: all wasm wasm-js clippy
