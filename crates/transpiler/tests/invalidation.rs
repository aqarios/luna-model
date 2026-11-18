//! Model-part based analysis invalidation: runtime dropping, static conflict detection, and the
//! check that transformations only modify the parts they declare.

use lunamodel_core::{
    Model, Solution,
    prelude::{Constraint, Expression, LmAddAssign},
};
use lunamodel_transpiler::{
    AnalysisKey, AnalysisPass, Artifact, CompositePass, ControlFlowPass, ControlFlowPlan,
    PassContext, PassManager, PipelineStep, Reversible, TransformationPass, TranspileErrorKind,
    TranspileKindResult, analysis, control_flow, prelude::composite, transformation,
};
use lunamodel_types::{Comparator, EnumSet, ModelPart, Vtype};

struct NoArtifact;

impl Artifact for NoArtifact {
    fn static_type_tag() -> &'static str {
        "test::no-artifact"
    }

    fn serialize(&self) -> TranspileKindResult<Vec<u8>> {
        Ok(Vec::new())
    }

    fn deserialize(_: &[u8]) -> TranspileKindResult<Self> {
        Ok(NoArtifact)
    }
}

/// Changes each part in `parts` of the model.
fn touch(model: &mut Model, parts: EnumSet<ModelPart>) {
    for part in parts {
        match part {
            ModelPart::Objective => model.objective_mut().add_assign(&1.0).unwrap(),
            ModelPart::Sense => model.sense = !model.sense,
            ModelPart::Variables => {
                let name = format!("v{}", model.environment.vars().len());
                model.add_var(&name, Vtype::Binary, None).unwrap();
            }
            ModelPart::Constraints => {
                let lhs = Expression::new(model.environment.clone());
                let constr = Constraint::new(lhs, 1.0, Comparator::Le, None).unwrap();
                model.add_constraint(constr, None).unwrap();
            }
        }
    }
}

/// Transformation that declares `modifies` but actually changes `touches`.
#[transformation]
struct Touch {
    name: &'static str,
    requires: Vec<String>,
    modifies: EnumSet<ModelPart>,
    touches: EnumSet<ModelPart>,
}

impl Touch {
    fn new(name: &'static str, modifies: EnumSet<ModelPart>) -> Self {
        Self {
            name,
            requires: Vec::new(),
            modifies,
            touches: modifies,
        }
    }

    fn reader(name: &'static str, requirement: &str) -> Self {
        Self {
            requires: vec![requirement.to_owned()],
            ..Self::new(name, EnumSet::empty())
        }
    }
}

impl TransformationPass for Touch {
    fn name(&self) -> &str {
        self.name
    }

    fn requires(&self) -> &[String] {
        &self.requires
    }

    fn modifies(&self) -> EnumSet<ModelPart> {
        self.modifies
    }

    fn forward(&self, model: &mut Model, _: &PassContext) -> TranspileKindResult<NoArtifact> {
        touch(model, self.touches);
        Ok(NoArtifact)
    }
}

impl Reversible for Touch {
    type Artifact = NoArtifact;
    const ID: &'static str = "test::touch";

    fn backward(_: &NoArtifact, solution: Solution) -> TranspileKindResult<Solution> {
        Ok(solution)
    }
}

/// Transformation relying on the default `modifies`.
#[transformation]
struct Opaque;

impl TransformationPass for Opaque {
    fn name(&self) -> &str {
        "opaque"
    }

    fn forward(&self, _: &mut Model, _: &PassContext) -> TranspileKindResult<NoArtifact> {
        Ok(NoArtifact)
    }
}

impl Reversible for Opaque {
    type Artifact = NoArtifact;
    const ID: &'static str = "test::opaque";

    fn backward(_: &NoArtifact, solution: Solution) -> TranspileKindResult<Solution> {
        Ok(solution)
    }
}

/// Analysis providing `key`, depending on `depends_on`.
#[analysis]
struct Probe {
    key: &'static str,
    depends_on: EnumSet<ModelPart>,
}

impl AnalysisPass for Probe {
    type Result = ();
    const PROVIDES: &'static str = "test::probe";

    fn name(&self) -> &str {
        self.key
    }

    fn provides(&self) -> &str {
        self.key
    }

    fn key<T>() -> AnalysisKey<T> {
        AnalysisKey::new(Self::PROVIDES.to_owned())
    }

    fn depends_on(&self) -> EnumSet<ModelPart> {
        self.depends_on
    }

    fn run(&self, _: &Model, _: &PassContext) -> TranspileKindResult<()> {
        Ok(())
    }
}

/// Composite that modifies every part and provides `test::fused`.
#[composite]
struct Fuse;

impl CompositePass for Fuse {
    type Result = ();
    const NAME: &'static str = "fuse";
    const PROVIDES: &'static str = "test::fused";

    fn key<T>() -> AnalysisKey<T> {
        AnalysisKey::new(Self::PROVIDES.to_owned())
    }

    fn forward(&self, model: &mut Model, _: &PassContext) -> TranspileKindResult<(NoArtifact, ())> {
        touch(model, EnumSet::all());
        Ok((NoArtifact, ()))
    }
}

impl Reversible for Fuse {
    type Artifact = NoArtifact;
    const ID: &'static str = "test::fuse";

