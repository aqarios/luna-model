//! Integration tests that drive the C ABI exactly as a C consumer would.

use std::ffi::CStr;
use std::ptr;

use lunamodel_capi::CAPI_ABI_VERSION;
use lunamodel_capi::buffer::{LmBuffer, lm_buffer_free};
use lunamodel_capi::error::{LmStatus, lm_last_error_message};
use lunamodel_capi::model::{
    LmModel, lm_model_deserialize, lm_model_free, lm_model_new, lm_model_serialize,
};
use lunamodel_capi::solution::{LmSolution, lm_solution_deserialize, lm_solution_free};
use lunamodel_capi::version::{lm_abi_version, lm_version};

use luna_model::core::{Model, Solution};
use luna_model::serializer::prelude::*;

/// Copies an `LmBuffer`'s bytes into a `Vec`, then frees the buffer.
///
/// # Safety
/// `buf` must be a valid buffer returned by the C API.
unsafe fn take_buffer(buf: LmBuffer) -> Vec<u8> {
    let bytes = if buf.data.is_null() {
        Vec::new()
    } else {
        unsafe { std::slice::from_raw_parts(buf.data, buf.len) }.to_vec()
    };
    unsafe { lm_buffer_free(buf) };
    bytes
}

fn empty_buffer() -> LmBuffer {
    LmBuffer {
        data: ptr::null_mut(),
        len: 0,
        cap: 0,
    }
}

#[test]
fn version_and_abi() {
    let version = unsafe { CStr::from_ptr(lm_version()) };
    assert_eq!(version.to_str().unwrap(), env!("CARGO_PKG_VERSION"));
    assert_eq!(lm_abi_version(), CAPI_ABI_VERSION);
}

#[test]
fn model_serialize_deserialize_roundtrip() {
    let model = lm_model_new();
    assert!(!model.is_null());

    let mut buf = empty_buffer();
    assert_eq!(unsafe { lm_model_serialize(model, &mut buf) }, LmStatus::Ok);
    let bytes = unsafe { take_buffer(buf) };
    assert!(!bytes.is_empty());

    let mut restored: *mut LmModel = ptr::null_mut();
    let status = unsafe { lm_model_deserialize(bytes.as_ptr(), bytes.len(), &mut restored) };
    assert_eq!(status, LmStatus::Ok);
    assert!(!restored.is_null());

    // The restored model must itself re-serialize cleanly.
    let mut buf2 = empty_buffer();
    assert_eq!(
        unsafe { lm_model_serialize(restored, &mut buf2) },
        LmStatus::Ok
    );
    let bytes2 = unsafe { take_buffer(buf2) };
    assert!(!bytes2.is_empty());

    unsafe { lm_model_free(model) };
    unsafe { lm_model_free(restored) };
}

#[test]
fn model_deserialize_accepts_core_encoded_bytes() {
    // Bytes from the core serializer must load through the C API, proving format
    // compatibility with the Rust core and the Python wheel.
    let bytes = Model::default().encode(Some(true), None).unwrap();

    let mut model: *mut LmModel = ptr::null_mut();
    let status = unsafe { lm_model_deserialize(bytes.as_ptr(), bytes.len(), &mut model) };
    assert_eq!(status, LmStatus::Ok);
    assert!(!model.is_null());
    unsafe { lm_model_free(model) };
}

#[test]
fn null_pointer_is_reported() {
    let mut buf = empty_buffer();
    let status = unsafe { lm_model_serialize(ptr::null(), &mut buf) };
    assert_eq!(status, LmStatus::NullPointer);

    let msg = unsafe { CStr::from_ptr(lm_last_error_message()) };
    assert!(msg.to_str().unwrap().contains("model"));
}

#[test]
fn garbage_bytes_fail_without_unwinding() {
    let garbage = [1u8, 2, 3, 4, 5];
    let mut model: *mut LmModel = ptr::null_mut();
    let status = unsafe { lm_model_deserialize(garbage.as_ptr(), garbage.len(), &mut model) };
    // Either a clean decode error or a caught panic — never a crash, never Ok.
    assert_ne!(status, LmStatus::Ok);
    assert!(model.is_null());
    assert!(!lm_last_error_message().is_null());
}

#[test]
fn solution_roundtrips_from_core_bytes() {
    let bytes = Solution::default().encode(Some(true), None).unwrap();

    let mut solution: *mut LmSolution = ptr::null_mut();
    let status = unsafe { lm_solution_deserialize(bytes.as_ptr(), bytes.len(), &mut solution) };
    assert_eq!(status, LmStatus::Ok);
    assert!(!solution.is_null());
    unsafe { lm_solution_free(solution) };
}
