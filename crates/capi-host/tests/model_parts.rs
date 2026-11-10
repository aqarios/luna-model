//! End-to-end tests for the model parts plugin passes declare across the C ABI: `modifies` and
//! `depends_on` drive analysis invalidation and static validation exactly as for native passes, the
//! step view reports them, and an undeclared modification is caught.

use luna_model_ffi::plugin::{
    descriptor, descriptor_analysis, descriptor_meta_analysis, register_backward,
};
use luna_model_ffi::types::{EnumSet, ModelPart};
use lunamodel_capi_host::capi_table::LUNA_MODEL_C_API;
use lunamodel_capi_host::plugin::{
    CPluginAnalysisPass, CPluginMetaAnalysisPass, CPluginTransformPass,
};
use lunamodel_transpiler::{PassManager, TranspileErrorKind};
use prost::Message;

fn init() {
    match unsafe { luna_model_ffi::init(&LUNA_MODEL_C_API as *const _) } {
        Ok(()) | Err(luna_model_ffi::InitError::AlreadyInitialized) => {}
        Err(e) => panic!("luna_model_ffi init failed: {e}"),
    }
    register_backward::<passes::Mark>().expect("register backward");
}

mod passes {
    use luna_model_ffi::plugin::{
        AnalysisPass, AnalysisResult, Artifact, MetaAnalysisPass, PassContext, Reversible,
        StepList, TransformationPass,
    };
    use luna_model_ffi::types::{EnumSet, ModelPart, Sense};
    use prost::Message;

    pub const COUNT: &str = "parts::count";
    pub const NEXT_MODIFIES: &str = "parts::next-modifies";

    #[derive(Message, Clone, PartialEq)]
    pub struct Count {
        #[prost(uint64, tag = "1")]
        pub value: u64,
    }

    impl AnalysisResult for Count {
        fn static_type_tag() -> &'static str {
            "parts::count"
        }
    }

    /// Analysis whose result depends only on the constraints.
    pub struct CountConstraints;

    impl AnalysisPass for CountConstraints {
        type Output = Count;
        const ID: &'static str = "parts::count-constraints";
        const PROVIDES: &'static str = COUNT;

        fn name(&self) -> &str {
            "count-constraints"
        }

        fn depends_on(&self) -> EnumSet<ModelPart> {
            ModelPart::Constraints.into()
        }

        fn run(&self, _: &luna_model_ffi::Model, _: &PassContext) -> luna_model_ffi::Result<Count> {
            Ok(Count { value: 0 })
        }
    }

    /// Meta-analysis reporting the `modifies` bits of the step after it.
    pub struct NextModifies;

    impl MetaAnalysisPass for NextModifies {
        type Output = Count;
        const PROVIDES: &'static str = NEXT_MODIFIES;

        fn name(&self) -> &str {
            "next-modifies"
        }

        fn run(&self, steps: &StepList) -> luna_model_ffi::Result<Count> {
            let parts = steps.get(1).expect("a following step").modifies()?;
            Ok(Count {
                value: parts.as_u64(),
            })
        }
    }

    #[derive(Message)]
    pub struct NoArtifact {}

    impl Artifact for NoArtifact {
        fn static_type_tag() -> &'static str {
            "parts::no-artifact"
        }
    }

    /// Transformation that declares `modifies`, optionally requires an analysis, and flips the
    /// sense when `flip_sense` is set.
    pub struct Mark {
        pub name: &'static str,
        pub requires: Vec<String>,
        pub modifies: EnumSet<ModelPart>,
        pub flip_sense: bool,
    }

    impl Mark {
        pub fn new(name: &'static str, modifies: EnumSet<ModelPart>) -> Self {
            Self {
                name,
                requires: Vec::new(),
                modifies,
                flip_sense: false,
            }
        }

        pub fn reader(name: &'static str) -> Self {
            Self {
                requires: vec![COUNT.to_owned()],
                ..Self::new(name, EnumSet::empty())
            }
        }
    }

    impl Reversible for Mark {
        type Artifact = NoArtifact;
        const ID: &'static str = "parts::mark";

        fn backward(
            _: &NoArtifact,
            solution: luna_model_ffi::Solution,
        ) -> luna_model_ffi::Result<luna_model_ffi::Solution> {
            Ok(solution)
        }
    }

    impl TransformationPass for Mark {
        fn name(&self) -> &str {
            self.name
        }

        fn requires(&self) -> &[String] {
            &self.requires
        }

        fn modifies(&self) -> EnumSet<ModelPart> {
            self.modifies
        }

        fn forward(
            &self,
            model: &mut luna_model_ffi::Model,
            _: &PassContext,
        ) -> luna_model_ffi::Result<NoArtifact> {
            if self.flip_sense {
                model.set_sense(Sense::Max)?;
            }
            Ok(NoArtifact {})
        }
    }
}

