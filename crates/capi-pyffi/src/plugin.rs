use std::ffi::CStr;

use lunamodel_capi::plugin::{
    LmPluginAnalysis, LmPluginComposite, LmPluginControlFlow, LmPluginMetaAnalysis,
    LmPluginTransform,
};
use pyo3::{
    Bound, PyResult, Python,
    types::{PyCapsule, PyCapsuleMethods},
};

/// Capusle name carrying a plugin transformation descriptor from a plugin to the host pass manager.
pub const PLUGIN_TRANSFORM_CAPSULE_NAME: &CStr = c"luna_model.capi.PluginTransform.v1";

/// Method name a plugin pass object must expose so the host can obtain its transform descriptor.
///
/// It must return a [`PLUGIN_TRANSFORM_CAPSULE_NAME`] capsule built with
/// [`new_plugin_transform_capsule`].
pub const PLUGIN_TRANSFORM_METHOD: &str = "_luna_plugin_transform_capsule";

/// `Send` wrapper letting an `LmPluginTranform` (raw pointers) live in a capsule.
///
/// The plugin owns the pointers' thread-safety. The host only reads the descriptor.
#[repr(transparent)]
pub struct PluginTransformCarrier(pub LmPluginTransform);

// Safety: the descriptor is only read on the host thread, its pointers are opaque.
unsafe impl Send for PluginTransformCarrier {}

/// Builds a capsule carrying a plugin transformation descriptor **by value**.
///
/// Unlike [`new_capsule`], the descriptor is stored inline in Python-managed memory, so there is
/// not cross-allocator handle for the host to free: it copies the descriptor out with
/// [`read_plugin_transform`], and the capsule frees its own copy on GC. Ownership of the
/// descriptor's `user_data` transfers to the host on copy, so a producer should hand out a fresh
/// descriptor per capusle.
pub fn new_plugin_transform_capsule(
    py: Python<'_>,
    descriptor: LmPluginTransform,
) -> PyResult<Bound<'_, PyCapsule>> {
    PyCapsule::new_with_value(
        py,
        PluginTransformCarrier(descriptor),
        PLUGIN_TRANSFORM_CAPSULE_NAME,
    )
}

/// Copies the plugin transformation descriptor out of a capsule (non-consuming).
pub fn read_plugin_transform(capsule: &Bound<'_, PyCapsule>) -> PyResult<LmPluginTransform> {
    let pointer = capsule.pointer_checked(Some(PLUGIN_TRANSFORM_CAPSULE_NAME))?;
    // `pointer_checked` yields a `NonNull`; `LmPluginTransform` is `Copy`, so this
    // reads a copy and leaves the capsule's own copy intact.
    let carrier = unsafe { pointer.cast::<PluginTransformCarrier>().as_ref() };
    Ok(carrier.0)
}

/// Capsule name carrying a plugin analysis descriptor from a plugin to the host pass manager.
pub const PLUGIN_ANALYSIS_CAPSULE_NAME: &CStr = c"luna_model.capi.PluginAnalysis.v1";

/// Method name a plugin analysis object must expose so the host can obtain its analysis descriptor.
///
/// It must return a [`PLUGIN_ANALYSIS_CAPSULE_NAME`] capsule built with
/// [`new_plugin_analysis_capsule`].
pub const PLUGIN_ANALYSIS_METHOD: &str = "_luna_plugin_analysis_capsule";

/// `Send` wrapper letting an `LmPluginAnalysis` (raw pointers) live in a capsule.
///
/// The plugin owns the pointers' thread-safety. The host only reads the descriptor.
#[repr(transparent)]
pub struct PluginAnalysisCarrier(pub LmPluginAnalysis);

// Safety: the descriptor is only read on the host thread, its pointers are opaque.
unsafe impl Send for PluginAnalysisCarrier {}

/// Builds a capsule carrying a plugin analysis descriptor **by value** (see
/// [`new_plugin_transform_capsule`] for the ownership rationale).
pub fn new_plugin_analysis_capsule(
    py: Python<'_>,
    descriptor: LmPluginAnalysis,
) -> PyResult<Bound<'_, PyCapsule>> {
    PyCapsule::new_with_value(
        py,
        PluginAnalysisCarrier(descriptor),
        PLUGIN_ANALYSIS_CAPSULE_NAME,
    )
}

/// Copies the plugin analysis descriptor out of a capsule (non-consuming).
pub fn read_plugin_analysis(capsule: &Bound<'_, PyCapsule>) -> PyResult<LmPluginAnalysis> {
    let pointer = capsule.pointer_checked(Some(PLUGIN_ANALYSIS_CAPSULE_NAME))?;
    // `LmPluginAnalysis` is `Copy`; read a copy and leave the capsule's own copy intact.
    let carrier = unsafe { pointer.cast::<PluginAnalysisCarrier>().as_ref() };
    Ok(carrier.0)
}

/// Capsule name carrying a plugin composite descriptor from a plugin to the host pass manager.
pub const PLUGIN_COMPOSITE_CAPSULE_NAME: &CStr = c"luna_model.capi.PluginComposite.v1";

/// Method name a plugin composite object must expose so the host can obtain its descriptor.
///
/// It must return a [`PLUGIN_COMPOSITE_CAPSULE_NAME`] capsule built with
/// [`new_plugin_composite_capsule`].
pub const PLUGIN_COMPOSITE_METHOD: &str = "_luna_plugin_composite_capsule";

/// `Send` wrapper letting an `LmPluginComposite` (raw pointers) live in a capsule.
///
/// The plugin owns the pointers' thread-safety. The host only reads the descriptor.
#[repr(transparent)]
pub struct PluginCompositeCarrier(pub LmPluginComposite);

