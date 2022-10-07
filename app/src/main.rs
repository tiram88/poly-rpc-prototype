use std::str::FromStr;
use hashes::Hash;

fn main() {
    let h: Hash = 1.into();
    let h_bytes = h.as_bytes();
    let h_str = h.to_string();
    println!("Hash = {}", h_str);
    println!("Hash bytes = {:?}", h_bytes);
    let h2: Hash = Hash::from_str(&h_str).unwrap();
    assert_eq!(h, h2);

    let sa = ShapeA{ id: 1 };
    let sb = ShapeB::from(&sa);
    println!("A: {:?}, B: {:?}", sa, sb);

    let sa2 = ShapeA::from(&sb);
    assert_eq!(sa.id, sa2.id);

    assert_eq!(sa.id.to_string(), sb.id);
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
        ShapeB {
            id: item.id.to_string(),
        }
    }
}

impl From<&ShapeB> for ShapeA {
    fn from(item: &ShapeB) -> ShapeA {
        ShapeA {
            id: item.id.parse().unwrap_or(0),
        }
    }
}