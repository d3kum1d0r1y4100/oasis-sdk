//! EVM module types.

/// Transaction body for creating an EVM contract.
#[derive(Clone, Debug, cbor::Encode, cbor::Decode)]
pub struct CreateTx {
    pub value: U256,
    pub init_code: Vec<u8>,
    pub gas_price: U256,
    pub gas_limit: u64,
}

/// Transaction body for calling an EVM contract.
#[derive(Clone, Debug, cbor::Encode, cbor::Decode)]
pub struct CallTx {
    pub address: H160,
    pub value: U256,
    pub data: Vec<u8>,
    pub gas_price: U256,
    pub gas_limit: u64,
}

/// Transaction body for peeking into EVM storage.
#[derive(Clone, Debug, cbor::Encode, cbor::Decode)]
pub struct PeekStorageQuery {
    pub address: H160,
    pub index: H256,
}

/// Transaction body for peeking into EVM code storage.
#[derive(Clone, Debug, cbor::Encode, cbor::Decode)]
pub struct PeekCodeQuery {
    pub address: H160,
}

// The rest of the file contains wrappers for primitive_types::{H160, H256, U256},
// so that we can implement cbor::{Encode, Decode} for them, ugh.
// Remove this once oasis-cbor#8 is implemented.
//
// Thanks to Nick for providing the fancy macros below :)

// This `mod` exists solely to place an `#[allow(warnings)]` around the generated code.
#[allow(warnings)]
mod eth {
    use super::*;

    macro_rules! construct_fixed_hash {
        ($name:ident($num_bytes:literal)) => {
            fixed_hash::construct_fixed_hash! {
                pub struct $name($num_bytes);
            }

            impl cbor::Encode for $name {
                fn into_cbor_value(self) -> cbor::Value {
                    cbor::Value::ByteString(self.as_bytes().to_vec())
                }
            }

            impl cbor::Decode for $name {
                fn try_from_cbor_value(value: cbor::Value) -> Result<Self, cbor::DecodeError> {
                    match value {
                        cbor::Value::ByteString(v) => Ok(Self::from_slice(&v)),
                        _ => Err(cbor::DecodeError::UnexpectedType),
                    }
                }
            }
        };
    }

    macro_rules! construct_uint {
        ($name:ident($num_words:tt)) => {
            uint::construct_uint! {
                pub struct $name($num_words);
            }

            impl cbor::Encode for $name {
                fn into_cbor_value(self) -> cbor::Value {
                    let mut out = [0u8; 32];
                    self.to_big_endian(&mut out);
                    cbor::Value::ByteString(out.to_vec())
                }
            }

            impl cbor::Decode for $name {
                fn try_from_cbor_value(value: cbor::Value) -> Result<Self, cbor::DecodeError> {
                    match value {
                        cbor::Value::ByteString(v) => Ok(Self::from_big_endian(&v)),
                        _ => Err(cbor::DecodeError::UnexpectedType),
                    }
                }
            }
        };
    }

    construct_fixed_hash!(H160(20));
    construct_fixed_hash!(H256(32));
    construct_uint!(U256(4));

    macro_rules! impl_upstream_conversions {
        ($($ty:ident),* $(,)?) => {
            $(
                impl From<$ty> for primitive_types::$ty {
                    fn from(t: $ty) -> Self {
                        Self(t.0)
                    }
                }

                impl From<primitive_types::$ty> for $ty {
                    fn from(t: primitive_types::$ty) -> Self {
                        Self(t.0)
                    }
                }
            )*
        }
    }

    impl_upstream_conversions!(H160, H256, U256);
}
pub use eth::{H160, H256, U256};