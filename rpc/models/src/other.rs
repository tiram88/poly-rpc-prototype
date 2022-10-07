use serde::{Deserialize, Serialize};
use borsh::{BorshSerialize, BorshDeserialize, BorshSchema};
use super::stubs::*;



//   ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// ^ ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
//   ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// ^ ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
//   ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// GetCurrentNetworkRequest requests the network kaspad is currently running against.
///
/// Possible networks are: Mainnet, Testnet, Simnet, Devnet
#[derive(Debug, Serialize, Deserialize, BorshSerialize, BorshDeserialize, BorshSchema)]
#[serde(rename_all = "camelCase")]
pub struct GetCurrentNetworkRequest;

#[derive(Debug, Serialize, Deserialize, BorshSerialize, BorshDeserialize, BorshSchema)]
#[serde(rename_all = "camelCase")]
pub struct GetCurrentNetworkResponse {
    pub current_network: NetworkType, // FIXME
    // pub current_network: String, // FIXME
    // RpcError error = 1000;
    // error : RpcError,
}

/// SubmitBlockRequest requests to submit a block into the DAG.
/// Blocks are generally expected to have been generated using the getBlockTemplate call.
///
/// See: [`GetBlockTemplateRequest`]
#[derive(Debug, Serialize, Deserialize, BorshSerialize, BorshDeserialize, BorshSchema)]
#[serde(rename_all = "camelCase")]
pub struct SubmitBlockRequest {
    pub block: RpcBlock,
    // allowNonDAABlocks
    #[serde(alias = "allowNonDAABlocks")]
    pub allow_non_daa_blocks: bool,
}


#[derive(Debug, Serialize, Deserialize, BorshSerialize, BorshDeserialize, BorshSchema)]
#[serde(rename_all = "camelCase")]
pub enum SubmitBlockRejectReason  {
    // None = 0,
    BlockInvalid = 1,
    IsInIBD = 2
}

#[derive(Debug, Serialize, Deserialize, BorshSerialize, BorshDeserialize, BorshSchema)]
#[serde(rename_all = "camelCase")]
pub enum SubmitBlockResponse  {
    Success,
    Reject(SubmitBlockRejectReason),
    // RpcError(RpcError)
}

// pub struct SubmitBlockResponse {
//     enum RejectReason  {
//         NONE = 0;
//         BLOCK_INVALID = 1;
//         IS_IN_IBD = 2;
//     }
//     RejectReason rejectReason = 1;
//     RpcError error = 1000;
// }

/// GetBlockTemplateRequest requests a current block template.
/// Callers are expected to solve the block template and submit it using the submitBlock call
///
/// See: [`SubmitBlockRequest`]
#[derive(Debug, Serialize, Deserialize, BorshSerialize, BorshDeserialize, BorshSchema)]
#[serde(rename_all = "camelCase")]
pub struct GetBlockTemplateRequest {
    /// Which kaspa address should the coinbase block reward transaction pay into
    pub pay_address: Address,
    pub extra_data: String, // FIXME
}

#[derive(Debug, Serialize, Deserialize, BorshSerialize, BorshDeserialize, BorshSchema)]
#[serde(rename_all = "camelCase")]
pub struct GetBlockTemplateResponse {
    pub block: RpcBlock,

    /// Whether kaspad thinks that it's synced.
    /// Callers are discouraged (but not forbidden) from solving blocks when kaspad is not synced.
    /// That is because when kaspad isn't in sync with the rest of the network there's a high
    /// chance the block will never be accepted, thus the solving effort would have been wasted.
    pub is_synced: bool,

    // RpcError error = 1000;
}

/// NotifyBlockAddedRequest registers this connection for blockAdded notifications.
///
/// See: [`BlockAddedNotification`]
#[derive(Debug, Serialize, Deserialize, BorshSerialize, BorshDeserialize, BorshSchema)]
#[serde(rename_all = "camelCase")]
pub struct NotifyBlockAddedRequest;

#[derive(Debug, Serialize, Deserialize, BorshSerialize, BorshDeserialize, BorshSchema)]
#[serde(rename_all = "camelCase")]
pub struct NotifyBlockAddedResponse {
    // RpcError error = 1000;
}

