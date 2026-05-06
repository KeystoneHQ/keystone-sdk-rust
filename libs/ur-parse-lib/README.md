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

If you manually depend on `ur-registry` in the same project with
`default-features = false`, ensure the `core` feature is enabled to keep error
types compatible:

```toml
ur-registry = { version = "1.0.0", default-features = false, features = ["core"] }
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
