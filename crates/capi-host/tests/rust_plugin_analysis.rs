//! End-to-end tests for plugin **analysis** passes driven by luna-model, and for one plugin
//! consuming another plugin's analysis result across the C-ABI.
//!
//! Everything below is authored in 100% safe Rust through the 1:1 mirror of the native pass traits
//! (`AnalysisPass` + `AnalysisResult`, and `TransformationPass` + `Reversible` + `Artifact`). The
//! SDK generates every `extern "C"` trampoline and descriptor; the core schedules the passes,
//! deposits the analysis result, and lends it back to the consuming pass — exactly as it does for
//! built-in passes.

use lunamodel_capi_host::capi_table::LUNA_MODEL_C_API;
use lunamodel_capi_host::plugin::{CPluginAnalysisPass, CPluginTransformPass};
use lunamodel_transpiler::PassManager;
use lunamodel_types::Vtype;
use rs_lunamodel::plugin::{descriptor, descriptor_analysis, register_backward};

/// Installs the host C-API table (the one host-side `unsafe`, shared by all tests).
fn init() {
    match unsafe { rs_lunamodel::init(&LUNA_MODEL_C_API as *const _) } {
        Ok(()) | Err(rs_lunamodel::InitError::AlreadyInitialized) => {}
        Err(e) => panic!("rs_lunamodel init failed: {e}"),
    }
}

/// A model with `n` binary variables in its objective, so the analysis has real, participating
/// variables to count.
fn model_with_vars(n: usize) -> lunamodel_core::Model {
    use lunamodel_core::prelude::LmAddAssign;
    let mut model = lunamodel_core::Model::default();
    for i in 0..n {
        let var = model
            .add_var(&format!("x{i}"), Vtype::Binary, None)
            .expect("add variable");
        model
            .objective_mut()
            .add_assign(&var)
            .expect("add variable to objective");
    }
    model
}

/// The shared **contract**: the analysis result type a producer and a consumer agree on. In a real
/// split this lives in plugin A's crate, which plugin B depends on and compiles — giving B the type
/// (and its `type_tag`) needed to decode A's result.
mod contract {
    use prost::Message;
    use rs_lunamodel::plugin::AnalysisResult;

    /// Stable name the result is stored/read under.
    pub const VARIABLE_COUNT: &str = "contract::variable-count";

    #[derive(Message, Clone, PartialEq)]
    pub struct VariableCount {
        #[prost(uint64, tag = "1")]
        pub count: u64,
    }

    impl AnalysisResult for VariableCount {
        fn static_type_tag() -> &'static str {
            "contract::variable-count"
        }
    }
}

/// **Plugin A** — provides an analysis computing the model's variable count.
mod plugin_a {
    use super::contract::{VARIABLE_COUNT, VariableCount};
    use rs_lunamodel::plugin::{AnalysisPass, PassContext};

    pub struct CountVariables;

    impl AnalysisPass for CountVariables {
        type Output = VariableCount;

        const ID: &'static str = "pluginA::count-variables";
        const PROVIDES: &'static str = VARIABLE_COUNT;

        fn name(&self) -> &str {
            "count-variables"
        }

        fn run(
            &self,
            model: &rs_lunamodel::Model,
            _ctx: &PassContext,
        ) -> rs_lunamodel::Result<VariableCount> {
            Ok(VariableCount {
                count: model.num_variables()? as u64,
            })
        }
    }
}

/// **Plugin B** — a transform that *requires* plugin A's analysis and reads it back through the
/// context. B depends only on the shared contract type, never on the core.
mod plugin_b {
    use super::contract::{VARIABLE_COUNT, VariableCount};
    use prost::Message;
    use rs_lunamodel::plugin::{Artifact, PassContext, Reversible, TransformationPass};

    #[derive(Message)]
    pub struct CountArtifact {
        #[prost(uint64, tag = "1")]
        pub count: u64,
    }

    impl Artifact for CountArtifact {
        fn static_type_tag() -> &'static str {
            "pluginB::count-artifact"
        }
    }

    pub struct TagWithVariableCount {
        requires: Vec<String>,
    }

    impl TagWithVariableCount {
        pub fn new() -> Self {
            Self {
                requires: vec![VARIABLE_COUNT.to_string()],
            }
        }
    }

    impl Reversible for TagWithVariableCount {
        type Artifact = CountArtifact;

        const ID: &'static str = "pluginB::tag-with-variable-count";

        fn backward(
            _artifact: &CountArtifact,
            solution: rs_lunamodel::Solution,
        ) -> rs_lunamodel::Result<rs_lunamodel::Solution> {
            Ok(solution)
        }
    }

    impl TransformationPass for TagWithVariableCount {
        fn name(&self) -> &str {
            "tag-with-variable-count"
        }

        fn requires(&self) -> &[String] {
            &self.requires
        }

        fn forward(
            &self,
            model: &mut rs_lunamodel::Model,
            ctx: &PassContext,
        ) -> rs_lunamodel::Result<CountArtifact> {
            // Read plugin A's analysis result across the C-ABI, decoding into the contract type.
            let counted: VariableCount = ctx.require_analysis(VARIABLE_COUNT)?;
            model.set_name(format!("vars={}", counted.count))?;
            Ok(CountArtifact {
                count: counted.count,
            })
        }
    }
}

/// plugin ↔ luna-model: luna-model runs a plugin analysis, deposits its result, then lends it to a
/// plugin transform in the same run. Forward reads it; backward still round-trips.
#[test]
fn plugin_transform_reads_plugin_analysis() {
    init();
    register_backward::<plugin_b::TagWithVariableCount>().expect("register plugin backward");

    let analysis = descriptor_analysis(plugin_a::CountVariables).expect("analysis descriptor");
    let transform =
        descriptor(plugin_b::TagWithVariableCount::new()).expect("transform descriptor");

    let pm = PassManager::default()
        .add_analysis(CPluginAnalysisPass::from_descriptor(&analysis))
        .add_transform(CPluginTransformPass::from_descriptor(&transform));

    let out = pm.run(model_with_vars(3)).expect("pass manager run");
    assert_eq!(
        out.model.name, "vars=3",
        "transform read the analysis result"
    );

    out.record
        .backward(lunamodel_core::Solution::default())
        .expect("backward");
}

/// plugin A used-by plugin B, mediated by luna-model: B consumes A's analysis result across the
/// C-ABI. Also proves the dependency is real — B alone (no A) fails.
#[test]
fn cross_plugin_analysis_dependency() {
    init();
    register_backward::<plugin_b::TagWithVariableCount>().expect("register plugin backward");

    // Without plugin A, plugin B's required analysis is absent -> the run fails.
    let transform =
        descriptor(plugin_b::TagWithVariableCount::new()).expect("transform descriptor");
    let pm_missing =
        PassManager::default().add_transform(CPluginTransformPass::from_descriptor(&transform));
    assert!(
        pm_missing.run(model_with_vars(2)).is_err(),
        "plugin B must fail when plugin A's analysis was not run"
    );

    // With plugin A scheduled first, plugin B reads A's result and the run succeeds.
    let analysis = descriptor_analysis(plugin_a::CountVariables).expect("analysis descriptor");
    let transform =
        descriptor(plugin_b::TagWithVariableCount::new()).expect("transform descriptor");
    let pm = PassManager::default()
        .add_analysis(CPluginAnalysisPass::from_descriptor(&analysis))
        .add_transform(CPluginTransformPass::from_descriptor(&transform));

    let out = pm.run(model_with_vars(2)).expect("pass manager run");
    assert_eq!(
        out.model.name, "vars=2",
        "plugin B consumed plugin A's result"
    );
}
