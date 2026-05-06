# ur-registry-ffi

`ur-registry-ffi` provides FFI bindings for `ur-registry`, enabling access to
Keystone UR payload functionality from non-Rust environments.

This crate is used by mobile and cross-language integrations that consume
native libraries (for example iOS and Android SDK wrappers).

## Installation

```toml
[dependencies]
ur-registry-ffi = "0.0.1"
```

## What It Provides

- FFI exports around UR registry operations
- Language bridge support for mobile SDK integration
- Structured encode/decode helpers for chain-specific payloads

## Build Notes

- This crate builds as `cdylib` and `staticlib`
- Default feature includes `jni`

## Related Crates

- [`ur-registry`](../ur-registry/README.md)
- [`ur-parse-lib`](../ur-parse-lib/README.md)

## License

MIT