// Safety: the descriptor is only read on the host thread, its pointers are opaque.
unsafe impl Send for PluginCompositeCarrier {}

/// Builds a capsule carrying a plugin composite descriptor **by value** (see
/// [`new_plugin_transform_capsule`] for the ownership rationale).
pub fn new_plugin_composite_capsule(
    py: Python<'_>,
    descriptor: LmPluginComposite,
) -> PyResult<Bound<'_, PyCapsule>> {
    PyCapsule::new_with_value(
        py,
        PluginCompositeCarrier(descriptor),
        PLUGIN_COMPOSITE_CAPSULE_NAME,
    )
}

/// Copies the plugin composite descriptor out of a capsule (non-consuming).
pub fn read_plugin_composite(capsule: &Bound<'_, PyCapsule>) -> PyResult<LmPluginComposite> {
    let pointer = capsule.pointer_checked(Some(PLUGIN_COMPOSITE_CAPSULE_NAME))?;
    // `LmPluginComposite` is `Copy`; read a copy and leave the capsule's own copy intact.
    let carrier = unsafe { pointer.cast::<PluginCompositeCarrier>().as_ref() };
    Ok(carrier.0)
}

/// Capsule name carrying a plugin meta-analysis descriptor from a plugin to the host pass manager.
pub const PLUGIN_META_ANALYSIS_CAPSULE_NAME: &CStr = c"luna_model.capi.PluginMetaAnalysis.v1";

/// Method name a plugin meta-analysis object must expose so the host can obtain its descriptor.
///
/// It must return a [`PLUGIN_META_ANALYSIS_CAPSULE_NAME`] capsule built with
/// [`new_plugin_meta_analysis_capsule`].
pub const PLUGIN_META_ANALYSIS_METHOD: &str = "_luna_plugin_meta_analysis_capsule";

/// `Send` wrapper letting an `LmPluginMetaAnalysis` (raw pointers) live in a capsule.
///
/// The plugin owns the pointers' thread-safety. The host only reads the descriptor.
#[repr(transparent)]
pub struct PluginMetaAnalysisCarrier(pub LmPluginMetaAnalysis);

// Safety: the descriptor is only read on the host thread, its pointers are opaque.
unsafe impl Send for PluginMetaAnalysisCarrier {}

/// Builds a capsule carrying a plugin meta-analysis descriptor **by value** (see
/// [`new_plugin_transform_capsule`] for the ownership rationale).
pub fn new_plugin_meta_analysis_capsule(
    py: Python<'_>,
    descriptor: LmPluginMetaAnalysis,
) -> PyResult<Bound<'_, PyCapsule>> {
    PyCapsule::new_with_value(
        py,
        PluginMetaAnalysisCarrier(descriptor),
        PLUGIN_META_ANALYSIS_CAPSULE_NAME,
    )
}

/// Copies the plugin meta-analysis descriptor out of a capsule (non-consuming).
pub fn read_plugin_meta_analysis(
    capsule: &Bound<'_, PyCapsule>,
) -> PyResult<LmPluginMetaAnalysis> {
    let pointer = capsule.pointer_checked(Some(PLUGIN_META_ANALYSIS_CAPSULE_NAME))?;
    // `LmPluginMetaAnalysis` is `Copy`; read a copy and leave the capsule's own copy intact.
    let carrier = unsafe { pointer.cast::<PluginMetaAnalysisCarrier>().as_ref() };
    Ok(carrier.0)
}

/// Capsule name carrying a plugin control-flow descriptor from a plugin to the host pass manager.
pub const PLUGIN_CONTROL_FLOW_CAPSULE_NAME: &CStr = c"luna_model.capi.PluginControlFlow.v1";

/// Method name a plugin control-flow object must expose so the host can obtain its descriptor.
///
/// It must return a [`PLUGIN_CONTROL_FLOW_CAPSULE_NAME`] capsule built with
/// [`new_plugin_control_flow_capsule`].
pub const PLUGIN_CONTROL_FLOW_METHOD: &str = "_luna_plugin_control_flow_capsule";

/// `Send` wrapper letting an `LmPluginControlFlow` (raw pointers) live in a capsule.
///
/// The plugin owns the pointers' thread-safety. The host only reads the descriptor.
#[repr(transparent)]
pub struct PluginControlFlowCarrier(pub LmPluginControlFlow);

// Safety: the descriptor is only read on the host thread, its pointers are opaque.
unsafe impl Send for PluginControlFlowCarrier {}

/// Builds a capsule carrying a plugin control-flow descriptor **by value** (see
/// [`new_plugin_transform_capsule`] for the ownership rationale).
pub fn new_plugin_control_flow_capsule(
    py: Python<'_>,
    descriptor: LmPluginControlFlow,
) -> PyResult<Bound<'_, PyCapsule>> {
    PyCapsule::new_with_value(
        py,
        PluginControlFlowCarrier(descriptor),
        PLUGIN_CONTROL_FLOW_CAPSULE_NAME,
    )
}

/// Copies the plugin control-flow descriptor out of a capsule (non-consuming).
pub fn read_plugin_control_flow(
    capsule: &Bound<'_, PyCapsule>,
) -> PyResult<LmPluginControlFlow> {
    let pointer = capsule.pointer_checked(Some(PLUGIN_CONTROL_FLOW_CAPSULE_NAME))?;
    // `LmPluginControlFlow` is `Copy`; read a copy and leave the capsule's own copy intact.
    let carrier = unsafe { pointer.cast::<PluginControlFlowCarrier>().as_ref() };
    Ok(carrier.0)
}