fn count() -> CPluginAnalysisPass {
    CPluginAnalysisPass::from_descriptor(&descriptor_analysis(passes::CountConstraints).unwrap())
}

fn mark(pass: passes::Mark) -> CPluginTransformPass {
    CPluginTransformPass::from_descriptor(&descriptor(pass).unwrap())
}

#[test]
fn unrelated_plugin_modification_keeps_plugin_analysis() {
    init();
    PassManager::default()
        .add_analysis(count())
        .add_transform(mark(passes::Mark::new(
            "objective",
            ModelPart::Objective.into(),
        )))
        .add_transform(mark(passes::Mark::reader("reader")))
        .run(lunamodel_core::Model::default())
        .expect("count survives a pass that only modifies the objective");
}

#[test]
fn overlapping_plugin_modification_invalidates_plugin_analysis() {
    init();
    let err = PassManager::default()
        .add_analysis(count())
        .add_transform(mark(passes::Mark::new(
            "constraints",
            ModelPart::Constraints.into(),
        )))
        .add_transform(mark(passes::Mark::reader("reader")))
        .run(lunamodel_core::Model::default())
        .expect_err("static validation fails");

    let TranspileErrorKind::InvalidatedRequirement(inv) = &err.kind else {
        panic!("unexpected error: {err}");
    };
    assert_eq!(inv.computed_by, "count-constraints");
    assert_eq!(inv.invalidated_by, "constraints");
    assert_eq!(inv.parts, EnumSet::only(ModelPart::Constraints));
}

#[test]
fn step_list_reports_modifies() {
    init();
    let meta = CPluginMetaAnalysisPass::from_descriptor(
        &descriptor_meta_analysis(passes::NextModifies).unwrap(),
    );
    let declared = ModelPart::Objective | ModelPart::Sense;
    let out = PassManager::default()
        .add_meta_analysis(meta)
        .add_transform(mark(passes::Mark::new("next", declared)))
        .run(lunamodel_core::Model::default())
        .expect("pass manager run");

    let stored = out
        .analysis
        .get_serialized(passes::NEXT_MODIFIES)
        .expect("meta result");
    let reported = passes::Count::decode(stored.bytes.as_slice()).unwrap();
    assert_eq!(reported.value, declared.as_u64());
}

#[test]
fn undeclared_plugin_modification_is_caught() {
    init();
    let liar = passes::Mark {
        flip_sense: true,
        ..passes::Mark::new("liar", EnumSet::empty())
    };
    let err = PassManager::default()
        .add_transform(mark(liar))
        .verify_modifies(true)
        .run(lunamodel_core::Model::default())
        .expect_err("undeclared sense change is caught");

    let TranspileErrorKind::UndeclaredModification { pass_name, parts } = &err.kind else {
        panic!("unexpected error: {err}");
    };
    assert_eq!(pass_name, "liar");
    assert_eq!(*parts, EnumSet::only(ModelPart::Sense));
}
