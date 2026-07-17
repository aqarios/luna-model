//! Process-global table of plugin reversal handlers.
//!
//! Plugin transformation passes all share one [`Reversible::ID`], so their backward dispatch is
//! routed by the plugin's own stable pass id. `luna_model.register(...)` (via
//! `lm_register_plugin_backward`) populates this table. It is consulted by
//! [`super::pass::CPluginTransformPass`]'s `backward`, possibly long after the pass instance is
//! gone or after a record was reloaded, so each handler carries its own module-level `user_data`
//! rather than any per-instance state.

use std::{
    collections::HashMap,
    ffi::c_void,
    sync::{Mutex, OnceLock},
};

use lunamodel_cext_vtable::transpiler::LmBackwardFn;

struct PluginBackward {
    func: LmBackwardFn,
    user_data: *mut c_void,
}

// Safety: `func`/`user_data` are opaque to luna-model. The plugin is responsible for their
// thread-safety. luna-model only calls them during reversal.
unsafe impl Send for PluginBackward {}
unsafe impl Sync for PluginBackward {}

static TRANSFORM_BACKWARDS: OnceLock<Mutex<HashMap<String, PluginBackward>>> = OnceLock::new();

fn table() -> &'static Mutex<HashMap<String, PluginBackward>> {
    TRANSFORM_BACKWARDS.get_or_init(|| Mutex::new(HashMap::new()))
}

/// Registers (or replaces) the reversal handler for a transformation plugin `id`.
pub fn register_transform(id: String, func: LmBackwardFn, user_data: *mut c_void) {
    table()
        .lock()
        .unwrap()
        .insert(id, PluginBackward { func, user_data });
}

/// Looks up a transformation reversal handler by plugin id.
pub(crate) fn lookup_transform(id: &str) -> Option<(LmBackwardFn, *mut c_void)> {
    table()
        .lock()
        .unwrap()
        .get(id)
        .map(|b| (b.func, b.user_data))
}
