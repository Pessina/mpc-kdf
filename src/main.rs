use near_sdk::PublicKey;

mod kdf;
mod types;
mod util;
extern crate bs58;

fn main() {
    use k256::{AffinePoint, EncodedPoint};
    use k256::elliptic_curve::sec1::FromEncodedPoint;
    
    let public_key: PublicKey = "secp256k1:37aFybhUHCxRdDkuCcB3yHzxqK7N8EQ745MujyAQohXSsYymVeHzhLxKvZ2qYeRHf3pGFiAsxqFJZjpF9gP2JV5u".parse().unwrap();
    let account_id: near_primitives::types::AccountId = "felipe-sandbox.testnet".parse().unwrap();
    let path = ",ethereum,near.org";

    let mut pk_bytes= vec![0x04];
    // Extend `pk_bytes` with the decoded bytes
    pk_bytes.extend_from_slice(&public_key.as_bytes()[1..]);

    let point = EncodedPoint::from_bytes(pk_bytes).unwrap();
    let public_key = AffinePoint::from_encoded_point(&point).unwrap();
    let epsilon = &kdf::derive_epsilon(&account_id, path);
    let derived_public_key= &kdf::derive_key(public_key, *epsilon);

    println!("Derived Public Key: {:?}", derived_public_key);
}