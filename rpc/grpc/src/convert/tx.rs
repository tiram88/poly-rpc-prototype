use std::str::FromStr;
use rpc_core::{RpcHash, RpcHexData, RpcError, RpcResult};
use crate::protowire;

// ----------------------------------------------------------------------------
// rpc_core to protowire
// ----------------------------------------------------------------------------

impl From<&rpc_core::RpcTransaction> for protowire::RpcTransaction {
    fn from(item: &rpc_core::RpcTransaction) -> Self {
        Self {
            version: item.version,
            inputs: item.inputs
                .iter()
                .map(|x| protowire::RpcTransactionInput::from(x))
                .collect(),
            outputs: item.outputs
                .iter()
                .map(|x| protowire::RpcTransactionOutput::from(x))
                .collect(),
            lock_time: item.lock_time,
            subnetwork_id: item.subnetwork_id.to_string(),
            gas: item.gas,
            payload: item.payload.to_string(),
            verbose_data: Some((&item.verbose_data).into()),
        }
    }
}

impl From<&rpc_core::RpcTransactionInput> for protowire::RpcTransactionInput {
    fn from(item: &rpc_core::RpcTransactionInput) -> Self {
        Self {
            previous_outpoint: Some((&item.previous_outpoint).into()),
            signature_script: item.signature_script.to_string(),
            sequence: item.sequence,
            sig_op_count: item.sig_op_count,
            verbose_data: Some((&item.verbose_data).into()),
        }
    }
}

impl From<&rpc_core::RpcTransactionOutput> for protowire::RpcTransactionOutput {
    fn from(item: &rpc_core::RpcTransactionOutput) -> Self {
        Self {
            amount: item.amount,
            script_public_key: Some((&item.script_public_key).into()),
            verbose_data: Some((&item.verbose_data).into()),
        }
    }
}

impl From<&rpc_core::RpcOutpoint> for protowire::RpcOutpoint {
    fn from(item: &rpc_core::RpcOutpoint) -> Self {
        Self {
            transaction_id: item.transaction_id.to_string(),
            index: item.index,
        }
    }
}

impl From<&rpc_core::RpcUtxoEntry> for protowire::RpcUtxoEntry {
    fn from(item: &rpc_core::RpcUtxoEntry) -> Self {
        Self {
            amount: item.amount,
            script_public_key: Some((&item.script_public_key).into()),
            block_daa_score: item.block_daa_score,
            is_coinbase: item.is_coinbase,
        }
    }
}

impl From<&rpc_core::RpcScriptPublicKey> for protowire::RpcScriptPublicKey {
    fn from(item: &rpc_core::RpcScriptPublicKey) -> Self {
        Self {
            version: item.version.into(),
            script_public_key: item.script_public_key.to_string(),
        }
    }
}

impl From<&rpc_core::RpcTransactionVerboseData> for protowire::RpcTransactionVerboseData {
    fn from(item: &rpc_core::RpcTransactionVerboseData) -> Self {
        Self {
            transaction_id: item.transaction_id.to_string(),
            hash: item.hash.to_string(),
            mass: item.mass,
            block_hash: item.block_hash.to_string(),
            block_time: item.block_time,
        }
    }
}

impl From<&rpc_core::RpcTransactionInputVerboseData> for protowire::RpcTransactionInputVerboseData {
    fn from(_item: &rpc_core::RpcTransactionInputVerboseData) -> Self {
        Self {}
    }
}

impl From<&rpc_core::RpcTransactionOutputVerboseData> for protowire::RpcTransactionOutputVerboseData {
    fn from(item: &rpc_core::RpcTransactionOutputVerboseData) -> Self {
        Self {
            script_public_key_type: item.script_public_key_type.to_string(),
            script_public_key_address: item.script_public_key_address.clone(),
        }
    }
}

// ----------------------------------------------------------------------------
// protowire to rpc_core
// ----------------------------------------------------------------------------

impl TryFrom<&protowire::RpcTransaction> for rpc_core::RpcTransaction {
    type Error = RpcError;
    fn try_from(item: &protowire::RpcTransaction) -> RpcResult<Self> {
        Ok(Self {
            version: item.version,
            inputs: item.inputs
                .iter()
                .map(|x| rpc_core::RpcTransactionInput::try_from(x))
                .collect::<RpcResult<Vec<rpc_core::RpcTransactionInput>>>()?,
            outputs: item.outputs
                .iter()
                .map(|x| rpc_core::RpcTransactionOutput::try_from(x))
                .collect::<RpcResult<Vec<rpc_core::RpcTransactionOutput>>>()?,
            lock_time: item.lock_time,
            subnetwork_id: rpc_core::RpcSubnetworkId::from_str(&item.subnetwork_id)?,
            gas: item.gas,
            payload: RpcHexData::from_str(&item.payload)?,
            verbose_data: item.verbose_data
                .as_ref()
                .ok_or(RpcError::MissingRpcFieldError("RpcTransaction".to_string(), "verbose_data".to_string()))?
                .try_into()?,
        })
    }
}

