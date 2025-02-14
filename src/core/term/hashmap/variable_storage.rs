use hashbrown::HashMap;
// use nohash::IntMap;
use crate::core::operations::Key;
// use std::hash::{BuildHasher, Hasher};
// use std::collections::BTreeMap;
// use std::collections::HashMap;
// use rustc_hash::FxHashMap;
//
// Default
// pub type Variables<K> = HashMap<K, f64>;

// Using FxHash
// pub type Variables<K> = FxHashMap<K, f64>;

// Using Hashbrown
pub type Variables<K> = HashMap<K, f64>;

// Using NoHash
// pub type Variables<K> = IntMap<K, f64>;

// Using Binary Tree (SLOW)
// pub type Variables<K> = BTreeMap<K, f64>;
//

// #[derive(Default)]
// pub struct NoHashHasher(u64);
//
// impl Hasher for NoHashHasher {
//     #[inline]
//     fn finish(&self) -> u64 {
//         self.0
//     }
//
//     #[inline]
//     fn write(&mut self, _bytes: &[u8]) {
//         // For a real implementation, you'd want to handle this case,
//         // but for our u64-only use case we can ignore it
//         unimplemented!("NoHashHasher only works with u64 keys")
//     }
//
//     #[inline]
//     fn write_u32(&mut self, i: u32) {
//         self.0 = i as u64;
//     }
//
//     #[inline]
//     fn write_u64(&mut self, i: u64) {
//         self.0 = i;
//     }
// }
//
// /// Builder for NoHashHasher
// #[derive(Clone, Default)]
// pub struct BuildNoHashHasher;
//
// impl BuildHasher for BuildNoHashHasher {
//     type Hasher = NoHashHasher;
//
//     #[inline]
//     fn build_hasher(&self) -> NoHashHasher {
//         NoHashHasher::default()
//     }
// }
//
// // Type alias for a HashMap using NoHashHasher
// pub type NoHashMap<K> = HashMap<K, f64, BuildNoHashHasher>;
//
#[inline]
pub fn variables_with_capacity<K: Key>(capacity: usize) -> Variables<K> {
    // HashMap::with_capacity_and_hasher(capacity, BuildNoHashHasher)
    Variables::with_capacity(capacity)
    // Variables::with_capacity_and_hasher(capacity, NoHashHasher)
    // let mut out = Variables::default();
    // out.reserve(capacity);
    // out
    // Variables::default()
}
//
// // pub type Variables<K> = HashMap<K, f64>;
// pub type Variables<K> = NoHashMap<K>;