/// BlockAddedNotification is sent whenever a blocks has been added (NOT accepted)
/// into the DAG.
///
/// See: [`NotifyBlockAddedRequest`]
#[derive(Debug, Serialize, Deserialize, BorshSerialize, BorshDeserialize, BorshSchema)]
#[serde(rename_all = "camelCase")]
pub struct BlockAddedNotification {
    block: RpcBlock,
}

/// GetPeerAddressesRequest requests the list of known kaspad addresses in the
/// current network. (mainnet, testnet, etc.)
#[derive(Debug, Serialize, Deserialize, BorshSerialize, BorshDeserialize, BorshSchema)]
#[serde(rename_all = "camelCase")]
pub struct GetPeerAddressesRequest;


#[derive(Debug, Serialize, Deserialize, BorshSerialize, BorshDeserialize, BorshSchema)]
#[serde(rename_all = "camelCase")]
pub struct GetPeerAddressesResponse {
    pub addresses: Vec<String>, // FIXME GetPeerAddressesKnownAddress>,
    pub banned_addresses: Vec<String>, // FIXME GetPeerAddressesKnownAddress,
    // RpcError error = 1000;
}

// pub struct GetPeerAddressesKnownAddress  {
//     string Addr = 1;
// }

/// GetSelectedTipHashRequest requests the hash of the current virtual's
/// selected parent.
#[derive(Debug, Serialize, Deserialize, BorshSerialize, BorshDeserialize, BorshSchema)]
#[serde(rename_all = "camelCase")]
pub struct GetSelectedTipHashRequest;

#[derive(Debug, Serialize, Deserialize, BorshSerialize, BorshDeserialize, BorshSchema)]
#[serde(rename_all = "camelCase")]
pub struct GetSelectedTipHashResponse {
    pub selected_tip_hash: String, // FIXME
    // RpcError error = 1000;
}

/// GetMempoolEntryRequest requests information about a specific transaction
/// in the mempool.
#[derive(Debug, Serialize, Deserialize, BorshSerialize, BorshDeserialize, BorshSchema)]
#[serde(rename_all = "camelCase")]
pub struct GetMempoolEntryRequest {
    /// The transaction's TransactionID.
    pub tx_id: String, 
    pub include_orphan_pool: bool,
    pub filter_transaction_pool: bool,
}

#[derive(Debug, Serialize, Deserialize, BorshSerialize, BorshDeserialize, BorshSchema)]
#[serde(rename_all = "camelCase")]
pub struct GetMempoolEntryResponse {
    pub entry: MempoolEntry
    // RpcError error = 1000;
}

/// GetMempoolEntriesRequest requests information about all the transactions
/// currently in the mempool.
#[derive(Debug, Serialize, Deserialize, BorshSerialize, BorshDeserialize, BorshSchema)]
#[serde(rename_all = "camelCase")]
pub struct GetMempoolEntriesRequest {
    pub include_orphan_pool: bool,
    pub filter_transaction_pool: bool,
}

#[derive(Debug, Serialize, Deserialize, BorshSerialize, BorshDeserialize, BorshSchema)]
#[serde(rename_all = "camelCase")]
pub struct GetMempoolEntriesResponse {
    pub entries: Vec<MempoolEntry>,
}

#[derive(Debug, Serialize, Deserialize, BorshSerialize, BorshDeserialize, BorshSchema)]
#[serde(rename_all = "camelCase")]
pub struct MempoolEntry {
    pub fee: u64,
    pub transaction: RpcTransaction,
    pub is_orphan: bool,
}

/// GetConnectedPeerInfoRequest requests information about all the p2p peers
/// currently connected to this kaspad.
#[derive(Debug, Serialize, Deserialize, BorshSerialize, BorshDeserialize, BorshSchema)]
#[serde(rename_all = "camelCase")]
pub struct GetConnectedPeerInfoRequest;

#[derive(Debug, Serialize, Deserialize, BorshSerialize, BorshDeserialize, BorshSchema)]
#[serde(rename_all = "camelCase")]
pub struct GetConnectedPeerInfoResponse {
    pub infos: Vec<GetConnectedPeerInfo>
}

#[derive(Debug, Serialize, Deserialize, BorshSerialize, BorshDeserialize, BorshSchema)]
#[serde(rename_all = "camelCase")]
pub struct GetConnectedPeerInfo {
    pub id: String, // FIXME
    pub address: String, // FIXME

    /// How long did the last ping/pong exchange take
    pub last_ping_duration: i64,

