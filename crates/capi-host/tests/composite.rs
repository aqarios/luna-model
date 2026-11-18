//! End-to-end tests for plugin **composite** passes driven by luna-model.
//!
//! A composite fuses a transformation and an analysis: one `forward` both mutates the model AND
//! produces an analysis result. Authored in 100% safe Rust through the 1:1 mirror of the native
//! `CompositePass` trait (`Reversible` + `Artifact` for the transform half, `AnalysisResult` for the
//! analysis half). The SDK generates every `extern "C"` trampoline; the core runs the pass, deposits
//! the result, and lends it to later passes — exactly as for a built-in composite.

use luna_model_ffi::plugin::{descriptor, descriptor_composite, register_composite_backward};
use lunamodel_capi_host::capi_table::LUNA_MODEL_C_API;
use lunamodel_capi_host::plugin::{CPluginCompositePass, CPluginTransformPass};
use lunamodel_transpiler::PassManager;
use lunamodel_types::Vtype;

/// Installs the host C-API table (the one host-side `unsafe`, shared by all tests).
fn init() {
    match unsafe { luna_model_ffi::init(&LUNA_MODEL_C_API as *const _) } {
        Ok(()) | Err(luna_model_ffi::InitError::AlreadyInitialized) => {}
        Err(e) => panic!("luna_model_ffi init failed: {e}"),
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

/// The shared **contract**: the analysis result type the composite produces and a consumer reads.
mod contract {
    use luna_model_ffi::plugin::AnalysisResult;
    use prost::Message;

    /// Stable name the composite's result is stored/read under.
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

/// The **composite**: renames the model (transform half) AND publishes the variable count (analysis
/// half), in a single forward.
mod composite {
    use super::contract::{VARIABLE_COUNT, VariableCount};
    use luna_model_ffi::plugin::{Artifact, CompositePass, PassContext, Reversible};
    use prost::Message;

    pub const NEW_NAME: &str = "renamed-by-composite";

    /// Inverse artifact: the model's name before the pass ran.
    #[derive(Message)]
    pub struct RenameInverse {
        #[prost(string, tag = "1")]
        pub previous: String,
    }

    impl Artifact for RenameInverse {
        fn static_type_tag() -> &'static str {
            "composite::rename-inverse"
        }
    }

    pub struct RenameAndCount;

    impl Reversible for RenameAndCount {
        type Artifact = RenameInverse;

        const ID: &'static str = "composite::rename-and-count";

        fn backward(
            _artifact: &RenameInverse,
            solution: luna_model_ffi::Solution,
        ) -> luna_model_ffi::Result<luna_model_ffi::Solution> {
            // A rename does not affect the solution space.
            Ok(solution)
        }
    }

    impl CompositePass for RenameAndCount {
        type Output = VariableCount;

        const PROVIDES: &'static str = VARIABLE_COUNT;

        fn name(&self) -> &str {
            "rename-and-count"
        }

        fn forward(
            &self,
            model: &mut luna_model_ffi::Model,
            _ctx: &PassContext,
        ) -> luna_model_ffi::Result<(RenameInverse, VariableCount)> {
            let previous = model.name()?;
            model.set_name(NEW_NAME.to_string())?;
            let count = model.num_variables()? as u64;
            Ok((RenameInverse { previous }, VariableCount { count }))
        }
    }
}

/// A consumer transform that reads the composite's deposited analysis result across the C-ABI.
mod consumer {
    use super::contract::{VARIABLE_COUNT, VariableCount};
    use luna_model_ffi::plugin::{Artifact, PassContext, Reversible, TransformationPass};
    use prost::Message;

    #[derive(Message)]
    pub struct TagInverse {}

    impl Artifact for TagInverse {
        fn static_type_tag() -> &'static str {
            "consumer::tag-inverse"
        }
    }

    pub struct TagWithCount {
        requires: Vec<String>,
    }

    impl TagWithCount {
        pub fn new() -> Self {
            Self {
                requires: vec![VARIABLE_COUNT.to_string()],
            }
        }
    }

    impl Reversible for TagWithCount {
        type Artifact = TagInverse;

        const ID: &'static str = "consumer::tag-with-count";

        fn backward(
            _artifact: &TagInverse,
            solution: luna_model_ffi::Solution,
        ) -> luna_model_ffi::Result<luna_model_ffi::Solution> {
            Ok(solution)
        }
    }

    impl TransformationPass for TagWithCount {
        fn name(&self) -> &str {
            "tag-with-count"
        }

        fn requires(&self) -> &[String] {
            &self.requires
        }

        fn forward(
            &self,
            model: &mut luna_model_ffi::Model,
            ctx: &PassContext,
        ) -> luna_model_ffi::Result<TagInverse> {
            let counted: VariableCount = ctx.require_analysis(VARIABLE_COUNT)?;
            model.set_name(format!("vars={}", counted.count))?;
            Ok(TagInverse {})
        }
    }
}

/// The composite's transform half mutates the model, and reversal round-trips through its
/// registered `backward` (which routes by the plugin's stable id).
#[test]
fn composite_forward_transforms_and_reverses() {
    init();
    register_composite_backward::<composite::RenameAndCount>()
        .expect("register composite backward");

    let pass = CPluginCompositePass::from_descriptor(
        &descriptor_composite(composite::RenameAndCount).unwrap(),
    );
    let out = PassManager::default()
        .add_composite(pass)
        .run(model_with_vars(3))
        .expect("pass manager run");

    assert_eq!(out.model.name, composite::NEW_NAME);

    // Reversal must find the composite's handler by id and dispatch into its `backward`.
    out.record
        .backward(lunamodel_core::Solution::default())
        .expect("composite backward");
}

/// The composite's analysis half deposits a result under `PROVIDES`, which a later plugin transform
/// reads back across the C-ABI — proving a composite acts as an analysis producer too.
#[test]
fn composite_result_is_consumed_downstream() {
    init();
    register_composite_backward::<composite::RenameAndCount>()
        .expect("register composite backward");

    let composite = CPluginCompositePass::from_descriptor(
        &descriptor_composite(composite::RenameAndCount).unwrap(),
    );
    let tag =
        CPluginTransformPass::from_descriptor(&descriptor(consumer::TagWithCount::new()).unwrap());

    let out = PassManager::default()
        .add_composite(composite)
        .add_transform(tag)
        .run(model_with_vars(3))
        .expect("pass manager run");

    // The consumer read the count the composite published (3 participating variables).
    assert_eq!(out.model.name, "vars=3");
}
