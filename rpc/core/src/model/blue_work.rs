extern crate derive_more;

use std::str::FromStr;
use std::convert::TryFrom;
use derive_more::{ Display };
use borsh::{BorshSerialize, BorshDeserialize, BorshSchema};
use serde::{Serialize, Deserialize};
use consensus_core::BlueWorkType;

use crate::errors;


#[repr(transparent)]
#[derive(Display, Debug, PartialEq, Copy, Clone, Serialize, Deserialize, BorshSerialize, BorshDeserialize, BorshSchema)]
#[serde(rename_all = "camelCase", try_from = "String", into = "String")]
pub struct RpcBlueWorkType(BlueWorkType);

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
      item.0.to_string()
    }
}

impl FromStr for RpcBlueWorkType {
    type Err = errors::RpcError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<u128>() {
            Ok(v) => Ok(RpcBlueWorkType(v)),
            Err(err) => Err(err.into()),
        }
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
        let bw: BlueWorkType = 123456789012345678901234567890123456789;
        let rbw: RpcBlueWorkType = bw.into();
        assert_eq!(bw, rbw.into());

        assert!(RpcBlueWorkType::try_from("123456789012345678901234567890123456789").is_ok());
        assert!(RpcBlueWorkType::try_from("not a number").is_err());

        assert!(RpcBlueWorkType::from_str("123456789012345678901234567890123456789").is_ok());
        assert!(RpcBlueWorkType::from_str("not a number").is_err());

        let rbw2 = rbw;
        assert_eq!(rbw, rbw2);
    }
}