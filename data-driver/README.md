# Data-driver

Data driver for my-first-contract. It converts between JSON and RKYV bytes for function inputs/outputs and events.

## Functions

- `read_value`: input `()`; output `u32`
- `increment`: input `()`; output `()`
- `init`: input `u32`; output `()`

## Events

- `INIT`: payload `u32`

## Build

```bash
# Build for wasm with JS-friendly exports
make wasm-js
```
