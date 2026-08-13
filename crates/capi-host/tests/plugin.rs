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

use lunamodel_capi::plugin::{LmByteSink, LmPassContext, LmPassKind, LmPluginTransform, LmStrSink};
use lunamodel_capi_host::{
    LmBuffer, LmModel, LmSol, LmStatus,
    error::{
        lm_error_cause, lm_error_free, lm_error_kind, lm_error_message, lm_last_error,
        lm_last_error_message, lm_set_last_error,
    },
    model::lm_model_set_name,
    plugin::{CPluginTransformPass, lm_register_plugin_backward},
};
use lunamodel_core::{Model, Solution};
use lunamodel_error::LunaModelError;
use lunamodel_transpiler::PassManager;

/// Reads back an `LmBuffer`'s bytes as a `String` and frees the buffer.
unsafe fn take_buffer_string(buf: LmBuffer) -> String {
    let bytes = if buf.len == 0 {
        Vec::new()
    } else {
        unsafe { std::slice::from_raw_parts(buf.data, buf.len) }.to_vec()
    };
    unsafe { lunamodel_capi_host::buffer::lm_buffer_free(buf) };
    String::from_utf8(bytes).expect("utf8 error text")
}

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
        requires_analyses: requires,
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

const FAILING_PLUGIN_ID: &CStr = c"test::failing";
const FAILING_KIND: &CStr = c"test::domain_specific_failure";
const FAILING_MESSAGE: &CStr = c"the model has no variable named 'q'";

unsafe extern "C" fn failing_id(_ud: *mut c_void) -> *const c_char {
    FAILING_PLUGIN_ID.as_ptr()
}

unsafe extern "C" fn failing_forward(
    _ud: *mut c_void,
    _model: *mut LmModel,
    _ctx: *const LmPassContext,
    _sink: LmByteSink,
    _sink_ctx: *mut c_void,
) -> LmStatus {
    // Exactly what a plugin SDK trampoline does on the author's `Err`: report the real error
    // before returning the status code, instead of letting it get silently discarded.
    unsafe {
        lm_set_last_error(
            FAILING_KIND.as_ptr(),
            FAILING_MESSAGE.as_ptr(),
            ptr::null_mut(),
        )
    };
    LmStatus::Errored
}

unsafe extern "C" fn failing_backward(
    _ud: *mut c_void,
    _artifact: *const u8,
    _len: usize,
    solution: *mut LmSol,
    out_solution: *mut *mut LmSol,
) -> LmStatus {
    unsafe { *out_solution = solution };
    LmStatus::Ok
}

fn failing_descriptor() -> LmPluginTransform {
    LmPluginTransform {
        user_data: ptr::null_mut(),
        id: failing_id,
        name,
        requires_analyses: requires,
        invalidates,
        forward: failing_forward,
        free,
    }
}

/// A plugin pass that fails must have its own message (and kind) reach the caller, not just a
/// generic "plugin pass returned Errored" — this is the bug the plugin trampoline / host adapter
/// error plumbing exists to fix.
#[test]
fn plugin_forward_failure_message_reaches_the_caller() {
    let status = unsafe {
        lm_register_plugin_backward(
            LmPassKind::Transformation,
            FAILING_PLUGIN_ID.as_ptr(),
            failing_backward,
            ptr::null_mut(),
        )
    };
    assert_eq!(status, LmStatus::Ok);

    let pm = PassManager::default()
        .add_transform(CPluginTransformPass::from_descriptor(&failing_descriptor()));
    let err = pm
        .run(Model::default())
        .expect_err("the failing plugin pass should fail the run");

    let rendered = err.to_string();
    assert!(
        rendered.contains(FAILING_MESSAGE.to_str().unwrap()),
        "expected the plugin's own error message in {rendered:?}"
    );

    // The plugin's own `kind` must survive being folded into a `LunaModelError`, not collapse to
    // the generic "transformation"/"external" tag the wrapping machinery would otherwise report.
    let lme: LunaModelError = err.into();
    assert_eq!(
        lme.kind_tag(),
        FAILING_KIND.to_str().unwrap(),
        "expected the plugin's own kind tag to survive, got {:?} in {lme}",
        lme.kind_tag()
    );

    // The message must not be duplicated by folding the error's own chain onto itself (the outer
    // frame's message already covers the top of the chain).
    let rendered = lme.to_string();
    let occurrences = rendered.matches(FAILING_MESSAGE.to_str().unwrap()).count();
    assert_eq!(
        occurrences, 1,
        "expected the plugin's message to appear exactly once in {rendered:?}"
    );
}