    /// Whether this kaspad initiated the connection
    pub is_outbound: bool,
    pub time_offset: i64,
    pub user_agent: String,

    /// The protocol version that this peer claims to support
    pub advertised_protocol_version: u32,

    /// The timestamp of when this peer connected to this kaspad
    pub time_connected: i64,

    /// Whether this peer is the IBD peer (if IBD is running)
    pub is_ibd_peer: bool,
}

/// AddPeerRequest adds a peer to kaspad's outgoing connection list.
/// This will, in most cases, result in kaspad connecting to said peer.
#[derive(Debug, Serialize, Deserialize, BorshSerialize, BorshDeserialize, BorshSchema)]
#[serde(rename_all = "camelCase")]
pub struct AddPeerRequest {
    pub address: String, // FIXME

    /// Whether to keep attempting to connect to this peer after disconnection
    pub is_permanent: bool,
}

#[derive(Debug, Serialize, Deserialize, BorshSerialize, BorshDeserialize, BorshSchema)]
#[serde(rename_all = "camelCase")]
pub struct AddPeerResponse;

/// SubmitTransactionRequest submits a transaction to the mempool
#[derive(Debug, Serialize, Deserialize, BorshSerialize, BorshDeserialize, BorshSchema)]
#[serde(rename_all = "camelCase")]
pub struct SubmitTransactionRequest {
    pub transaction: RpcTransaction,
    pub allow_orphan: bool,
}

#[derive(Debug, Serialize, Deserialize, BorshSerialize, BorshDeserialize, BorshSchema)]
#[serde(rename_all = "camelCase")]
pub struct SubmitTransactionResponse {
    /// The transaction ID of the submitted transaction
    pub transaction_id: String, // FIXME
    // RpcError error = 1000;
}

/// NotifyVirtualSelectedParentChainChangedRequest registers this connection for virtualSelectedParentChainChanged notifications.
///
/// See: [`VirtualSelectedParentChainChangedNotification`]
#[derive(Debug, Serialize, Deserialize, BorshSerialize, BorshDeserialize, BorshSchema)]
#[serde(rename_all = "camelCase")]
pub struct NotifyVirtualSelectedParentChainChangedRequest {
    pub include_accepted_transaction_ids: bool,
}

#[derive(Debug, Serialize, Deserialize, BorshSerialize, BorshDeserialize, BorshSchema)]
#[serde(rename_all = "camelCase")]
pub struct NotifyVirtualSelectedParentChainChangedResponse {
    // RpcError error = 1000;
}

/// VirtualSelectedParentChainChangedNotification is sent whenever the DAG's selected parent
/// chain had changed.
///
/// See: [`NotifyVirtualSelectedParentChainChangedRequest`]
#[derive(Debug, Serialize, Deserialize, BorshSerialize, BorshDeserialize, BorshSchema)]
#[serde(rename_all = "camelCase")]
pub struct VirtualSelectedParentChainChangedNotification {
    /// The chain blocks that were removed, in high-to-low order
    pub removed_chain_block_hashes: Vec<String>,

    /// The chain blocks that were added, in low-to-high order
    pub added_chain_block_hashes: Vec<String>,

    /// Will be filled only if `includeAcceptedTransactionIds = true` in the notify request.
    pub accepted_transaction_ids: Vec<AcceptedTransactionIds>,
}

/// GetBlockRequest requests information about a specific block
#[derive(Debug, Serialize, Deserialize, BorshSerialize, BorshDeserialize, BorshSchema)]
#[serde(rename_all = "camelCase")]
pub struct GetBlockRequest {
    /// The hash of the requested block
    pub hash: String, // FIXME

    /// Whether to include transaction data in the response
    pub include_transactions: bool,
}

#[derive(Debug, Serialize, Deserialize, BorshSerialize, BorshDeserialize, BorshSchema)]
#[serde(rename_all = "camelCase")]
pub struct GetBlockResponse {
    pub block: RpcBlock,
    // RpcError error = 1000;
}

/// GetSubnetworkRequest requests information about a specific subnetwork
///
/// Currently unimplemented
#[derive(Debug, Serialize, Deserialize, BorshSerialize, BorshDeserialize, BorshSchema)]
#[serde(rename_all = "camelCase")]
pub struct GetSubnetworkRequest {
    pub subnetwork_id: String, // FIXME
}

