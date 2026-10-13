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

## PSBT Extend

The iOS C exports are declared in `include/URRegistryFFI/lib_ur_registry_ffi.h`:

- `generate_crypto_psbt_extend(err, psbt_hex, coin_id)` returns
  `{"type":"crypto-psbt-extend","cbor":"..."}`.
- `parse_crypto_psbt_extend(err, ur_type, cbor_hex)` returns
  `{"psbt":"...","coin_id":2}`. An omitted coin field returns `coin_id: null`.

Android wrappers in `com.keystone.sdk.KeystoneNativeSDK` can declare:

```java
public static native String generateCryptoPSBTExtend(String psbtHex, int coinId);
public static native String parseCryptoPSBTExtend(String urType, String cborHex);
```

Use coin ID `0` for Bitcoin (the CBOR coin field is omitted), `2` for Litecoin,
`3` for Dogecoin, `5` for Dash, or `145` for Bitcoin Cash. Other IDs are rejected.
Hex inputs may include a `0x` prefix. Errors are returned as JSON with an `error`
field, following the existing PSBT interface. These helpers wrap PSBT bytes;
they do not validate or sign the underlying transaction.

## Related Crates

- [`ur-registry`](../ur-registry/README.md)
- [`ur-parse-lib`](../ur-parse-lib/README.md)

## License

MIT