/// Drives the raw `lm_error_*` C accessors directly against a `RichError` reported by
/// `lm_set_last_error`, exercising the ABI surface no existing test touched: reading kind,
/// message, and cause back off an owned error handle, and freeing it.
#[test]
fn error_accessors_read_back_kind_message_and_cause() {
    let cause_kind = c"test::root_cause";
    let cause_message = c"underlying failure";
    let top_kind = c"test::wrapping_failure";
    let top_message = c"wrapping failure";

    let status =
        unsafe { lm_set_last_error(cause_kind.as_ptr(), cause_message.as_ptr(), ptr::null_mut()) };
    assert_eq!(status, LmStatus::Ok);
    let cause = lm_last_error();
    assert!(!cause.is_null());

    let status = unsafe { lm_set_last_error(top_kind.as_ptr(), top_message.as_ptr(), cause) };
    assert_eq!(status, LmStatus::Ok);

    let top = lm_last_error();
    assert!(!top.is_null());

    let mut buf = LmBuffer::default();
    assert_eq!(unsafe { lm_error_kind(top, &mut buf) }, LmStatus::Ok);
    assert_eq!(
        unsafe { take_buffer_string(buf) },
        top_kind.to_str().unwrap()
    );

    let mut buf = LmBuffer::default();
    assert_eq!(unsafe { lm_error_message(top, &mut buf) }, LmStatus::Ok);
    assert_eq!(
        unsafe { take_buffer_string(buf) },
        top_message.to_str().unwrap()
    );

    let mut cause_out = ptr::null_mut();
    assert_eq!(unsafe { lm_error_cause(top, &mut cause_out) }, LmStatus::Ok);
    assert!(!cause_out.is_null());

    let mut buf = LmBuffer::default();
    assert_eq!(unsafe { lm_error_kind(cause_out, &mut buf) }, LmStatus::Ok);
    assert_eq!(
        unsafe { take_buffer_string(buf) },
        cause_kind.to_str().unwrap()
    );

    unsafe {
        lm_error_free(cause_out);
        lm_error_free(top);
    }
}

/// `lm_error_kind`/`lm_error_message`/`lm_error_cause` are read-only accessors on an
/// already-owned handle; they must not clobber `lm_last_error`/`lm_last_error_message` as a side
/// effect of merely inspecting that handle.
#[test]
fn error_accessors_do_not_clobber_last_error_state() {
    let kind = c"test::inspected";
    let message = c"an error being inspected";

    let status = unsafe { lm_set_last_error(kind.as_ptr(), message.as_ptr(), ptr::null_mut()) };
    assert_eq!(status, LmStatus::Ok);

    let err = lm_last_error();
    assert!(!err.is_null());

    let mut buf = LmBuffer::default();
    assert_eq!(unsafe { lm_error_kind(err, &mut buf) }, LmStatus::Ok);
    unsafe { take_buffer_string(buf) };

    let message_ptr = lm_last_error_message();
    assert!(
        !message_ptr.is_null(),
        "lm_error_kind must not clear the thread-local last-error message"
    );
    let still_there = unsafe { std::ffi::CStr::from_ptr(message_ptr) }
        .to_str()
        .unwrap();
    assert_eq!(still_there, message.to_str().unwrap());

    unsafe { lm_error_free(err) };
}
