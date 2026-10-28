# keystone-sdk-rust

## Signing API changes

The legacy `keystone-sign-request` / `keystone-sign-result` APIs have been removed. Use TRON Native for TRON and PSBT Extend for LTC, BCH, DASH and DOGE. These protocols require different transaction inputs and response handling; they are not drop-in replacements. Integrations that still require the removed APIs must pin an earlier SDK version.

The Keystone Rust SDK is built to simplify the way how software wallets
communicate with the Keystone hardware wallet via
[UR](https://github.com/BlockchainCommons/Research/blob/master/papers/bcr-2020-005-ur.md).

The Rust SDK provides
- The implementation of UR for different blockchains
- FFI for iOS SDK and Android SDK

## License

This project is licensed under the MIT License. See the [LICENSE.md](./LICENSE.md) file for details.

## TRON native signing (FFI)

The `ur-registry-ffi` module `tron::native` exposes two new operations to mobile SDKs:

| Rust / C name | Android JNI method |
| --- | --- |
| `generate_tron_native_sign_request` | `generateTronNativeSignRequest` |
| `parse_tron_native_signature` | `parseTronNativeSignature` |

The C ABI is enabled with default features disabled. Its declarations are in `include/URRegistryFFI/lib_ur_registry_ffi.h`:

```c
const char* generate_tron_native_sign_request(
    struct ExternError* error,
    const char* request_id, const char* sign_data,
    const char* path, const char* xfp, int32_t data_type,
    const char* address, const char* origin);

const char* parse_tron_native_signature(
    struct ExternError* error, const char* ur_type, const char* cbor_hex);
```

Example C calls (the input strings below are supplied by the wallet):

```c
struct ExternError error = {0};
const char* request_json = generate_tron_native_sign_request(
    &error, request_id, transaction_raw_data_hex,
    "m/44'/195'/0'/0/0", device_xfp, 1, "", "My Wallet");
// Check error.code, then parse request_json and check its "error" field.
// Success: {"type":"tron-sign-request", "cbor":"<hex>"}.
// Encode the CBOR bytes with this UR type for QR display.
keystone_sdk_destroy_string(request_json);

struct ExternError response_error = {0};
const char* response_json = parse_tron_native_signature(
    &response_error, "tron-signature", scanned_cbor_hex);
// Check response_error.code and the JSON "error" field before using the result.
// Success: {"request_id":"<uuid>", "signature":"<65-byte hex>"}.
// Match request_id to the pending request before accepting the signature.
keystone_sdk_destroy_string(response_json);
```

Free each returned JSON string with `keystone_sdk_destroy_string`. The JSON keys use snake_case; mobile wrappers expose camelCase properties. Invalid input returns an `{"error":"..."}` JSON object; callers must also handle any FFI-level error before reading the returned pointer. Android's JNI methods accept the same payload arguments without the C error pointer and return JSON strings.

### Request parameters

| Parameter | Meaning |
| --- | --- |
| `requestId` | A new UUID for each signing request. Match the response UUID before using its signature. |
| `signData` | Hex with or without a `0x` / `0X` prefix; the SDK strips the prefix before validation and encoding. For `dataType = 1`, use the serialized TRON transaction **raw_data** (`raw_data_hex`), not JSON, a full signed transaction, or a transaction hash. For `dataType = 2`, use the original personal-message bytes encoded as hex. |
| `path` | Signing derivation path, for example `m/44'/195'/0'/0/0`. |
| `xfp` | The connected device's master fingerprint: exactly 8 hex characters. |
| `dataType` | `1` = transaction (default in mobile SDKs); `2` = personal message. |
| `address` | Optional TRON Base58Check address; an empty string omits it. |
| `origin` | Optional wallet/application name; an empty string omits it. |

`signData` must contain 1–32768 bytes. `path` is limited to 256 bytes and `origin` to 128 UTF-8 bytes.

The request UR type is `tron-sign-request`; the response UR type is `tron-signature`.
The response contains `requestId` and a 65-byte signature encoded as hex. It does not contain a signed transaction or transaction ID. For transactions, retain the original transaction, verify the response request ID and signature against the intended signer, then attach the signature to that transaction before broadcasting. Parsing does not perform that cryptographic verification or broadcast.

Use a firmware version that supports the native TRON UR types.
