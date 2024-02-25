mod kdf;
mod types;
mod util;
extern crate bs58;

fn main() {
    use k256::{AffinePoint, EncodedPoint};
    use k256::elliptic_curve::sec1::FromEncodedPoint;
    
    let public_key_str = "secp256k1:37aFybhUHCxRdDkuCcB3yHzxqK7N8EQ745MujyAQohXSsYymVeHzhLxKvZ2qYeRHf3pGFiAsxqFJZjpF9gP2JV5u";
    let account_id: near_primitives::types::AccountId = "felipe-sandbox.testnet".parse().unwrap();
    let path = ",ethereum,near.org";

    let mut pk_bytes = vec![0x04];
    let hex_str = public_key_str.split(':').nth(1).expect("Invalid format for public_key_str");

    // Decode the base58 string
    let decoded_bytes = bs58::decode(hex_str).into_vec().expect("Failed to decode base58");

    // Extend `pk_bytes` with the decoded bytes
    pk_bytes.extend_from_slice(&decoded_bytes);

    let point = EncodedPoint::from_bytes(pk_bytes).unwrap();
    let public_key = AffinePoint::from_encoded_point(&point).unwrap();
    let epsilon = &kdf::derive_epsilon(&account_id, path);
    let derived_public_key= &kdf::derive_key(public_key, *epsilon);

    println!("Derived Public Key: {:?}", derived_public_key);
}