#[derive(Debug, Serialize, Deserialize, BorshSerialize, BorshDeserialize, BorshSchema)]
#[serde(rename_all = "camelCase")]
pub struct GetSubnetworkResponse {
    pub gas_limit: u64,
    // RpcError error = 1000;
}

/// GetVirtualSelectedParentChainFromBlockRequest requests the virtual selected
/// parent chain from some startHash to this kaspad's current virtual
#[derive(Debug, Serialize, Deserialize, BorshSerialize, BorshDeserialize, BorshSchema)]
#[serde(rename_all = "camelCase")]
pub struct GetVirtualSelectedParentChainFromBlockRequest {
    pub start_hash: String, // FIXME
    pub include_accepted_transaction_ids: bool
}

#[derive(Debug, Serialize, Deserialize, BorshSerialize, BorshDeserialize, BorshSchema)]
#[serde(rename_all = "camelCase")]
pub struct AcceptedTransactionIds {
    pub accepting_block_hash: String, // FIXME
    pub accepted_transaction_ids: Vec<String>, // FIXME
}

#[derive(Debug, Serialize, Deserialize, BorshSerialize, BorshDeserialize, BorshSchema)]
#[serde(rename_all = "camelCase")]
pub struct GetVirtualSelectedParentChainFromBlockResponse {
    /// The chain blocks that were removed, in high-to-low order
    pub removed_chain_block_hashes: Vec<String>, // FIXME

    /// The chain blocks that were added, in low-to-high order
    pub added_chain_block_hashes: Vec<String>, // FIXME

    /// The transactions accepted by each block in addedChainBlockHashes.
    /// Will be filled only if `includeAcceptedTransactionIds = true` in the request.
    pub accepted_transaction_ids: Vec<AcceptedTransactionIds>,

    // RpcError error = 1000;
}

/// GetBlocksRequest requests blocks between a certain block lowHash up to this
/// kaspad's current virtual.
#[derive(Debug, Serialize, Deserialize, BorshSerialize, BorshDeserialize, BorshSchema)]
#[serde(rename_all = "camelCase")]
pub struct GetBlocksRequest {
    pub low_hash: String,
    pub include_blocks: bool,
    pub include_transactions: bool,
}

#[derive(Debug, Serialize, Deserialize, BorshSerialize, BorshDeserialize, BorshSchema)]
#[serde(rename_all = "camelCase")]
pub struct GetBlocksResponse {
    pub block_hashes: Vec<String>,
    pub blocks: Vec<RpcBlock>,
    // RpcError error = 1000;
}

/// GetBlockCountRequest requests the current number of blocks in this kaspad.
/// Note that this number may decrease as pruning occurs.
#[derive(Debug, Serialize, Deserialize, BorshSerialize, BorshDeserialize, BorshSchema)]
#[serde(rename_all = "camelCase")]
pub struct GetBlockCountRequest;

#[derive(Debug, Serialize, Deserialize, BorshSerialize, BorshDeserialize, BorshSchema)]
#[serde(rename_all = "camelCase")]
pub struct GetBlockCountResponse {
    pub block_count: u64,
    pub header_count: u64,
    // RpcError error = 1000;
}

/// GetBlockDagInfoRequest requests general information about the current state
/// of this kaspad's DAG.
#[derive(Debug, Serialize, Deserialize, BorshSerialize, BorshDeserialize, BorshSchema)]
#[serde(rename_all = "camelCase")]
pub struct GetBlockDagInfoRequest;

#[derive(Debug, Serialize, Deserialize, BorshSerialize, BorshDeserialize, BorshSchema)]
#[serde(rename_all = "camelCase")]
pub struct GetBlockDagInfoResponse {
    pub network_name: String, // FIXME
    pub block_count: u64,
    pub header_count: u64,
    pub tip_hashes: Vec<String>, // FIXME
    pub difficulty: f64,
    pub past_median_time: i64,
    pub virtual_parent_hashes: Vec<String>, // FIXME
    pub pruning_point_hash: String, // FIXME
    pub virtual_daa_score: u64,
    // RpcError error = 1000;
}

#[derive(Debug, Serialize, Deserialize, BorshSerialize, BorshDeserialize, BorshSchema)]
#[serde(rename_all = "camelCase")]
pub struct ResolveFinalityConflictRequest {
    pub finality_block_hash: String, // FIXME
}

