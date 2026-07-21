//! Example luna-model plugin extension — authored entirely in **safe Rust**, using the **1:1
//! mirror of the native pass traits** (`Artifact` + `Reversible` + `TransformationPass`).
//!
//! The transformation logic is identical in shape to a native pass. The `py_transform_plugin!` and
//! `register_plugins!` macros generate the Python class and the `__luna_register__` hook, so a user
//! drives everything from Python:
//!
//! ```python
//! import luna_model, luna_plugin_ext
//! luna_model.register(luna_plugin_ext)
//! from luna_model.transformation import PassManager
//! out = PassManager([luna_plugin_ext.RenamePass(), ...]).run(model)
//! ```

use pyo3::prelude::*;
use rs_lunamodel::plugin::{Artifact, PassContext, Reversible, TransformationPass};

const NEW_NAME: &str = "renamed-by-py-plugin";

/// Typed inverse-transformation artifact.
struct RenameArtifact {
    previous: String,
}

impl Artifact for RenameArtifact {
    fn static_type_tag() -> &'static str {
        "luna_plugin_ext::rename::artifact"
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

/// The plugin transformation — 100% safe Rust, identical in shape to a native pass.
#[derive(Default)]
struct RenameLogic;

impl Reversible for RenameLogic {
    type Artifact = RenameArtifact;

    const ID: &'static str = "luna_plugin_ext::rename";

    fn backward(
        _artifact: &RenameArtifact,
        solution: rs_lunamodel::Solution,
    ) -> rs_lunamodel::Result<rs_lunamodel::Solution> {
        // Renaming does not affect assignments, so reversal is a no-op on the solution.
        Ok(solution)
    }
}

impl TransformationPass for RenameLogic {
    fn name(&self) -> &str {
        "py-rename"
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

// Generate the Python-facing class and the module registration hook. No unsafe, no boilerplate.
pyo3_lunamodel::py_transform_plugin!(RenamePass => RenameLogic);

#[pymodule]
fn luna_plugin_ext(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<RenamePass>()?;
    pyo3_lunamodel::register_plugins!(m, RenameLogic)?;
    Ok(())
}
