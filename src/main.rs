mod kdf;
mod types;
mod util;
fn main() {
    use kdf::{derive_epsilon, derive_key};
    use near_primitives::types::AccountId;
    use k256::ecdsa::VerifyingKey;
    use k256::EncodedPoint;

    // Example public key and account ID
    let public_key_str = "secp256k1:37aFybhUHCxRdDkuCcB3yHzxqK7N8EQ745MujyAQohXSsYymVeHzhLxKvZ2qYeRHf3pGFiAsxqFJZjpF9gP2JV5u";
    let account_id: AccountId = "felipe-sandbox.testnet".parse().unwrap();
    let path = ",ethereum,near.org";

    // Step 1: Extract the key part from the string and convert it to a PublicKey type
    // This step is hypothetical and needs to be adjusted based on the actual PublicKey type and conversion method
    let public_key_bytes = hex::decode(&public_key_str[9..]).unwrap(); // Skip the prefix and decode
    let public_key = near_crypto::PublicKey::from(public_key_bytes); // Adjust this line to match the actual conversion method

    // Step 2: Convert the PublicKey to an affine point
    let affine_point = public_key.into_affine_point();

    // Derive epsilon
    let epsilon = derive_epsilon(&account_id, path);

    // Derive the new public key using the original public key and epsilon
    let derived_public_key = derive_key(public_key, epsilon);

    println!("Derived Public Key: {:?}", derived_public_key);
}