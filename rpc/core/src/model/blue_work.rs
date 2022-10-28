extern crate derive_more;

use std::fmt::{Debug, Display, Formatter};
use std::str::{self, FromStr};
use std::convert::TryFrom;
use borsh::{BorshSerialize, BorshDeserialize, BorshSchema};
use serde::{Serialize, Deserialize};
use consensus_core::BlueWorkType;

use crate::errors;


#[repr(transparent)]
#[derive(Debug, PartialEq, Copy, Clone, Serialize, Deserialize, BorshSerialize, BorshDeserialize, BorshSchema)]
#[serde(rename_all = "camelCase", try_from = "String", into = "String")]
pub struct RpcBlueWorkType(BlueWorkType);

impl Display for RpcBlueWorkType {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{0:x}", self.0))
    }
}
impl From<BlueWorkType> for RpcBlueWorkType {
    fn from(item: BlueWorkType) -> RpcBlueWorkType {
        RpcBlueWorkType(item)
    }
}

impl From<RpcBlueWorkType> for BlueWorkType {
    fn from(item: RpcBlueWorkType) -> BlueWorkType {
        item.0
    }
}

impl From<RpcBlueWorkType> for String {
    fn from(item: RpcBlueWorkType) -> String {
        item.to_string()
    }
}

impl FromStr for RpcBlueWorkType {
    type Err = errors::RpcError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(RpcBlueWorkType(u128::from_str_radix(s, 16)?))
    }
}

impl TryFrom<&str> for RpcBlueWorkType {
    type Error = errors::RpcError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl TryFrom<String> for RpcBlueWorkType {
    type Error = errors::RpcError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rpc_blue_work() {
        const HEX_STR: &str = "1234567890abcdef1234567890abc";
        const HEX_BIN: u128 = 0x1234567890abcdef1234567890abc;

        let bw: BlueWorkType = 123456789012345678901234567890123456789;
        let rbw: RpcBlueWorkType = bw.into();
        assert_eq!(bw, rbw.into());

        assert!(RpcBlueWorkType::try_from(HEX_STR).is_ok());
        assert!(RpcBlueWorkType::try_from("not a number").is_err());

        assert!(RpcBlueWorkType::from_str(HEX_STR).is_ok());
        assert!(RpcBlueWorkType::from_str("not a number").is_err());

        let b1 = RpcBlueWorkType::try_from(HEX_STR).unwrap();
        assert_eq!(b1.to_string(), HEX_STR);
        assert_eq!(b1, RpcBlueWorkType(HEX_BIN));

        let b2: u128 = u128::from_str_radix("a12", 16).unwrap();
        assert!(RpcBlueWorkType::try_from("a12").is_ok());
        assert_eq!(RpcBlueWorkType(b2), RpcBlueWorkType::from_str("a12").unwrap());

        let rbw2 = rbw;
        assert_eq!(rbw, rbw2);
    }
}