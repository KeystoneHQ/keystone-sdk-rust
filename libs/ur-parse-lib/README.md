# ur-parse-lib

`ur-parse-lib` provides convenience helpers to encode and decode UR payloads
using `keystone-ur` and `ur-registry`.

It is intended for SDK integrations that need a simpler API surface on top of
registry objects and UR transport strings.

## Installation

```toml
[dependencies]
ur-parse-lib = "1.0.0"
```

## What It Provides

- UR encoder helpers
- UR decoder helpers
- Conversion utilities between raw bytes and UR strings

## Example

```rust
// See crate source for encoder/decoder entry points:
// - keystone_ur_encoder
// - keystone_ur_decoder
```

## License

MIT
