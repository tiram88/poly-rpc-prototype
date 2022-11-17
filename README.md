# Poly RPC Prototype

Essentially a proof of concept of a multi-protocol RPC sub-system for [rusty-kaspa](https://github.com/kaspanet/rusty-kaspa).

This demo features:

- Trait `RpcApi` (initial work of [Aspectron](https://github.com/aspectron/rusty-kaspa/blob/ffa8dd95264e68580acabb6cccb6a37dfd39c30a/rpc/core/src/client/interface.rs)) exposing a protocol-agnostic API
- rpc-core `RpcApi` server responsible for fetching data from consensus
- rpc-grpc `RpcApi` service and client implementing (partially) the `protowire` RPC from kaspad
- Notification system with following pipeline: consensus -> rpc-core server -> rpc-grpc service -> rpc-grpc client -> client code

## Limitations

The gRPC implementation at this stage should be considered essentially a proof of concept. It is limited to 2 queries: `get_block` and `get_info`.

The same is true of the notification system that only implements `BlockAdded`.

The whole rpc sub-system is not actually connected to consensus for now, neither for queries nor for notifications.

The client code requires an actual go kaspa node in order to demonstrate inter-operability and backwards compatibility.

## Playing the demo

To run the prototype:

- Open 2 terminals and run respectively following commands:
  
   1. `cargo run --bin server`
   2. `cargo run --bin client -- -a http://<kaspad_ip>:<rpc_port>` (ie. -a http://192.168.1.233:16110)