#[derive(Debug, Serialize, Deserialize, BorshSerialize, BorshDeserialize, BorshSchema)]
#[serde(rename_all = "camelCase")]
pub struct ResolveFinalityConflictResponse {
    // RpcError error = 1000;
}

#[derive(Debug, Serialize, Deserialize, BorshSerialize, BorshDeserialize, BorshSchema)]
#[serde(rename_all = "camelCase")]
pub struct NotifyFinalityConflictsRequest;

#[derive(Debug, Serialize, Deserialize, BorshSerialize, BorshDeserialize, BorshSchema)]
#[serde(rename_all = "camelCase")]
pub struct NotifyFinalityConflictsResponse {
    // RpcError error = 1000;
}

#[derive(Debug, Serialize, Deserialize, BorshSerialize, BorshDeserialize, BorshSchema)]
#[serde(rename_all = "camelCase")]
pub struct FinalityConflictNotification {
    pub violating_block_hash: String, // FIXME
}

#[derive(Debug, Serialize, Deserialize, BorshSerialize, BorshDeserialize, BorshSchema)]
#[serde(rename_all = "camelCase")]
pub struct FinalityConflictResolvedNotification {
    pub finality_block_hash: String, // FIXME
}

/// ShutDownRequest shuts down this kaspad.
#[derive(Debug, Serialize, Deserialize, BorshSerialize, BorshDeserialize, BorshSchema)]
#[serde(rename_all = "camelCase")]
// pub struct ShutDownRequest;
pub struct ShutdownRequest;

#[derive(Debug, Serialize, Deserialize, BorshSerialize, BorshDeserialize, BorshSchema)]
#[serde(rename_all = "camelCase")]
// pub struct ShutDownResponse {
pub struct ShutdownResponse {
    // RpcError error = 1000;
}

/// GetHeadersRequest requests headers between the given startHash and the
/// current virtual, up to the given limit.
#[derive(Debug, Serialize, Deserialize, BorshSerialize, BorshDeserialize, BorshSchema)]
#[serde(rename_all = "camelCase")]
pub struct GetHeadersRequest {
    pub start_hash: String, // FIXME
    pub limit: u64,
    pub is_ascending: bool,
}

#[derive(Debug, Serialize, Deserialize, BorshSerialize, BorshDeserialize, BorshSchema)]
#[serde(rename_all = "camelCase")]
pub struct GetHeadersResponse {
    pub headers: Vec<String>, // FIXME
    // RpcError error = 1000;
}

/// NotifyUtxosChangedRequest registers this connection for utxoChanged notifications
/// for the given addresses.
///
/// This call is only available when this kaspad was started with `--utxoindex`
///
/// See: [`UtxosChangedNotification`]
#[derive(Debug, Serialize, Deserialize, BorshSerialize, BorshDeserialize, BorshSchema)]
#[serde(rename_all = "camelCase")]
pub struct NotifyUtxosChangedRequest  {
    /// Leave empty to get all updates
    pub addresses: Vec<Address>, // FIXME
}

#[derive(Debug, Serialize, Deserialize, BorshSerialize, BorshDeserialize, BorshSchema)]
#[serde(rename_all = "camelCase")]
pub struct NotifyUtxosChangedResponse  {
    // RpcError error = 1000;
}

/// UtxosChangedNotification is sent whenever the UTXO index had been updated.
///
/// See: [`NotifyUtxosChangedRequest`]
#[derive(Debug, Serialize, Deserialize, BorshSerialize, BorshDeserialize, BorshSchema)]
#[serde(rename_all = "camelCase")]
pub struct UtxosChangedNotification  {
    pub added: Vec<UtxosByAddressesEntry>,
    pub removed: Vec<UtxosByAddressesEntry>
}

#[derive(Debug, Serialize, Deserialize, BorshSerialize, BorshDeserialize, BorshSchema)]
#[serde(rename_all = "camelCase")]
pub struct UtxosByAddressesEntry  {
    pub address: Address, // FIXME
    pub outpoint: RpcOutpoint,
    pub utxo_entry: RpcUtxoEntry,
}

