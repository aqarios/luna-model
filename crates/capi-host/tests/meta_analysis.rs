//! End-to-end test for plugin **meta-analysis** passes driven by luna-model.
//!
//! A meta-analysis reads no model; it introspects the pipeline steps that follow it and deposits a
//! result under `provides`, read back by later passes. Authored in 100% safe Rust through the 1:1
//! mirror of the native `MetaAnalysisPass` trait, over the `StepList` facade (the ABI-faithful view
//! of the following steps). The SDK generates the `extern "C"` trampolines and the `step_list_*`
//! dispatch; the core lends the step view and deposits the result.

use luna_model_ffi::plugin::{descriptor, descriptor_meta_analysis, register_backward};
use lunamodel_capi_host::capi_table::LUNA_MODEL_C_API;
use lunamodel_capi_host::plugin::{CPluginMetaAnalysisPass, CPluginTransformPass};
use lunamodel_transpiler::PassManager;

fn init() {
    match unsafe { luna_model_ffi::init(&LUNA_MODEL_C_API as *const _) } {
        Ok(()) | Err(luna_model_ffi::InitError::AlreadyInitialized) => {}
        Err(e) => panic!("luna_model_ffi init failed: {e}"),
    }
}

/// The shared contract: what the meta-analysis reports about the steps following it.
mod contract {
    use luna_model_ffi::plugin::AnalysisResult;
    use prost::Message;

    pub const FOLLOWING_STEPS: &str = "contract::following-steps";

    #[derive(Message, Clone, PartialEq)]
    pub struct FollowingSteps {
        #[prost(uint64, tag = "1")]
        pub count: u64,
        #[prost(string, tag = "2")]
        pub first_name: String,
        #[prost(string, tag = "3")]
        pub first_kind: String,
    }

    impl AnalysisResult for FollowingSteps {
        fn static_type_tag() -> &'static str {
            "contract::following-steps"
        }
    }
}

/// The meta-analysis: reports how many steps follow it, plus the first one's name and kind — read
/// entirely through the `StepList` facade.
mod meta {
    use super::contract::{FOLLOWING_STEPS, FollowingSteps};
    use luna_model_ffi::plugin::{MetaAnalysisPass, StepList};

    pub struct DescribeFollowing;

    impl MetaAnalysisPass for DescribeFollowing {
        type Output = FollowingSteps;

        const PROVIDES: &'static str = FOLLOWING_STEPS;

        fn name(&self) -> &str {
            "describe-following"
        }

        fn run(&self, steps: &StepList) -> luna_model_ffi::Result<FollowingSteps> {
            let (first_name, first_kind) = match steps.get(0) {
                Some(step) => (step.name()?, format!("{:?}", step.kind()?)),
                None => (String::new(), String::new()),
            };
            Ok(FollowingSteps {
                count: steps.len() as u64,
                first_name,
                first_kind,
            })
        }
    }
}

/// A consumer transform that reads the meta-analysis result and tags the model with it.
mod consumer {
    use super::contract::{FOLLOWING_STEPS, FollowingSteps};
    use luna_model_ffi::plugin::{Artifact, PassContext, Reversible, TransformationPass};
    use prost::Message;

    #[derive(Message)]
    pub struct TagInverse {}

    impl Artifact for TagInverse {
        fn static_type_tag() -> &'static str {
            "consumer::tag-inverse"
        }
    }

    pub struct TagWithFollowingInfo {
        requires: Vec<String>,
    }

    impl TagWithFollowingInfo {
        pub fn new() -> Self {
            Self {
                requires: vec![FOLLOWING_STEPS.to_string()],
            }
        }
    }

    impl Reversible for TagWithFollowingInfo {
        type Artifact = TagInverse;

        const ID: &'static str = "consumer::tag-with-following-info";

        fn backward(
            _artifact: &TagInverse,
            solution: luna_model_ffi::Solution,
        ) -> luna_model_ffi::Result<luna_model_ffi::Solution> {
            Ok(solution)
        }
    }

    impl TransformationPass for TagWithFollowingInfo {
        fn name(&self) -> &str {
            "tag-following-info"
        }

        fn requires(&self) -> &[String] {
            &self.requires
        }

        fn forward(
            &self,
            model: &mut luna_model_ffi::Model,
            ctx: &PassContext,
        ) -> luna_model_ffi::Result<TagInverse> {
            let info: FollowingSteps = ctx.require_analysis(FOLLOWING_STEPS)?;
            model.set_name(format!(
                "steps={}:first={}:kind={}",
                info.count, info.first_name, info.first_kind
            ))?;
            Ok(TagInverse {})
        }
    }
}

/// The meta-analysis introspects the step view via the `StepList` facade — count, name, and kind —
/// and the consumer reads that result back. Matching the native semantics, the view starts at the
/// meta pass's own position, so it sees itself (a `MetaAnalysis`) followed by the consumer
/// transform: two steps, the first being the meta pass.
#[test]
fn meta_analysis_describes_following_steps() {
    init();
    register_backward::<consumer::TagWithFollowingInfo>().expect("register consumer backward");

    let meta = CPluginMetaAnalysisPass::from_descriptor(
        &descriptor_meta_analysis(meta::DescribeFollowing).unwrap(),
    );
    let tag = CPluginTransformPass::from_descriptor(
        &descriptor(consumer::TagWithFollowingInfo::new()).unwrap(),
    );

    let out = PassManager::default()
        .add_meta_analysis(meta)
        .add_transform(tag)
        .run(lunamodel_core::Model::default())
        .expect("pass manager run");

    // The step view is `[self (the meta pass), the consumer transform]` — count 2, first is the
    // meta pass itself. This exercises `StepList::len`/`get`/`Step::name`/`Step::kind`.
    assert_eq!(
        out.model.name,
        "steps=2:first=describe-following:kind=MetaAnalysis"
    );
}
