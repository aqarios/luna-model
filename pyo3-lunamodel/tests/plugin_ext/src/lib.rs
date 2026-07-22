//! Example luna-model plugin extension — authored entirely in **safe Rust**, using the **1:1
//! mirror of the native pass traits** (`Artifact`/`Reversible`/`TransformationPass` and
//! `AnalysisPass`/`AnalysisResult`). The `py_transform_plugin!` / `py_analysis_plugin!` /
//! `register_plugins!` macros generate the Python classes and the `__luna_register__` hook, so a
//! user drives everything from Python:
//!
//! ```python
//! import luna_model, luna_plugin_ext
//! luna_model.register(luna_plugin_ext)
//! from luna_model.transformation import PassManager
//! # plugin A's analysis feeds plugin B's transform, all via luna_model:
//! out = PassManager([luna_plugin_ext.CountVariablesAnalysis(),
//!                    luna_plugin_ext.TagWithVariableCount()]).run(model)
//! ```
//!
//! This is the C-boundary (Rust-backed) authoring path; it coexists with writing passes in pure
//! Python (subclassing luna-model's `PyTransformationPass`/`PyAnalysisPass`).

use pyo3::prelude::*;
use rs_lunamodel::plugin::{Artifact, PassContext, Reversible, TransformationPass};

const NEW_NAME: &str = "renamed-by-py-plugin";

/// Typed inverse-transformation artifact for the standalone rename pass.
pub struct RenameInverse {
    previous: String,
}

impl Artifact for RenameInverse {
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

/// A standalone transformation (no analysis dependency) — 100% safe Rust, shaped like a native pass.
#[derive(Clone, Default)]
struct RenameLogic;

impl Reversible for RenameLogic {
    type Artifact = RenameInverse;

    const ID: &'static str = "luna_plugin_ext::rename";

    fn backward(
        _artifact: &RenameInverse,
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
    ) -> rs_lunamodel::Result<RenameInverse> {
        let previous = model.name()?;
        model.set_name(NEW_NAME.to_string())?;
        Ok(RenameInverse { previous })
    }
}

pyo3_lunamodel::py_transform_plugin!(pub RenamePass(RenameLogic) => RenameArtifact);

#[pymethods]
impl RenamePass {
    #[new]
    fn new() -> Self {
        Self(RenameLogic)
    }
}

/// The shared **contract** both plugins agree on: the analysis result type. In a real split this
/// lives in plugin A's crate, which plugin B depends on to obtain the type (and its `type_tag`).
mod contract {
    use rs_lunamodel::plugin::AnalysisResult;

    /// Stable name the result is stored/read under.
    pub const VARIABLE_COUNT: &str = "luna_plugin_ext::variable-count";

    #[derive(Clone)]
    pub struct VariableCount {
        pub count: u64,
    }

    impl AnalysisResult for VariableCount {
        fn static_type_tag() -> &'static str {
            "luna_plugin_ext::variable-count"
        }

        fn serialize(&self) -> rs_lunamodel::Result<Vec<u8>> {
            Ok(self.count.to_le_bytes().to_vec())
        }

        fn deserialize(bytes: &[u8]) -> rs_lunamodel::Result<Self> {
            let arr: [u8; 8] = bytes
                .try_into()
                .map_err(|_| rs_lunamodel::Error::computation("bad VariableCount bytes"))?;
            Ok(Self {
                count: u64::from_le_bytes(arr),
            })
        }
    }
}

/// **Plugin A** — an analysis counting the model's participating variables.
mod plugin_a {
    use super::contract::{VARIABLE_COUNT, VariableCount};
    use pyo3::prelude::*;
    use rs_lunamodel::plugin::{AnalysisPass, PassContext};

    #[derive(Clone)]
    pub struct CountVariablesLogic;

    impl AnalysisPass for CountVariablesLogic {
        type Output = VariableCount;

        const ID: &'static str = "luna_plugin_ext::count-variables";
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

    pyo3_lunamodel::py_analysis_plugin!(pub CountVariablesAnalysis(CountVariablesLogic) => VariableCountResult);

    #[pymethods]
    impl CountVariablesAnalysis {
        #[new]
        fn new() -> Self {
            Self(CountVariablesLogic)
        }
    }
}

/// **Plugin B** — a transform that *requires* plugin A's analysis and reads it back through the
/// context, then tags the model name with the count. Depends only on the shared contract type.
mod plugin_b {
    use super::contract::{VARIABLE_COUNT, VariableCount};
    use pyo3::prelude::*;
    use rs_lunamodel::plugin::{Artifact, PassContext, Reversible, TransformationPass};

    /// Inverse artifact — tagging carries no reversal data.
    pub struct TagInverse;

    impl Artifact for TagInverse {
        fn static_type_tag() -> &'static str {
            "luna_plugin_ext::tag::artifact"
        }

        fn serialize(&self) -> rs_lunamodel::Result<Vec<u8>> {
            Ok(Vec::new())
        }

        fn deserialize(_bytes: &[u8]) -> rs_lunamodel::Result<Self> {
            Ok(Self)
        }
    }

    #[derive(Clone)]
    pub struct TagWithCountLogic {
        requires: Vec<String>,
    }

    impl TagWithCountLogic {
        pub fn new() -> Self {
            Self {
                requires: vec![VARIABLE_COUNT.to_string()],
            }
        }
    }

    impl Reversible for TagWithCountLogic {
        type Artifact = TagInverse;

        const ID: &'static str = "luna_plugin_ext::tag-with-variable-count";

        fn backward(
            _artifact: &TagInverse,
            solution: rs_lunamodel::Solution,
        ) -> rs_lunamodel::Result<rs_lunamodel::Solution> {
            Ok(solution)
        }
    }

    impl TransformationPass for TagWithCountLogic {
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
        ) -> rs_lunamodel::Result<TagInverse> {
            // Read plugin A's analysis result across the C-ABI, decoding into the contract type.
            let counted: VariableCount = ctx.require_analysis(VARIABLE_COUNT)?;
            model.set_name(format!("vars={}", counted.count))?;
            Ok(TagInverse)
        }
    }

    pyo3_lunamodel::py_transform_plugin!(pub TagWithVariableCount(TagWithCountLogic) => TagArtifact);

    #[pymethods]
    impl TagWithVariableCount {
        #[new]
        fn new() -> Self {
            Self(TagWithCountLogic::new())
        }
    }
}

#[pymodule]
fn luna_plugin_ext(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<RenamePass>()?;
    m.add_class::<RenameArtifact>()?;
    m.add_class::<plugin_a::CountVariablesAnalysis>()?;
    m.add_class::<plugin_a::VariableCountResult>()?;
    m.add_class::<plugin_b::TagWithVariableCount>()?;
    m.add_class::<plugin_b::TagArtifact>()?;
    // Only reversible passes (transforms) register a backward handler; analyses do not.
    pyo3_lunamodel::register_plugins!(m, RenameLogic, plugin_b::TagWithCountLogic)?;
    Ok(())
}
