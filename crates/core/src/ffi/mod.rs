// use std::sync::Arc;
//
// use parking_lot::RwLock;
//
// use crate::{ArcEnv, Environment};
//
// #[repr(transparent)]
// pub struct ArcEnvRawPtr(pub usize);
//
// impl Into<usize> for ArcEnvRawPtr {
//     fn into(self) -> usize {
//         self.0
//     }
// }
//
// impl From<usize> for ArcEnvRawPtr {
//     fn from(value: usize) -> Self {
//         ArcEnvRawPtr(value)
//     }
// }
//
// impl ArcEnv {
//     pub fn into_raw_ptr(&self) -> ArcEnvRawPtr {
//         let cloned = Arc::clone(&self.env);
//         let ptr = Arc::into_raw(cloned) as usize;
//         ArcEnvRawPtr(ptr)
//     }
//
//     /// Safety: Only callable on passable type with fixed representation.
//     pub fn from_raw_ptr(raw_ptr: ArcEnvRawPtr) -> ArcEnv {
//         let arc = unsafe { Arc::from_raw(raw_ptr.0 as *const RwLock<Environment>) };
//         ArcEnv { env: arc }
//     }
// }
