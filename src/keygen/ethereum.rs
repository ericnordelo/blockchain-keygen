use secp256k1::{PublicKey, Secp256k1, SecretKey};
use std::str::FromStr;

use crate::utils::{get_random_u256, prefixed_hex};
use ethers::utils::keccak256;
use hex;

pub fn get_random_pvk() -> String {
    let value = get_random_u256();
    value.to_string()
}

pub fn get_random_keys_and_address() -> (String, String, String) {
    let pvk = get_random_pvk();
    let secret = SecretKey::from_str(&pvk).unwrap();

    let secp = Secp256k1::new();
    let public = PublicKey::from_secret_key(&secp, &secret);
    let pbk = &hex::encode(public.serialize_uncompressed())[2..];

    // Address is the last 20 bytes of the keccak256 hash of the public key (hex encoded)
    let hex_encoded_bytes = hex::decode(pbk).unwrap();
    let address = hex::encode(keccak256(hex_encoded_bytes));

    (
        prefixed_hex(&pvk),
        prefixed_hex(pbk),
        prefixed_hex(&address[24..]),
    )
}
