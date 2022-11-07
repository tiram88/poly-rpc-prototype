use std::{fmt::{Display, Formatter}, str::FromStr};
use serde::{Deserialize, Serialize};
use borsh::{BorshSerialize, BorshDeserialize, BorshSchema};
use crate::RpcError;


#[derive(Clone, Debug, Serialize, Deserialize, BorshSerialize, BorshDeserialize, BorshSchema)]
#[repr(u8)]
pub enum RpcScriptClass {
    /// None of the recognized forms.
	NonStandardTy = 0,
    
    /// Pay to pubkey.
	PubKeyTy = 1,

    /// Pay to pubkey ECDSA.
	PubKeyECDSATy = 2,

    /// Pay to script hash.
	ScriptHashTy = 3,
}


impl RpcScriptClass {
    fn as_str(&self) -> &'static str {
        match self {
            RpcScriptClass::NonStandardTy => "nonstandard",
            RpcScriptClass::PubKeyTy => "pubkey",
            RpcScriptClass::PubKeyECDSATy => "pubkeyecdsa",
            RpcScriptClass::ScriptHashTy => "scripthash",
        }
    }
}

impl Display for RpcScriptClass {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for RpcScriptClass {
    type Err = RpcError;

    fn from_str(script_class: &str) -> Result<Self, Self::Err> {
        match script_class {
            "nonstandard" => Ok(RpcScriptClass::NonStandardTy),
            "pubkey" => Ok(RpcScriptClass::PubKeyTy),
            "pubkeyecdsa" => Ok(RpcScriptClass::PubKeyECDSATy),
            "scripthash" => Ok(RpcScriptClass::ScriptHashTy),

            _ => Err(RpcError::InvalidRpcScriptClass(script_class.to_string()))
        }
    }
}

impl TryFrom<&str> for RpcScriptClass {
    type Error = RpcError;

    fn try_from(script_class: &str) -> Result<Self, Self::Error> {
        script_class.parse()
    }
}
