# My first contract

This is an example contract being used in the docs. You can read more about it [here](https://docs.dusk.network/developer/smart-contract/guides/my-first-contract/)

This workspace consists of two members:
```
my-first-contract/
├─ contract/     # The smart contract (WASM target)
└─ data-driver/  # JSON <-> RKYV translator (WASM target for frontends/indexers)
```

## Prerequisites

- Rustup
- `make`
- [Wasm-pack](https://github.com/rustwasm/wasm-pack)
- [Wasm-tools](https://github.com/bytecodealliance/wasm-tools)

```bash
cargo install --locked wasm-pack
cargo install --locked --version 1.207.0 wasm-tools
```

## Build everything

```bash
make
```

### Artifacts

- Contract WASM: `./target/wasm32-unknown-unknown/release/my_first_contract.wasm`
- Data‑driver WASM: `./target/wasm32-unknown-unknown/release/my_first_contract_dd.wasm`

## What each member does

### `contract/`

A minimal counter contract with three calls:

- `read_value`: input `()` → output `u32`
- `increment`: input `()` → output `()`
- `init`: input `u32` → output `()`

### `data-driver/`

A WASM library that converts JSON <-> RKYV bytes for the contract’s ABI surface.

- **Functions**
  - `read_value`: input `()`; output `u32`
  - `increment`: input `()`; output `()`
  - `init`: input `u32`; output `()`
- **Events**
  - `INIT`: payload `u32`

The driver is `no_std` and exports an FFI suitable for JS host code (see `features = "js"` build).
