# ur-registry

`ur-registry` provides Keystone UR registry data types and CBOR encode/decode
logic used to build multi-chain signing payloads.

This crate is a core building block in `keystone-sdk-rust` and is designed for
low-level UR payload construction and parsing.

## Installation

```toml
[dependencies]
ur-registry = "0.1.1"
```

## Features

- Multi-chain UR registry payload support
- CBOR serialization and deserialization for registry types
- Integration with `keystone-ur` for UR formatting
- `no_std`-friendly design with optional `std` feature

## Example

```rust
// See crate tests and module docs for chain-specific usage examples.
// The crate exposes registry models and conversion helpers for UR payloads.
```

## License

MIT
