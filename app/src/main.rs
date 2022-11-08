use hashes::Hash;
use rpc_grpc::protowire;
use std::str::FromStr;

fn main() {
    test_hashes();
    test_from();
    test_blue_score();
    test_rpc_block_level_parents();
}

fn test_hashes() {
    let h: Hash = 1.into();
    let h_bytes = h.as_bytes();
    let h_str = h.to_string();
    println!("Hash = {}", h_str);
    println!("Hash bytes = {:?}", h_bytes);
    let h2: Hash = Hash::from_str(&h_str).unwrap();
    assert_eq!(h, h2);

    let r = RpcHash::from_str("too short");
    println!("Failing rpc hash parsing: {:?}", r);

    let r = RpcHash::from_str("wrong character 0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF");
    println!("Failing rpc hash parsing: {:?}", r);

    let r = RpcHash::from_str("0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF");
    println!("Successfull rpc hash parsing: {:?}", r);
}

fn test_from() {
    let sa = ShapeA { id: 1 };
    let sb = ShapeB::from(&sa);
    println!("A: {:?}, B: {:?}", sa, sb);

    let sa2 = ShapeA::from(&sb);
    assert_eq!(sa.id, sa2.id);

    assert_eq!(sa.id.to_string(), sb.id);
}

use rpc_core::{RpcBlueWorkType, RpcHash};
fn test_blue_score() {
    let b1 = RpcBlueWorkType::from(12345);
    let b2: RpcBlueWorkType = "12345".try_into().unwrap();
    assert_eq!(b1, b2);

    let b3 = RpcBlueWorkType::try_from("not a number");
    println!("rpc blue score parse error (TryFrom) {:?}", b3);

    let b4 = RpcBlueWorkType::from_str("not a number");
    println!("rpc blue score parse error (FromStr) {:?}", b4);

    let b5 = RpcBlueWorkType::from_str("1234567890123456789012345678901234567890");
    println!("rpc blue score parse error (FromStr), overflow {:?}", b5);
}

fn test_rpc_block_level_parents() {
    let a_core = rpc_core::RpcBlockLevelParents { parent_hashes: vec![Hash::from(1), Hash::from(123456789)] };
    let a_grpc: protowire::RpcBlockLevelParents = (&a_core).try_into().unwrap();
    println!("A core RpcBlockLevelParents {:?}", a_core);
    println!("A gRPC RpcBlockLevelParents {:?}", a_grpc);

    let b_grpc = protowire::RpcBlockLevelParents {
        parent_hashes: vec!["0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF".to_string(), "wrong hash".to_string()],
    };
    let b_core_result: rpc_core::RpcResult<rpc_core::RpcBlockLevelParents> = (&b_grpc).try_into();
    println!("B gRPC RpcBlockLevelParents {:?}", b_grpc);
    println!("B core RpcBlockLevelParents result {:?}", b_core_result);
}

#[derive(Debug)]
struct ShapeA {
    id: u64,
}

#[derive(Debug)]
struct ShapeB {
    id: String,
}

impl From<&ShapeA> for ShapeB {
    fn from(item: &ShapeA) -> ShapeB {
        ShapeB { id: item.id.to_string() }
    }
}

impl From<&ShapeB> for ShapeA {
    fn from(item: &ShapeB) -> ShapeA {
        ShapeA { id: item.id.parse().unwrap_or(0) }
    }
}
