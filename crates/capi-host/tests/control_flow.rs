//! End-to-end test for plugin **control-flow** passes driven by luna-model.
//!
//! A control-flow pass reads the model and builds a `ControlFlowPlan` at run time — the pipeline of
//! steps to execute next — which the engine then runs. Authored in 100% safe Rust through the 1:1
//! mirror of the native `ControlFlowPass` trait, over the `ControlFlowPlan` builder facade (steps
//! are added by handing it passes, exactly like building a native pipeline). The SDK generates the
//! `extern "C"` trampolines and the `control_flow_plan_*` dispatch; the core runs the selected plan.

use std::sync::Arc;

use lunamodel_capi_host::capi_table::LUNA_MODEL_C_API;
use lunamodel_capi_host::plugin::CPluginControlFlowPass;
use lunamodel_transpiler::{PassManager, PipelineStep};
use rs_lunamodel::plugin::{descriptor_control_flow, register_backward};

fn init() {
    match unsafe { rs_lunamodel::init(&LUNA_MODEL_C_API as *const _) } {
        Ok(()) | Err(rs_lunamodel::InitError::AlreadyInitialized) => {}
        Err(e) => panic!("rs_lunamodel init failed: {e}"),
    }
}

/// The transform the control-flow pass selects into its branch.
mod branch {
    use prost::Message;
    use rs_lunamodel::plugin::{Artifact, PassContext, Reversible, TransformationPass};

    pub const NEW_NAME: &str = "via-control-flow";

    #[derive(Message)]
    pub struct RenameInverse {
        #[prost(string, tag = "1")]
        pub previous: String,
    }

    impl Artifact for RenameInverse {
        fn static_type_tag() -> &'static str {
            "cf-test::rename-inverse"
        }
    }

    #[derive(Clone, Default)]
    pub struct RenameTo;

    impl Reversible for RenameTo {
        type Artifact = RenameInverse;

        const ID: &'static str = "cf-test::rename";

        fn backward(
            _artifact: &RenameInverse,
            solution: rs_lunamodel::Solution,
        ) -> rs_lunamodel::Result<rs_lunamodel::Solution> {
            Ok(solution)
        }
    }

    impl TransformationPass for RenameTo {
        fn name(&self) -> &str {
            "rename-to"
        }

        fn forward(
            &self,
            model: &mut rs_lunamodel::Model,
            _ctx: &PassContext,
        ) -> rs_lunamodel::Result<RenameInverse> {
            let previous = model.name()?;
            model.set_name(NEW_NAME.to_string())?;
            Ok(RenameInverse { previous })
        }
    }
}

/// The control-flow pass: builds a one-step plan (the rename branch) at run time.
mod cf {
    use rs_lunamodel::plugin::{ControlFlowPass, ControlFlowPlan, PassContext};

    pub struct PickBranch;

    impl ControlFlowPass for PickBranch {
        fn name(&self) -> &str {
            "pick-branch"
        }

        fn run(
            &self,
            _model: &rs_lunamodel::Model,
            _ctx: &PassContext,
        ) -> rs_lunamodel::Result<ControlFlowPlan> {
            let mut plan = ControlFlowPlan::new("chosen")?;
            plan.add_transform(super::branch::RenameTo)?;
            Ok(plan)
        }
    }
}

/// The control-flow pass builds a plan whose single step (the rename transform) the engine runs;
/// reversal then round-trips back through the selected branch.
#[test]
fn control_flow_selects_and_runs_a_branch() {
    init();
    // The branch transform is reversible, so its backward must be registered (routed by its id).
    register_backward::<branch::RenameTo>().expect("register branch backward");

    let cf = CPluginControlFlowPass::from_descriptor(&descriptor_control_flow(cf::PickBranch).unwrap());
    let out = PassManager::default()
        .add_step(PipelineStep::ControlFlow(Arc::new(cf)))
        .run(lunamodel_core::Model::default())
        .expect("pass manager run");

    // The engine executed the branch the control-flow pass selected.
    assert_eq!(out.model.name, branch::NEW_NAME);

    // Reversal recurses into the selected sub-pipeline and dispatches the branch's `backward`.
    out.record
        .backward(lunamodel_core::Solution::default())
        .expect("control-flow backward");
}
