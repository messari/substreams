use ethabi::ethereum_types::H256;
use tiny_keccak::{Hasher, Keccak};

pub fn keccak_256<S>(bytes: S) -> [u8; 32]
where
    S: AsRef<[u8]>,
{
    let mut output = [0u8; 32];
    let mut hasher = Keccak::v256();
    hasher.update(bytes.as_ref());
    hasher.finalize(&mut output);
    output
}

pub fn name_hash(name: &str) -> H256 {
    if name.is_empty() {
        return H256::zero();
    }

    name.rsplit('.')
        .fold([0u8; 32], |node, label| {
            keccak_256(&[node, keccak_256(label.as_bytes())].concat())
        })
        .into()
}
