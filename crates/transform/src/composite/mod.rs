//! Built-in composite passes.

/// Registers backward handlers for built-in composite passes.
///
/// The crate currently does not ship any composite passes with reversible
/// artifacts, so this function is intentionally a no-op placeholder.
pub fn register_backward() {
    // Register the backwards function here. P are the CompositePass implementations.
    // lunamodel_transpiler::register_backward::<P>();
}
