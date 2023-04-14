#include "stdint.h"

struct ExternError {
    int32_t code;
    char *message; // note: nullable
};

void keystone_sdk_destroy_string(const char* cstring);

// Wallet
const char* parse_crypto_multi_accounts(struct ExternError*, const char* ur_type, const char* cbor_hex);

// BTC
const char* generate_crypto_psbt(struct ExternError*, const char* psbt_hex);
const char* parse_crypto_psbt(struct ExternError*, const char* ur_type, const char* cbor_hex);

// ETH
const char* generate_eth_sign_request(struct ExternError*, const char* request_id, const char* sign_data, const int data_type, const int chain_id, const char* path, const char* xfp, const char* address, const char* origin);
const char* parse_eth_signature(struct ExternError*, const char* ur_type, const char* cbor_hex);

// SOL
const char* generate_sol_sign_request(struct ExternError*, const char* request_id, const char* sign_data, const char* path, const char* xfp, const char* address, const char* origin, const int sign_type);
const char* parse_sol_signature(struct ExternError*, const char* ur_type, const char* cbor_hex);

// Cosmos
const char* generate_cosmos_sign_request(struct ExternError*, const char* request_id, const char* sign_data, const int data_type, const char* accounts, const char* origin);
const char* parse_cosmos_signature(struct ExternError*, const char* ur_type, const char* cbor_hex);

// Tron
const char* generate_tron_sign_request(struct ExternError*, const char* request_id, const char* sign_data, const char* path, const char* xfp, const char* token_info, const char* address, const char* origin);
const char* parse_tron_signature(struct ExternError*, const char* ur_type, const char* cbor_hex);