impl TryFrom<&protowire::RpcTransactionInput> for rpc_core::RpcTransactionInput {
    type Error = RpcError;
    fn try_from(item: &protowire::RpcTransactionInput) -> RpcResult<Self> {
        Ok(Self {
            previous_outpoint: item.previous_outpoint
                .as_ref()
                .ok_or(RpcError::MissingRpcFieldError("RpcTransactionInput".to_string(), "previous_outpoint".to_string()))?
                .try_into()?,
            signature_script: RpcHexData::from_str(&item.signature_script)?,
                sequence: item.sequence,
                sig_op_count: item.sig_op_count,
            verbose_data: item.verbose_data
                .as_ref()
                .ok_or(RpcError::MissingRpcFieldError("RpcTransactionInput".to_string(), "verbose_data".to_string()))?
                .try_into()?,
        })
    }
}

impl TryFrom<&protowire::RpcTransactionOutput> for rpc_core::RpcTransactionOutput {
    type Error = RpcError;
    fn try_from(item: &protowire::RpcTransactionOutput) -> RpcResult<Self> {
        Ok(Self {
            amount: item.amount,
            script_public_key: item.script_public_key
                .as_ref()
                .ok_or(RpcError::MissingRpcFieldError("RpcTransactionOutput".to_string(), "script_public_key".to_string()))?
                .try_into()?,
            verbose_data: item.verbose_data
                .as_ref()
                .ok_or(RpcError::MissingRpcFieldError("RpcTransactionOutput".to_string(), "verbose_data".to_string()))?
                .try_into()?,
        })
    }
}

impl TryFrom<&protowire::RpcOutpoint> for rpc_core::RpcOutpoint {
    type Error = RpcError;
    fn try_from(item: &protowire::RpcOutpoint) -> RpcResult<Self> {
        Ok(Self {
            transaction_id: RpcHash::from_str(&item.transaction_id)?,
            index: item.index,
        })
    }
}

impl TryFrom<&protowire::RpcUtxoEntry> for rpc_core::RpcUtxoEntry {
    type Error = RpcError;
    fn try_from(item: &protowire::RpcUtxoEntry) -> RpcResult<Self> {
        Ok(Self {
            amount: item.amount,
            script_public_key: item.script_public_key
                .as_ref()
                .ok_or(RpcError::MissingRpcFieldError("RpcTransactionOutput".to_string(), "script_public_key".to_string()))?
                .try_into()?,
            block_daa_score: item.block_daa_score,
            is_coinbase: item.is_coinbase,
        })
    }
}

impl TryFrom<&protowire::RpcScriptPublicKey> for rpc_core::RpcScriptPublicKey {
    type Error = RpcError;
    fn try_from(item: &protowire::RpcScriptPublicKey) -> RpcResult<Self> {
        Ok(Self {
            version: u16::try_from(item.version)?,
            script_public_key: item.script_public_key.as_str().try_into()?,
        })
    }
}

impl TryFrom<&protowire::RpcTransactionVerboseData> for rpc_core::RpcTransactionVerboseData {
    type Error = RpcError;
    fn try_from(item: &protowire::RpcTransactionVerboseData) -> RpcResult<Self> {
        Ok(Self {
            transaction_id: RpcHash::from_str(&item.transaction_id)?,
            hash: RpcHash::from_str(&item.hash)?,
            mass: item.mass,
            block_hash: RpcHash::from_str(&item.block_hash)?,
            block_time: item.block_time,
        })
    }
}

impl TryFrom<&protowire::RpcTransactionInputVerboseData> for rpc_core::RpcTransactionInputVerboseData {
    type Error = RpcError;
    fn try_from(_item: &protowire::RpcTransactionInputVerboseData) -> RpcResult<Self> {
        Ok(Self {})
    }
}

impl TryFrom<&protowire::RpcTransactionOutputVerboseData> for rpc_core::RpcTransactionOutputVerboseData {
    type Error = RpcError;
    fn try_from(item: &protowire::RpcTransactionOutputVerboseData) -> RpcResult<Self> {
        Ok(Self {
            script_public_key_type: item.script_public_key_type.parse()?,
            script_public_key_address: item.script_public_key_address.clone(),
        })
    }
}