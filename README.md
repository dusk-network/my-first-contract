# My first contract

This repository contains a minimal example smart contract for DuskDS:  
a simple counter that shows how contracts are structured, built and used on Dusk’s settlement layer.

The full hands‑on tutorial lives in this README. For conceptual background on DuskDS smart contracts, see the "Develop a contract" page in the Dusk docs.

## Overview

The workspace has two members:

```text
my-first-contract/
├─ contract/     # The smart contract (WASM target for DuskDS)
└─ data-driver/  # JSON <-> RKYV translator (WASM target for frontends/indexers)
```

- `contract/` is the actual DuskDS contract, compiled to WASM and deployed on-chain.
- `data-driver/` is an auxiliary WASM library that converts between JSON and the binary
  format (RKYV) used by the contract’s ABI surface, so host applications don’t need to
  know the low-level encoding.

The contract exposes three entrypoints:

- `read_value`: input `()` → output `u32` (reads the current counter)
- `increment`: input `()` → output `()` (increments the counter)
- `init`: input `u32` → output `()` (initializes the counter at deployment time)

The contract also emits an `INIT` event with the initial value.

## Prerequisites

You’ll need:

- [Rustup](https://rustup.rs/)
- `make`
- [Wasm-pack](https://github.com/rustwasm/wasm-pack)
- [Wasm-tools](https://github.com/bytecodealliance/wasm-tools)

```bash
cargo install --locked wasm-pack
cargo install --locked --version 1.207.0 wasm-tools
```

When you run `cargo` inside this repo, the `rust-toolchain.toml` will automatically
select the correct Rust version for you.

## 1. Clone the repo

```bash
git clone https://github.com/dusk-network/my-first-contract.git
cd my-first-contract
```

## 2. Build the contract and data driver

The root `Makefile` provides convenient targets:

```bash
make
```

This runs:

- `cargo build -p my-first-contract --release --target wasm32-unknown-unknown`
- `cargo build -p my-first-contract-dd --release --target wasm32-unknown-unknown --features js`

You should now have release WASM artifacts for:

- `contract/` (the on-chain contract)
- `data-driver/` (the JSON↔RKYV helper, with JS‑friendly exports)

## 3. What the contract does

The `contract/src/lib.rs` file contains a simple counter contract:

- It is a `no_std` Rust crate compiled to WASM.
- It uses `dusk-core`’s ABI helpers to talk to the DuskDS VM.
- It stores a single `u32` counter in persistent contract state.

Entry points (exported via `#[no_mangle]`):

```rust
#[no_mangle]
unsafe fn read_value(arg_len: u32) -> u32 {
    wrap_call(arg_len, |_: ()| STATE.read_value())
}

#[no_mangle]
unsafe fn increment(arg_len: u32) -> u32 {
    wrap_call(arg_len, |_: ()| STATE.increment())
}

#[no_mangle]
unsafe fn init(arg_len: u32) -> u32 {
    wrap_call(arg_len, |arg: u32| STATE.init(arg))
}
```

Each function:

1. Uses `wrap_call` from `dusk_core::abi` to decode the input,
2. Calls into the Rust implementation (`STATE.read_value()`, `STATE.increment()`, `STATE.init(arg)`),
3. Returns encoded output back to the host.

The entire linear memory of the contract is persisted between calls, so the counter value
is kept on-chain between invocations.

## 4. What the data driver does

The `data-driver` crate is a small WASM library used by off-chain applications
(frontends, indexers, etc.) to work with the contract’s ABI without knowing the
low-level binary encoding.

From `data-driver/src/lib.rs`:

- **Functions**
  - `read_value`: input `()`; output `u32`
  - `increment`: input `()`; output `()`
  - `init`: input `u32`; output `()`
- **Events**
  - `INIT`: payload `u32`

It implements `ConvertibleContract` from `dusk-data-driver` to:

- encode JSON payloads into RKYV bytes before sending them to the contract;
- decode RKYV bytes from the contract back into JSON.

To build the data driver with JS‑friendly exports:

```bash
cd data-driver
make wasm-js
```

This runs:

```bash
cargo build --release --target wasm32-unknown-unknown --features js
```

The resulting WASM can be loaded from a JS/TS host and used to prepare inputs/outputs for
the on-chain contract.

## 5. Deploying the contract

> **Note**  
> These steps assume you have a running DuskDS node and access to the appropriate
> deployment tooling (for example, via the Rusk CLI/wallet). Consult the main Dusk docs
> for up‑to‑date deployment commands.

At a high level, deployment looks like this:

1. **Build the contract** (see step 2 above). You’ll get a `*.wasm` artifact.
2. **Upload and deploy** using your preferred tool:
   - specify the contract WASM,
   - choose the initial value for the counter as the `init` argument,
   - pay the deployment fee.
3. Once the transaction is finalized, the contract address is available and you can
   interact with it.

For the most current deployment commands and examples, see the Dusk docs section
“Develop a contract on DuskDS”.

## 6. Interacting with the contract

Once deployed, you can:

- Call `read_value` to fetch the current counter.
- Call `increment` to increase the counter by 1.
- Subscribe to events and listen for `INIT` when the contract is first deployed.

From an off‑chain application, a typical flow is:

1. Use the **data driver** to encode the call arguments (JSON → RKYV).
2. Wrap the resulting bytes in a transaction payload.
3. Submit the transaction to a DuskDS node using the HTTP API.
4. Decode outputs and events using the data driver.

For details on transaction submission and lifecycle, see the **Integrate with DuskDS**
section of the Dusk docs.
