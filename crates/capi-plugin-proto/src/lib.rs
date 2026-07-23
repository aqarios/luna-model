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
    pub const TYPE_TAG: &'static str = "luna_model::min-constraint-values";
}

impl AnalysisResult for proto::MinConstraintValues {
    fn static_type_tag() -> &'static str {
        Self::TYPE_TAG
    }
}
