use ethabi::ethereum_types::Address;

use substreams::scalar::BigInt;
use substreams_ethereum::pb::eth as pbeth;
use tiny_keccak::{Hasher, Keccak};

use crate::common::HasAddresser;

/// Retrieves all storage changes in a block for all contracts under HasAddresser.
pub fn get_storage_changes_for_addresses(
    store: &impl HasAddresser,
    block: &pbeth::v2::Block,
) -> Vec<pbeth::v2::StorageChange> {
    let mut changes: Vec<pbeth::v2::StorageChange> = vec![];
    for call in block.calls() {
        for change in &call.call.storage_changes {
            if !store.has_address(Address::from_slice(change.address.as_slice())) {
                continue;
            }
            changes.push(change.to_owned());
        }
    }
    changes
}

/// Utility class to play around with Uint256 storage values.
pub struct Uint256 {
    pub slot: BigInt,
}

impl Uint256 {
    pub fn storage_key(&self) -> Vec<u8> {
        ethabi::encode(&[self.slot.abi_token()])
    }
}

/// Utility class to play around with Mapping storage values.
pub struct Mapping {
    pub slot: BigInt,
}

impl Mapping {
    /// For a given key, will calculate the abi encoded keccak preimage.
    /// Hashing the preimage will return the storage key where a given value from the mapping is.
    pub fn preimage(&self, key: &impl ABIEncodeable) -> Vec<u8> {
        ethabi::encode(&[key.abi_token(), self.slot.abi_token()])
    }

    /// Returns the EVM storage key where the given mapping key is stored.
    pub fn storage_key(&self, key: &impl ABIEncodeable) -> Vec<u8> {
        let preimage = self.preimage(key);
        let mut keccak = Keccak::v256();
        let mut output = [0u8; 32];
        keccak.update(preimage.as_slice());
        keccak.finalize(&mut output);
        return output.to_vec();
    }

    /// Given a keccak256 preimage, determines what is the key of the mapping associated to it.
    /// Returns None if the format is wrong or there isn't a match.
    pub fn key_from_preimage<T: ABIEncodeable>(&self, preimage: Vec<u8>) -> Option<T> {
        let slot = ethabi::encode(&[self.slot.abi_token()]);
        if !preimage.ends_with(slot.as_slice()) {
            return None;
        }

        let key = preimage.split_at(preimage.len() - slot.len()).0;
        let t = T::abi_decode(key.to_vec());
        if t.is_err() {
            return None;
        }
        Some(t.unwrap())
    }
}

pub trait ABIEncodeable {
    fn abi_token(&self) -> ethabi::Token;
    fn abi_decode(abi_encoded: Vec<u8>) -> Result<Self, ethabi::Error>
    where
        Self: Sized;
}

impl ABIEncodeable for BigInt {
    fn abi_token(&self) -> ethabi::Token {
        ethabi::Token::Uint(ethabi::Uint::from_big_endian(
            self.to_bytes_be().1.as_slice(),
        ))
    }

    fn abi_decode(abi_encoded: Vec<u8>) -> Result<Self, ethabi::Error> {
        let mut bytes: [u8; 32] = [0; 32];
        let first = ethabi::Uint::from_big_endian(abi_encoded.as_slice());
        first.to_big_endian(&mut bytes);
        Ok(BigInt::from_unsigned_bytes_be(&bytes))
    }
}

impl ABIEncodeable for Address {
    fn abi_token(&self) -> ethabi::Token {
        ethabi::Token::Address(self.clone())
    }

    fn abi_decode(abi_encoded: Vec<u8>) -> Result<Self, ethabi::Error> {
        let decoded = ethabi::decode(&[ethabi::ParamType::Address], abi_encoded.as_slice())?;
        Ok(decoded[0].to_owned().into_address().unwrap())
    }
}

#[cfg(test)]
mod tests {
    mod abi_encodeable {
        use std::str::FromStr;
        use substreams::scalar::BigInt;

