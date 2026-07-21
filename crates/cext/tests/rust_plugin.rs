//! End-to-end test of a plugin transformation authored as a **native Rust extension** — no Python,
//! plugin body in **100% safe Rust**, and using the **1:1 mirror of the native pass traits**
//! (`Artifact` + `Reversible` + `TransformationPass`). The SDK generates every `unsafe extern "C"`
//! trampoline, the descriptor, and the reversal registration; the core runs and reverses the pass.

use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

use lunamodel_cext::capi_table::LUNA_MODEL_C_API;
use lunamodel_cext::plugin::CPluginTransformPass;

use lunamodel_transpiler::PassManager;
use rs_lunamodel::plugin::{Artifact, PassContext, Reversible, TransformationPass};

const NEW_NAME: &str = "renamed-by-rust-plugin";

static BACKWARD_CALLS: AtomicUsize = AtomicUsize::new(0);
static BACKWARD_PREV_OK: AtomicBool = AtomicBool::new(false);

/// Typed inverse-transformation artifact — exactly as a native pass would define one.
struct RenameArtifact {
    previous: String,
}

impl Artifact for RenameArtifact {
    fn static_type_tag() -> &'static str {
        "rust::rename::artifact"
    }

    fn serialize(&self) -> rs_lunamodel::Result<Vec<u8>> {
        Ok(self.previous.clone().into_bytes())
    }

    fn deserialize(bytes: &[u8]) -> rs_lunamodel::Result<Self> {
        Ok(Self {
            previous: String::from_utf8_lossy(bytes).into_owned(),
        })
    }
}

/// The whole plugin — safe Rust, no C-boundary types.
struct RenamePass;

impl Reversible for RenamePass {
    type Artifact = RenameArtifact;

    const ID: &'static str = "rust::rename";

    fn backward(
        artifact: &RenameArtifact,
        solution: rs_lunamodel::Solution,
    ) -> rs_lunamodel::Result<rs_lunamodel::Solution> {
        // Renaming does not affect assignments; record that reversal dispatched here and that the
        // typed artifact round-tripped its data through serialize/deserialize.
        BACKWARD_CALLS.fetch_add(1, Ordering::SeqCst);
        BACKWARD_PREV_OK.store(artifact.previous == "unnamed", Ordering::SeqCst);
        Ok(solution)
    }
}

impl TransformationPass for RenamePass {
    fn name(&self) -> &str {
        "rust-rename"
    }

    fn forward(
        &self,
        model: &mut rs_lunamodel::Model,
        _ctx: &PassContext,
    ) -> rs_lunamodel::Result<RenameArtifact> {
        let previous = model.name()?;
        model.set_name(NEW_NAME.to_string())?;
        Ok(RenameArtifact { previous })
    }
}

#[test]
fn rust_plugin_forward_and_backward() {
    // Driver bootstrap: install the host C-API table (the one unavoidable `unsafe`, host-side).
    match unsafe { rs_lunamodel::init(&LUNA_MODEL_C_API as *const _) } {
        Ok(()) | Err(rs_lunamodel::InitError::AlreadyInitialized) => {}
        Err(e) => panic!("rs_lunamodel init failed: {e}"),
    }

    // Safe SDK calls — no unsafe, no descriptor hand-assembly.
    rs_lunamodel::plugin::register_backward::<RenamePass>().expect("register plugin backward");
    let descriptor = rs_lunamodel::plugin::descriptor(RenamePass).expect("build descriptor");

    // Driver: the core builds and runs the pipeline and reverses the solution.
    let pm =
        PassManager::default().add_transform(CPluginTransformPass::from_descriptor(&descriptor));
    let out = pm
        .run(luna_model::core::Model::default())
        .expect("pass manager run");

    assert_eq!(out.model.name, NEW_NAME);

    let _solution = out
        .record
        .backward(luna_model::core::Solution::default())
        .expect("backward");
    assert_eq!(BACKWARD_CALLS.load(Ordering::SeqCst), 1);
    assert!(
        BACKWARD_PREV_OK.load(Ordering::SeqCst),
        "typed artifact round-tripped the previous name through serialize/deserialize"
    );
}

/// Reproduces the Python test's pipeline in Rust: the plugin pass mixed with a built-in
/// (`ChangeSensePass`), forward + reverse through both. Isolates whether mixed-pass reversal is a
/// real defect or a build artifact of the Python harness.
#[test]
fn rust_plugin_mixed_with_builtin() {
    use lunamodel_transform::transformation::ChangeSensePass;

    match unsafe { rs_lunamodel::init(&LUNA_MODEL_C_API as *const _) } {
        Ok(()) | Err(rs_lunamodel::InitError::AlreadyInitialized) => {}
        Err(e) => panic!("rs_lunamodel init failed: {e}"),
    }
    lunamodel_transform::register_backward();
    rs_lunamodel::plugin::register_backward::<RenamePass>().expect("register plugin backward");

    let descriptor = rs_lunamodel::plugin::descriptor(RenamePass).expect("build descriptor");
    let pm = PassManager::default()
        .add_transform(CPluginTransformPass::from_descriptor(&descriptor))
        .add_transform(ChangeSensePass::new(luna_model::types::Sense::Max));
    let out = pm
        .run(luna_model::core::Model::default())
        .expect("pass manager run");

    assert_eq!(out.model.name, NEW_NAME);
    assert_eq!(out.model.sense, luna_model::types::Sense::Max);

    // Reverse through ChangeSensePass (native) then the plugin — this is where the Python test failed.
    out.record
        .backward(luna_model::core::Solution::default())
        .expect("mixed backward");
}
