use lunamodel_core::Solution;
use lunamodel_serializer::prelude::*;

fn main() {
    let bytes = Solution::default()
        .encode(Some(false), None)
        .expect("default solution should encode");

    for byte in bytes {
        print!("{byte:02x}");
    }
    println!();
}