/// StopNotifyingUtxosChangedRequest unregisters this connection for utxoChanged notifications
/// for the given addresses.
///
/// This call is only available when this kaspad was started with `--utxoindex`
///
/// See: [`UtxosChangedNotification`]
#[derive(Debug, Serialize, Deserialize, BorshSerialize, BorshDeserialize, BorshSchema)]
#[serde(rename_all = "camelCase")]
pub struct StopNotifyingUtxosChangedRequest  {
    pub addresses: Vec<Address>, // FIXME
}

#[derive(Debug, Serialize, Deserialize, BorshSerialize, BorshDeserialize, BorshSchema)]
#[serde(rename_all = "camelCase")]
pub struct StopNotifyingUtxosChangedResponse  {
    // RpcError error = 1000;
}

/// GetUtxosByAddressesRequest requests all current UTXOs for the given kaspad addresses
///
/// This call is only available when this kaspad was started with `--utxoindex`
#[derive(Debug, Serialize, Deserialize, BorshSerialize, BorshDeserialize, BorshSchema)]
#[serde(rename_all = "camelCase")]
pub struct GetUtxosByAddressesRequest  {
    pub addresses: Vec<Address>, // FIXME
}

#[derive(Debug, Serialize, Deserialize, BorshSerialize, BorshDeserialize, BorshSchema)]
#[serde(rename_all = "camelCase")]
pub struct GetUtxosByAddressesResponse  {
    pub entries: Vec<UtxosByAddressesEntry>,

    // RpcError error = 1000;
}

/// GetBalanceByAddressRequest returns the total balance in unspent transactions towards a given address
/// 
/// This call is only available when this kaspad was started with `--utxoindex`
#[derive(Debug, Serialize, Deserialize, BorshSerialize, BorshDeserialize, BorshSchema)]
#[serde(rename_all = "camelCase")]
pub struct GetBalanceByAddressRequest  {
    pub address: Address, // FIXME
}

#[derive(Debug, Serialize, Deserialize, BorshSerialize, BorshDeserialize, BorshSchema)]
#[serde(rename_all = "camelCase")]
pub struct GetBalanceByAddressResponse  {
    pub balance: u64,

    // RpcError error = 1000;
}

#[derive(Debug, Serialize, Deserialize, BorshSerialize, BorshDeserialize, BorshSchema)]
#[serde(rename_all = "camelCase")]
pub struct GetBalancesByAddressesRequest  {
    pub addresses: Vec<Address>, // FIXME
}

#[derive(Debug, Serialize, Deserialize, BorshSerialize, BorshDeserialize, BorshSchema)]
#[serde(rename_all = "camelCase")]
pub struct BalancesByAddressEntry {
    pub address: Address, // FIXME
    pub balance: u64,

    // RpcError error = 1000;
}

#[derive(Debug, Serialize, Deserialize, BorshSerialize, BorshDeserialize, BorshSchema)]
#[serde(rename_all = "camelCase")]
pub struct GetBalancesByAddressesResponse  {
    pub entries: Vec<BalancesByAddressEntry>,

    // RpcError error = 1000;
}

/// GetVirtualSelectedParentBlueScoreRequest requests the blue score of the current selected parent
/// of the virtual block.
#[derive(Debug, Serialize, Deserialize, BorshSerialize, BorshDeserialize, BorshSchema)]
#[serde(rename_all = "camelCase")]
pub struct GetVirtualSelectedParentBlueScoreRequest  {
}

#[derive(Debug, Serialize, Deserialize, BorshSerialize, BorshDeserialize, BorshSchema)]
#[serde(rename_all = "camelCase")]
pub struct GetVirtualSelectedParentBlueScoreResponse  {
    pub blue_score: u64,

    // RpcError error = 1000;
}

/// NotifyVirtualSelectedParentBlueScoreChangedRequest registers this connection for
/// virtualSelectedParentBlueScoreChanged notifications.
///
/// See: [`VirtualSelectedParentBlueScoreChangedNotification`]
#[derive(Debug, Serialize, Deserialize, BorshSerialize, BorshDeserialize, BorshSchema)]
#[serde(rename_all = "camelCase")]
pub struct NotifyVirtualSelectedParentBlueScoreChangedRequest  {
}

#[derive(Debug, Serialize, Deserialize, BorshSerialize, BorshDeserialize, BorshSchema)]
#[serde(rename_all = "camelCase")]
pub struct NotifyVirtualSelectedParentBlueScoreChangedResponse  {
    // RpcError error = 1000;
}

