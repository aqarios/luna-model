//! End-to-end: a plugin transform consumes a **built-in** analysis result across the C-ABI.
//!
//! The built-in `MinValueForConstraintAnalysis` stores a typed `MinConstraintValues`; the host
//! serializes it on demand (registered serializer) into the shared protobuf wire form; the plugin
//! decodes it into `lunamodel_capi_plugin_proto::proto::MinConstraintValues` — without linking core.

use lunamodel_capi_host::capi_table::LUNA_MODEL_C_API;
use lunamodel_capi_host::plugin::CPluginTransformPass;
use lunamodel_capi_plugin_proto::proto::MinConstraintValues;
use lunamodel_transform::analysis::MinValueForConstraintAnalysis;
use lunamodel_transpiler::{AnalysisPass, PassManager};
use rs_lunamodel::plugin::{Artifact, PassContext, Reversible, TransformationPass};

fn init() {
    match unsafe { rs_lunamodel::init(&LUNA_MODEL_C_API as *const _) } {
        Ok(()) | Err(rs_lunamodel::InitError::AlreadyInitialized) => {}
        Err(e) => panic!("rs_lunamodel init failed: {e}"),
    }
    // Register built-in analysis serializers so a plugin can consume built-in results.
    lunamodel_transform::register_builtins();
}

/// Empty plugin artifact (reading carries no reversal data).
#[derive(::prost::Message)]
struct ReadMarker {}

impl Artifact for ReadMarker {
    fn static_type_tag() -> &'static str {
        "test::read-marker"
    }
}

/// A plugin transform that requires the built-in analysis and reads it back through the context.
struct ReadMinConstraints {
    requires: Vec<String>,
}

impl ReadMinConstraints {
    fn new() -> Self {
        Self {
            requires: vec![MinValueForConstraintAnalysis::PROVIDES.to_string()],
        }
    }
}

impl Reversible for ReadMinConstraints {
    type Artifact = ReadMarker;
    const ID: &'static str = "test::read-min-constraints";
    fn backward(
        _artifact: &ReadMarker,
        solution: rs_lunamodel::Solution,
    ) -> rs_lunamodel::Result<rs_lunamodel::Solution> {
        Ok(solution)
    }
}

impl TransformationPass for ReadMinConstraints {
    fn name(&self) -> &str {
        "read-min-constraints"
    }
    fn requires(&self) -> &[String] {
        &self.requires
    }
    fn forward(
        &self,
        model: &mut rs_lunamodel::Model,
        ctx: &PassContext,
    ) -> rs_lunamodel::Result<ReadMarker> {
        // Decode the BUILT-IN analysis result into the shared proto type — no core linked here.
        let mins: MinConstraintValues =
            ctx.require_analysis(MinValueForConstraintAnalysis::PROVIDES)?;
        model.set_name(format!("min-constraints={}", mins.vals.len()))?;
        Ok(ReadMarker {})
    }
}

#[test]
fn plugin_reads_builtin_analysis() {
    init();
    rs_lunamodel::plugin::register_backward::<ReadMinConstraints>().expect("register backward");

    let transform =
        rs_lunamodel::plugin::descriptor(ReadMinConstraints::new()).expect("build descriptor");

    // Built-in analysis first, then the plugin transform that requires it.
    let pm = PassManager::default()
        .add_analysis(MinValueForConstraintAnalysis)
        .add_transform(CPluginTransformPass::from_descriptor(&transform));

    let out = pm
        .run(lunamodel_core::Model::default())
        .expect("pass manager run");

    // A default model has no constraints, so the map is empty — but a successful, correctly-typed
    // decode proves the whole built-in -> host-serialize -> plugin-decode path worked.
    assert_eq!(out.model.name, "min-constraints=0");
}
