#[derive(Clone, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum RpcApiOps {
    Ping = 0,
    GetCurrentNetwork,
    SubmitBlock,
    GetBlockTemplate,
    GetPeerAddresses,
    GetSelectedTipHash,
    GetMempoolEntry,
    GetMempoolEntries,
    GetConnectedPeerInfo,
    AddPeer,
    SubmitTransaction,
    GetBlock,
    GetSubnetwork,
    GetVirtualSelectedParentChainFromBlock,
    GetBlocks,
    GetBlockCount,
    GetBlockDagInfo,
    ResolveFinalityConflict,
    Shutdown,
    GetHeaders,
    GetUtxosByAddresses,
    GetBalanceByAddress,
    GetBalancesByAddresses,
    GetVirtualSelectedParentBlueScore,
    Ban,
    Unban,
    GetInfo,
    EstimateNetworkHashesPerSecond,
    GetMempoolEntriesByAddresses,
    GetCoinSupply,
    Notify,
}

impl Into<u32> for RpcApiOps {
    fn into(self) -> u32 {
        self as u32
    }
}