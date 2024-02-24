use k256::elliptic_curve::scalar::FromUintUnchecked;
use k256::{ Scalar, U256};

pub trait ScalarExt {
  fn from_bytes(bytes: &[u8]) -> Self;
}

impl ScalarExt for Scalar {
  fn from_bytes(bytes: &[u8]) -> Self {
      Scalar::from_uint_unchecked(U256::from_le_slice(bytes))
  }
}