    fn backward(_: &NoArtifact, solution: Solution) -> TranspileKindResult<Solution> {
        Ok(solution)
    }
}

/// Control flow that always takes the `then` branch, but exposes both for validation.
#[control_flow]
struct Branch {
    then_steps: Vec<PipelineStep>,
    else_steps: Vec<PipelineStep>,
}

impl ControlFlowPass for Branch {
    fn name(&self) -> &str {
        "branch"
    }

    fn run(&self, _: &Model, _: &PassContext) -> TranspileKindResult<ControlFlowPlan> {
        Ok(ControlFlowPlan::new("then".into(), self.then_steps.clone()))
    }

    fn branches(&self) -> Vec<&[PipelineStep]> {
        vec![&self.then_steps, &self.else_steps]
    }
}

fn probe(key: &'static str, depends_on: EnumSet<ModelPart>) -> PipelineStep {
    Probe { key, depends_on }.into()
}

fn run(
    steps: Vec<PipelineStep>,
) -> Result<lunamodel_transpiler::TransformationOutput, TranspileErrorKind> {
    PassManager::from_steps(steps)
        .run(Model::default())
        .map_err(|e| e.kind)
}

fn has(output: &lunamodel_transpiler::TransformationOutput, key: &str) -> bool {
    output
        .analysis
        .get(&AnalysisKey::<()>::new(key.to_owned()))
        .is_some()
}

#[test]
fn transform_drops_only_overlapping_analyses() {
    let output = run(vec![
        probe("cons", ModelPart::Constraints.into()),
        probe("obj", ModelPart::Objective.into()),
        Touch::new("rewrite-objective", ModelPart::Objective.into()).into(),
    ])
    .unwrap();

    assert!(has(&output, "cons"));
    assert!(!has(&output, "obj"));
}

#[test]
fn undeclared_modifies_invalidates_everything() {
    let output = run(vec![
        probe("cons", ModelPart::Constraints.into()),
        Opaque.into(),
    ])
    .unwrap();

    assert!(!has(&output, "cons"));
}

#[test]
fn unrelated_modification_keeps_requirement() {
    run(vec![
        probe("cons", ModelPart::Constraints.into()),
        Touch::new("flip-sense", ModelPart::Sense.into()).into(),
        Touch::reader("reader", "cons").into(),
    ])
    .unwrap();
}

#[test]
fn static_check_names_the_conflict() {
    let err = run(vec![
        probe("cons", ModelPart::Constraints | ModelPart::Variables),
        Touch::new(
            "rewrite-constraints",
            ModelPart::Constraints | ModelPart::Sense,
        )
        .into(),
        Touch::reader("reader", "cons").into(),
    ])
    .err()
    .unwrap();

    let TranspileErrorKind::InvalidatedRequirement(inv) = &err else {
        panic!("unexpected error: {err}");
    };
    assert_eq!(inv.computed_by, "cons");
    assert_eq!(inv.invalidated_by, "rewrite-constraints");
    assert_eq!(inv.parts, EnumSet::only(ModelPart::Constraints));
    assert_eq!(
        err.to_string(),
        "pass 'reader' requires 'cons', computed by 'cons' but invalidated by \
         'rewrite-constraints' which modifies {Constraints}; recompute it after \
         'rewrite-constraints' or move 'rewrite-constraints' earlier"
    );
}

#[test]
fn branch_invalidation_carries_over() {
    let err = run(vec![
        probe("cons", ModelPart::Constraints.into()),
        Branch {
            then_steps: vec![],
            else_steps: vec![Touch::new("rewrite", ModelPart::Constraints.into()).into()],
        }
        .into(),
        Touch::reader("reader", "cons").into(),
    ])
    .err()
    .unwrap();

    assert!(
        matches!(err, TranspileErrorKind::InvalidatedRequirement(inv) if inv.invalidated_by == "rewrite")
    );
}

#[test]
fn recomputing_in_every_branch_satisfies_requirement() {
    let recompute = || {
        vec![
            Touch::new("rewrite", ModelPart::Constraints.into()).into(),
            probe("cons", ModelPart::Constraints.into()),
        ]
    };
    run(vec![
        probe("cons", ModelPart::Constraints.into()),
        Branch {
            then_steps: recompute(),
            else_steps: recompute(),
        }
        .into(),
        Touch::reader("reader", "cons").into(),
    ])
    .unwrap();
}

#[test]
fn composite_keeps_its_own_result() {
    let output = run(vec![
        Fuse.into(),
        Touch::reader("reader", "test::fused").into(),
    ])
    .unwrap();

    assert!(has(&output, "test::fused"));
}

#[test]
fn undeclared_modification_is_caught() {
    let liar = || Touch {
        touches: ModelPart::Constraints | ModelPart::Sense,
        ..Touch::new("liar", ModelPart::Sense.into())
    };

    let err = run(vec![liar().into()]).err().unwrap();
    let TranspileErrorKind::UndeclaredModification { pass_name, parts } = &err else {
        panic!("unexpected error: {err}");
    };
    assert_eq!(pass_name, "liar");
    assert_eq!(*parts, EnumSet::only(ModelPart::Constraints));

    PassManager::from_steps(vec![liar().into()])
        .verify_modifies(false)
        .run(Model::default())
        .unwrap();
}