/// VirtualSelectedParentBlueScoreChangedNotification is sent whenever the blue score
/// of the virtual's selected parent changes.
///
/// See [`NotifyVirtualSelectedParentBlueScoreChangedRequest`]
#[derive(Debug, Serialize, Deserialize, BorshSerialize, BorshDeserialize, BorshSchema)]
#[serde(rename_all = "camelCase")]
pub struct VirtualSelectedParentBlueScoreChangedNotification  {
    pub virtual_selected_parent_blue_score: u64,
}

/// NotifyVirtualDaaScoreChangedRequest registers this connection for
/// virtualDaaScoreChanged notifications.
///
/// See: [`VirtualDaaScoreChangedNotification`]
#[derive(Debug, Serialize, Deserialize, BorshSerialize, BorshDeserialize, BorshSchema)]
#[serde(rename_all = "camelCase")]
pub struct NotifyVirtualDaaScoreChangedRequest  {
}

#[derive(Debug, Serialize, Deserialize, BorshSerialize, BorshDeserialize, BorshSchema)]
#[serde(rename_all = "camelCase")]
pub struct NotifyVirtualDaaScoreChangedResponse  {
    // RpcError error = 1000;
}

/// VirtualDaaScoreChangedNotification is sent whenever the DAA score
/// of the virtual changes.
///
/// See [`NotifyVirtualDaaScoreChangedRequest`]
#[derive(Debug, Serialize, Deserialize, BorshSerialize, BorshDeserialize, BorshSchema)]
#[serde(rename_all = "camelCase")]
pub struct VirtualDaaScoreChangedNotification  {
    pub virtual_daa_score: u64,
}

/// NotifyPruningPointUTXOSetOverrideRequest registers this connection for
/// pruning point UTXO set override notifications.
///
/// This call is only available when this kaspad was started with `--utxoindex`
///
/// See: [`NotifyPruningPointUTXOSetOverrideResponse`]
#[derive(Debug, Serialize, Deserialize, BorshSerialize, BorshDeserialize, BorshSchema)]
#[serde(rename_all = "camelCase")]
pub struct NotifyPruningPointUTXOSetOverrideRequest  {
}


#[derive(Debug, Serialize, Deserialize, BorshSerialize, BorshDeserialize, BorshSchema)]
#[serde(rename_all = "camelCase")]
pub struct NotifyPruningPointUTXOSetOverrideResponse  {
    // RpcError error = 1000;
}

/// PruningPointUTXOSetOverrideNotification is sent whenever the UTXO index
/// resets due to pruning point change via IBD.
///
/// See [`NotifyPruningPointUTXOSetOverrideRequest`]
#[derive(Debug, Serialize, Deserialize, BorshSerialize, BorshDeserialize, BorshSchema)]
#[serde(rename_all = "camelCase")]
pub struct PruningPointUTXOSetOverrideNotification  {
}

/// StopNotifyingPruningPointUTXOSetOverrideRequest unregisters this connection for
/// pruning point UTXO set override notifications.
///
/// This call is only available when this kaspad was started with `--utxoindex`
///
/// See: [`PruningPointUTXOSetOverrideNotification`]
#[derive(Debug, Serialize, Deserialize, BorshSerialize, BorshDeserialize, BorshSchema)]
#[serde(rename_all = "camelCase")]
pub struct StopNotifyingPruningPointUTXOSetOverrideRequest  {
}

#[derive(Debug, Serialize, Deserialize, BorshSerialize, BorshDeserialize, BorshSchema)]
#[serde(rename_all = "camelCase")]
pub struct StopNotifyingPruningPointUTXOSetOverrideResponse  {
    // RpcError error = 1000;
}

/// BanRequest bans the given ip.
#[derive(Debug, Serialize, Deserialize, BorshSerialize, BorshDeserialize, BorshSchema)]
#[serde(rename_all = "camelCase")]
pub struct BanRequest {
    ip: String, // FIXME
}

#[derive(Debug, Serialize, Deserialize, BorshSerialize, BorshDeserialize, BorshSchema)]
#[serde(rename_all = "camelCase")]
pub struct BanResponse {
    // RpcError error = 1000;
}

/// UnbanRequest unbans the given ip.
#[derive(Debug, Serialize, Deserialize, BorshSerialize, BorshDeserialize, BorshSchema)]
#[serde(rename_all = "camelCase")]
pub struct UnbanRequest {
    pub ip: String, // FIXME
}

