//! Protobuf wire messages for the plugin C-ABI boundary — leaf crate (no core).
//!
//! The `.proto` in `proto/` is the source of truth; Rust is generated here, and `cxx-lunamodel`
//! will generate C++ from the same schema. Host and plugins share these types, so a plugin decodes
//! a built-in analysis result without linking the core. Serialization comes free from the shared
//! `AnalysisResult`/`Artifact` prost path — each type only declares its `static_type_tag`.

use rs_lunamodel::plugin::AnalysisResult;

pub mod proto {
    include!(concat!(env!("OUT_DIR"), "/lunamodel.plugin.rs"));
}

impl proto::MinConstraintValues {
    /// The result's type tag: the protobuf fully-qualified name (`package.Message`). Using the FQN
    /// means every language derives the same tag straight from the generated type — Rust here, and
    /// C++ via `descriptor()->full_name()` — so no one hand-writes or shares a separate tag string.
    /// Must equal the `.proto`'s `package` + message name.
    pub const TYPE_TAG: &'static str = "lunamodel.plugin.MinConstraintValues";
}

impl AnalysisResult for proto::MinConstraintValues {
    fn static_type_tag() -> &'static str {
        Self::TYPE_TAG
    }
}
