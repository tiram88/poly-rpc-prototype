use hashes::{ Hash, HASH_SIZE };
use serde::{Deserialize, Serialize};
use borsh::{BorshSerialize, BorshDeserialize, BorshSchema};

use std::fmt::{Debug, Display, Formatter};
use std::str::{self, FromStr};

// pub struct RpcError {
//     pub message : String,
// }

#[derive(Debug, Serialize, Deserialize, BorshSerialize, BorshDeserialize, BorshSchema)]
#[serde(rename_all = "camelCase")]
pub struct RpcHash([u8; HASH_SIZE]);

impl RpcHash {
    #[inline(always)]
    pub const fn from_bytes(bytes: [u8; HASH_SIZE]) -> Self {
        RpcHash(bytes)
    }

    #[inline(always)]
    pub const fn as_bytes(self) -> [u8; 32] {
        self.0
    }

    #[inline(always)]
    /// # Panics
    /// Panics if `bytes` length is not exactly `HASH_SIZE`.
    pub fn from_slice(bytes: &[u8]) -> Self {
        Self(<[u8; HASH_SIZE]>::try_from(bytes).expect("Slice must have the length of RpcHash"))
    }

    #[inline(always)]
    pub fn to_le_u64(self) -> [u64; 4] {
        let mut out = [0u64; 4];
        out.iter_mut().zip(self.iter_le_u64()).for_each(|(out, word)| *out = word);
        out
    }

    #[inline(always)]
    pub fn iter_le_u64(&self) -> impl ExactSizeIterator<Item = u64> + '_ {
        self.0.chunks_exact(8).map(|chunk| u64::from_le_bytes(chunk.try_into().unwrap()))
    }

    #[inline(always)]
    pub fn from_le_u64(arr: [u64; 4]) -> Self {
        let mut ret = [0; HASH_SIZE];
        ret.chunks_exact_mut(8).zip(arr.iter()).for_each(|(bytes, word)| bytes.copy_from_slice(&word.to_le_bytes()));
        Self(ret)
    }

    #[inline(always)]
    pub fn from_u64_word(word: u64) -> Self {
        Self::from_le_u64([word, 0, 0, 0])
    }
}

impl Display for RpcHash {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut hex = [0u8; HASH_SIZE * 2];
        faster_hex::hex_encode(&self.0, &mut hex).expect("The output is exactly twice the size of the input");
        f.write_str(str::from_utf8(&hex).expect("hex is always valid UTF-8"))
    }
}

impl FromStr for RpcHash {
    type Err = faster_hex::Error;

    #[inline]
    fn from_str(hash_str: &str) -> Result<Self, Self::Err> {
        let mut bytes = [0u8; HASH_SIZE];
        faster_hex::hex_decode(hash_str.as_bytes(), &mut bytes)?;
        Ok(RpcHash(bytes))
    }
}

impl From<u64> for RpcHash {
    fn from(word: u64) -> Self {
        Self::from_u64_word(word)
    }
}

impl AsRef<[u8]> for RpcHash {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl From<&Hash> for RpcHash {
    fn from(item: &Hash) -> RpcHash {
        RpcHash::from_bytes(item.as_bytes().clone())
    }
}