        use super::super::*;
        use crate::hex::Hexable;

        #[test]
        fn test_abi_token() {
            struct TT<'a> {
                name: &'static str,
                abi_encodeable: Box<dyn ABIEncodeable + 'a>,
                encoded: &'static str,
            }

            let tests: Vec<TT> = vec![
                TT {
                    name: "Address impl",
                    abi_encodeable: Box::new(
                        Address::from_str("0xca0e8f557ea98f950029a41d74f16dd76648b1f1").unwrap(),
                    ),
                    encoded: "0x000000000000000000000000ca0e8f557ea98f950029a41d74f16dd76648b1f1",
                },
                TT {
                    name: "BigInt impl",
                    abi_encodeable: Box::new(BigInt::from(54)),
                    encoded: "0x0000000000000000000000000000000000000000000000000000000000000036",
                },
            ];

            for tt in tests {
                assert_eq!(
                    ethabi::encode(&[tt.abi_encodeable.abi_token()]).to_hex(),
                    tt.encoded,
                    "Failed at test with name: {}",
                    tt.name,
                );
            }
        }

        #[test]
        fn test_abi_decode_big_int() {
            {
                let encoded =
                    hex::decode("0000000000000000000000000000000000000000000000000000000000000036")
                        .unwrap();
                assert_eq!(
                    BigInt::abi_decode(encoded).unwrap(),
                    BigInt::from(54),
                    "Decode Normal Number"
                );
            }

            {
                let encoded =
                    hex::decode("ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff")
                        .unwrap();
                assert_eq!(
                    BigInt::abi_decode(encoded).unwrap(),
                    BigInt::from_str("115792089237316195423570985008687907853269984665640564039457584007913129639935").unwrap(),
                    "Decode Max Uint256"
                );
            }
        }

        #[test]
        fn test_abi_decode_address() {
            let encoded =
                hex::decode("000000000000000000000000ca0e8f557ea98f950029a41d74f16dd76648b1f1")
                    .unwrap();
            assert_eq!(
                Address::abi_decode(encoded).unwrap(),
                Address::from_str("0xca0e8f557ea98f950029a41d74f16dd76648b1f1").unwrap(),
                "Decode Padded Address",
            )
        }
    }

    mod storage {
        use std::str::FromStr;

        use substreams::{scalar::BigInt, Hex};

        use super::super::*;
        use crate::{hex::Hexable, storage::Mapping};

        #[test]
        fn test_mapping_address_key() {
            let m = Mapping {
                slot: BigInt::from(12),
            };
            let preimage = Hex::decode("0000000000000000000000001a13f4ca1d028320a707d99520abfefca3998b7f000000000000000000000000000000000000000000000000000000000000000c").unwrap();
            let encoded = "0x9e3150b55a3c0fe6929063dde4bd380cd1b3bd4bd4fca20b85691cb17e2880da";
            let key = Address::from_str("0x1a13f4ca1d028320a707d99520abfefca3998b7f").unwrap();

            assert_eq!(m.preimage(&key).to_hex(), preimage.to_hex());
            assert_eq!(m.storage_key(&key).to_hex(), encoded);
            assert_eq!(
                m.key_from_preimage::<Address>(preimage).unwrap().to_hex(),
                key.to_hex()
            );
        }

        #[test]
        fn test_mapping_big_int_key() {
            let m = Mapping {
                slot: BigInt::from(12),
            };
            let preimage = Hex::decode("000000000000000000000000000000000000000000000000000000000000007f000000000000000000000000000000000000000000000000000000000000000c").unwrap();
            let encoded = "0x20e322d575d78a86a3761da49565fbbf1fcb4f0a86011c98ac148d9ee6b9c682";
            let key = BigInt::from(127);

            assert_eq!(m.preimage(&key).to_hex(), preimage.to_hex());
            assert_eq!(m.storage_key(&key).to_hex(), encoded);
            assert_eq!(m.key_from_preimage::<BigInt>(preimage).unwrap(), key);
        }
    }
}
