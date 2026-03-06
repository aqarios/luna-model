use lunamodel_error::py::PyInternalPanicError;

use pyo3::PyResult;
use std::panic::{self, AssertUnwindSafe, PanicHookInfo};
use std::{backtrace::Backtrace, cell::RefCell};

thread_local! {
    static LAST_PANIC_BT: RefCell<Option<String>> = RefCell::new(None);
}

pub fn unwind<T, F>(f: F) -> PyResult<T>
where
    F: FnOnce() -> PyResult<T>,
{
    // Save the currently installed hook (maybe custom elsewhere).
    let prev_hook = panic::take_hook();

    // Grab the default hook so we can forward to it (prints + honors RUST_BACKTRACE).
    let default_hook = panic::take_hook();

    // Install our hook: call the default hook (so it prints as usual) and also capture a backtrace.
    panic::set_hook(Box::new(move |info: &PanicHookInfo<'_>| {
        // Keep the standard panic output
        default_hook(info);

        // And stash a textual backtrace for this thread
        let bt = Backtrace::force_capture();
        LAST_PANIC_BT.with(|slot| {
            *slot.borrow_mut() = Some(format!("{info}\n{bt}"));
        });
    }));

    let result = panic::catch_unwind(AssertUnwindSafe(f));

    // Restore the original hook.
    panic::set_hook(prev_hook);

    match result {
        Ok(v) => v,
        Err(payload) => {
            // Build the panic message
            // let mut msg = if let Some(s) = payload.downcast_ref::<&str>() {
            let msg = if let Some(s) = payload.downcast_ref::<&str>() {
                s.to_string()
            } else if let Some(s) = payload.downcast_ref::<String>() {
                s.clone()
            } else {
                "rust panic occurred".to_string()
            };

            // // Append captured backtrace, if any
            // if let Some(bt) = LAST_PANIC_BT.with(|slot| slot.borrow_mut().take()) {
            //     msg.push_str("\n\n--- Rust backtrace ---\n");
            //     msg.push_str(&bt);
            // }

            Err(PyInternalPanicError::new_err(format!(
                "internal panic: {msg}"
            )))
        }
    }
}
