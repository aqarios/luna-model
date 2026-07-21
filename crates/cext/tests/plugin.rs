//! In-process test of the plugin transformation mechanism.
//!
//! Drives the C-ABI plugin boundary exactly as an external plugin would: a set of `extern "C"`
//! callbacks is packaged into an `LmPluginTransform`, wrapped in the host adapter, and run through
//! a real `PassManager`. Verifies that `forward` mutates the model through the live handle, and
//! that reversal dispatches back into the plugin's `backward` with the artifact bytes intact.

use std::{
    ffi::{CStr, c_char, c_void},
    ptr,
    sync::atomic::{AtomicBool, AtomicUsize, Ordering},
};

use luna_model::core::{Model, Solution};
use lunamodel_transpiler::PassManager;
use lunamodel_cext::plugin::CPluginTransformPass;
use lunamodel_cext::{
    LmModel, LmSol, LmStatus, model::lm_model_set_name, plugin::lm_register_plugin_backward,
};
use lunamodel_cext_vtable::transpiler::{
    LmByteSink, LmPassContext, LmPassKind, LmPluginTransform, LmStrSink,
};

const PLUGIN_ID: &CStr = c"test::rename";
const NEW_NAME: &CStr = c"renamed-by-plugin";
const PAYLOAD: &[u8] = b"REVERSE";

static BACKWARD_CALLS: AtomicUsize = AtomicUsize::new(0);
static BACKWARD_PAYLOAD_OK: AtomicBool = AtomicBool::new(false);

unsafe extern "C" fn id(_ud: *mut c_void) -> *const c_char {
    PLUGIN_ID.as_ptr()
}

unsafe extern "C" fn name(_ud: *mut c_void) -> *const c_char {
    c"rename".as_ptr()
}
unsafe extern "C" fn requires(_ud: *mut c_void, _sink: LmStrSink, _ctx: *mut c_void) {}
unsafe extern "C" fn invalidates(_ud: *mut c_void, _sink: LmStrSink, _ctx: *mut c_void) {}

unsafe extern "C" fn forward(
    _ud: *mut c_void,
    model: *mut LmModel,
    _ctx: *const LmPassContext,
    sink: LmByteSink,
    sink_ctx: *mut c_void,
) -> LmStatus {
    // Mutate the model through the live handle, proving the C boundary works.
    let status = unsafe { lm_model_set_name(model, NEW_NAME.as_ptr()) };
    if status != LmStatus::Ok {
        return status;
    }
    // Emit the inverse artifact bytes.
    unsafe { sink(sink_ctx, PAYLOAD.as_ptr(), PAYLOAD.len()) };
    LmStatus::Ok
}

unsafe extern "C" fn backward(
    _ud: *mut c_void,
    artifact: *const u8,
    len: usize,
    solution: *mut LmSol,
    out_solution: *mut *mut LmSol,
) -> LmStatus {
    let bytes = if len == 0 {
        &[][..]
    } else {
        unsafe { std::slice::from_raw_parts(artifact, len) }
    };
    BACKWARD_CALLS.fetch_add(1, Ordering::SeqCst);
    BACKWARD_PAYLOAD_OK.store(bytes == PAYLOAD, Ordering::SeqCst);
    // Pass ownership of the incoming solution straight through (no-op reversal).
    unsafe { *out_solution = solution };
    LmStatus::Ok
}

unsafe extern "C" fn free(_ud: *mut c_void) {}

fn descriptor() -> LmPluginTransform {
    LmPluginTransform {
        user_data: ptr::null_mut(),
        id,
        name,
        requires,
        invalidates,
        forward,
        free,
    }
}

#[test]
fn plugin_transform_forward_and_backward() {
    let status = unsafe {
        lm_register_plugin_backward(
            LmPassKind::Transformation,
            PLUGIN_ID.as_ptr(),
            backward,
            ptr::null_mut(),
        )
    };
    assert_eq!(status, LmStatus::Ok);

    let pm =
        PassManager::default().add_transform(CPluginTransformPass::from_descriptor(&descriptor()));
    let out = pm.run(Model::default()).expect("pass manager run");

    assert_eq!(out.model.name, NEW_NAME.to_str().unwrap());

    let _solution = out.record.backward(Solution::default()).expect("backward");
    assert!(
        BACKWARD_PAYLOAD_OK.load(Ordering::SeqCst),
        "backward received the artifact bytes emitted by forward"
    );
}
