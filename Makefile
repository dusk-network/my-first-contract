build: ## Build contract
	@RUSTFLAGS="-C link-args=-zstack-size=65536" \
	cargo build \
	  --release \
	  --manifest-path=Cargo.toml \
	  --color=always \
	  -Z build-std=core,alloc \
	  --target wasm32-unknown-unknown
	@mkdir -p target/stripped
	@find target/wasm32-unknown-unknown/release -maxdepth 1 -name "*.wasm" \
	    | xargs -I % basename % \
	    | xargs -I % wasm-tools strip -a \
	 	          target/wasm32-unknown-unknown/release/% \
	 	          -o target/stripped/%

MAX_COUNTER_CONTRACT_SIZE = 8192

.PHONY: contract test
