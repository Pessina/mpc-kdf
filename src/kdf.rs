use crate::util::ScalarExt;
use k256::elliptic_curve::CurveArithmetic;
use k256::{Scalar, Secp256k1, AffinePoint, EncodedPoint};
use sha2::{Digest, Sha256};

use k256::elliptic_curve::sec1::FromEncodedPoint;

const EPSILON_DERIVATION_PREFIX: &str = "near-mpc-recovery v0.1.0 epsilon derivation:";

pub fn derive_epsilon(signer_id: &str, path: &str) -> Scalar {
    let signer_id_local: near_primitives::types::AccountId = signer_id.parse().unwrap();
    
    let derivation_path = format!("{EPSILON_DERIVATION_PREFIX}{},{}", signer_id_local, path);
    let mut hasher = Sha256::new();
    hasher.update(derivation_path);
    Scalar::from_bytes(&hasher.finalize())
}

pub fn derive_key(public_key: &str, epsilon: Scalar) -> crate::types::PublicKey {
    let public_key_local: near_sdk::PublicKey = public_key.parse().unwrap();

    let mut pk_bytes= vec![0x04];
    pk_bytes.extend_from_slice(&public_key_local.as_bytes()[1..]);

    let point = EncodedPoint::from_bytes(pk_bytes).unwrap();
    let public_key = AffinePoint::from_encoded_point(&point).unwrap();

    (<Secp256k1 as CurveArithmetic>::ProjectivePoint::GENERATOR * epsilon + public_key).to_affine()
}

pub fn derive_eth_address(derived_public_key: &AffinePoint) -> String {
    use sha3::Keccak256;
    use k256::elliptic_curve::sec1::ToEncodedPoint;

    let derived_public_key_bytes = derived_public_key.to_encoded_point(false).as_bytes().to_vec();
    let hash = Keccak256::digest(&derived_public_key_bytes[1..]); 
    let eth_address = &hash[12..]; 

    hex::encode(eth_address)
}