#[derive(Debug, Serialize, Deserialize, BorshSerialize, BorshDeserialize, BorshSchema)]
#[serde(rename_all = "camelCase")]
pub struct UnbanResponse {
    // RpcError error = 1000;
}

/// GetInfoRequest returns info about the node.
#[derive(Debug, Serialize, Deserialize, BorshSerialize, BorshDeserialize, BorshSchema)]
#[serde(rename_all = "camelCase")]
pub struct GetInfoRequest {
}

#[derive(Debug, Serialize, Deserialize, BorshSerialize, BorshDeserialize, BorshSchema)]
#[serde(rename_all = "camelCase")]
pub struct GetInfoResponse {
    pub p2p_id: String,
    pub mempool_size: u64,
    pub server_version: String, // FIXME ?
    pub is_utxo_indexed: bool,
    pub is_synced: bool,
    // RpcError error = 1000;
}

#[derive(Debug, Serialize, Deserialize, BorshSerialize, BorshDeserialize, BorshSchema)]
#[serde(rename_all = "camelCase")]
pub struct EstimateNetworkHashesPerSecondRequest {
    pub window_size: u32,
    pub start_hash: String, // FIXME
}

#[derive(Debug, Serialize, Deserialize, BorshSerialize, BorshDeserialize, BorshSchema)]
#[serde(rename_all = "camelCase")]
pub struct EstimateNetworkHashesPerSecondResponse {
    pub network_hashes_per_second: u64,
    // RpcError error = 1000;
}

/// NotifyNewBlockTemplateRequest registers this connection for
/// NewBlockTemplate notifications.
///
/// See: [`NewBlockTemplateNotification`]
#[derive(Debug, Serialize, Deserialize, BorshSerialize, BorshDeserialize, BorshSchema)]
#[serde(rename_all = "camelCase")]
pub struct NotifyNewBlockTemplateRequest  {
}

#[derive(Debug, Serialize, Deserialize, BorshSerialize, BorshDeserialize, BorshSchema)]
#[serde(rename_all = "camelCase")]
pub struct NotifyNewBlockTemplateResponse  {
    // RpcError error = 1000;
}

/// NewBlockTemplateNotification is sent whenever a new updated block template is
/// available for miners.
///
/// See [`NotifyNewBlockTemplateRequest`]
#[derive(Debug, Serialize, Deserialize, BorshSerialize, BorshDeserialize, BorshSchema)]
#[serde(rename_all = "camelCase")]
pub struct NewBlockTemplateNotification  {
}

#[derive(Debug, Serialize, Deserialize, BorshSerialize, BorshDeserialize, BorshSchema)]
#[serde(rename_all = "camelCase")]
pub struct MempoolEntryByAddress {
    pub address: String, // FIXME
    pub sending: Vec<MempoolEntry>,
    pub receiving: Vec<MempoolEntry>,
}

#[derive(Debug, Serialize, Deserialize, BorshSerialize, BorshDeserialize, BorshSchema)]
#[serde(rename_all = "camelCase")]
pub struct GetMempoolEntriesByAddressesRequest {
    pub addresses: Vec<String>, // FIXME
    pub include_orphan_pool: bool,
    pub filter_transaction_pool: bool,
}

#[derive(Debug, Serialize, Deserialize, BorshSerialize, BorshDeserialize, BorshSchema)]
#[serde(rename_all = "camelCase")]
pub struct  GetMempoolEntriesByAddressesResponse {
    pub entries : Vec<MempoolEntryByAddress>,

    // RpcError error = 1000;
}

#[derive(Debug, Serialize, Deserialize, BorshSerialize, BorshDeserialize, BorshSchema)]
#[serde(rename_all = "camelCase")]
pub struct GetCoinSupplyRequest {
}

#[derive(Debug, Serialize, Deserialize, BorshSerialize, BorshDeserialize, BorshSchema)]
#[serde(rename_all = "camelCase")]
pub struct GetCoinSupplyResponse {
    /// note: this is a hard coded maxSupply, actual maxSupply is expected to deviate by upto -5%, but cannot be measured exactly.
    pub max_sompi: u64,
    pub circulating_sompi: u64,

    // RpcError error = 1000;
}