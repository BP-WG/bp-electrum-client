//! Utilities helping to handle Electrum-related data.

use amplify::ByteArray;
use bpstd::{BlockMerkleRoot, Txid};
use sha2::{Digest, Sha256};
use types::GetMerkleRes;

/// Verifies a Merkle inclusion proof as retrieved via [`transaction_get_merkle`] for a transaction with the
/// given `txid` and `merkle_root` as included in the [`BlockHeader`].
///
/// Returns `true` if the transaction is included in the corresponding block, and `false`
/// otherwise.
///
/// [`transaction_get_merkle`]: crate::ElectrumApi::transaction_get_merkle
/// [`BlockHeader`]: bitcoin::BlockHeader
pub fn validate_merkle_proof(
    txid: &Txid,
    merkle_root: &BlockMerkleRoot,
    merkle_res: &GetMerkleRes,
) -> bool {
    let mut index = merkle_res.pos;
    let mut cur = txid.to_byte_array();
    for bytes in &merkle_res.merkle {
        let mut reversed = [0u8; 32];
        reversed.copy_from_slice(bytes);
        reversed.reverse();
        // unwrap() safety: `reversed` has len 32 so `from_slice` can never fail.
        let next_hash = reversed;

        let (left, right) = if index % 2 == 0 {
            (cur, next_hash)
        } else {
            (next_hash, cur)
        };

        let data = [&left[..], &right[..]].concat();
        cur = Sha256::digest(Sha256::digest(&data).as_slice()).into();
        index /= 2;
    }

    cur == merkle_root.to_byte_array()
}
