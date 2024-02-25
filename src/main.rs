mod kdf;
mod types;
mod util;
extern crate bs58;

fn main() {
    let public_key: &str = "secp256k1:37aFybhUHCxRdDkuCcB3yHzxqK7N8EQ745MujyAQohXSsYymVeHzhLxKvZ2qYeRHf3pGFiAsxqFJZjpF9gP2JV5u";
    let account_id: &str = "felipe-sandbox.testnet";
    let path = ",bitcoin,felipe.org";

    let epsilon = kdf::derive_epsilon(&account_id, path);

    println!("\nEpsilon: {:?}\n", epsilon);

    let derived_public_key=  kdf::derive_key(public_key, epsilon);

    println!("\nDerived Public Key: {:?}\n", derived_public_key);

    let eth_address = kdf::derive_eth_address(&derived_public_key);

    println!("\nETH Address: 0x{}\n", eth_address);

}