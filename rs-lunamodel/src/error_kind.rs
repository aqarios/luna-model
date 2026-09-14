//! Typed classification of the domain/business-logic error a [`crate::Error`] carries.
//!
//! The C ABI reports a domain error's identity as an open string tag (`RichError::kind`,
//! populated from [`lunamodel_error::LunaModelError::kind_tag`]) rather than a closed enum,
//! deliberately: plugins mint their own kinds (e.g. `"myplugin::infeasible_subproblem"`) without
//! requiring an ABI version bump, and the core can add new native error variants the same way.
//!
//! [`ErrorKind`] gives callers of this facade the best of both: exhaustive, compiler-checked
//! matching for every native LunaModel error kind known at the time this file was last kept in
//! sync with `LunaModelError::kind_tag()`, while [`ErrorKind::Other`] absorbs anything else
//! (a plugin-defined kind, or a newer core variant this catalog doesn't recognize yet) without
//! losing the raw tag or breaking forward compatibility.
//!
//! This intentionally does not attempt to also encode the coarse [`crate::Status`] fallback tags
//! (`"null_pointer"`, `"errored"`, etc., used when no richer kind was reported) — that
//! information already lives distinctly on `Error::status`; folding it into `ErrorKind` too would
//! just give two ways to learn the same fact. Those tags fall into `Other` here.

use std::fmt;

/// A closed classification of a [`crate::Error`]'s domain/business-logic kind, mirroring
/// [`lunamodel_error::LunaModelError::kind_tag`], with a catch-all for anything unrecognized.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErrorKind {
    /// Operands came from different environments.
    DifferentEnvironments,
    /// A variable name collided with an existing variable.
    VariableExists,
    /// A requested variable was not present.
    VariableNotExisting,
    /// A variable name failed validation.
    VariableNameInvalid,
    /// A constraint name failed validation.
    ConstraintNameInvalid,
    /// Bounds were invalid for the target variable type.
    InvalidBounds,
    /// A variable inversion request was invalid.
    InvalidInversion,
    /// Compression or decompression failed.
    Compression,
    /// Serialized data could not be decoded.
    Decoding,
    /// Formatting a developer/user-facing representation failed.
    Formatter,
    /// The requested operation is not supported for the target object.
    UnsupportedOperation,
    /// Internal invariant violation or unexpected state.
    Internal,
    /// Data type conversion or interpretation failed.
    Dtype,
    /// Numeric or symbolic computation failed.
    Computation,
    /// A named constraint could not be found.
    NoConstraintForKey,
    /// A duplicate constraint name was encountered.
    DuplicateConstraintName,
    /// A model exceeded the quadratic degree limit.
    ModelNotQuadratic,
    /// A model unexpectedly contained constraints.
    ModelNotUnconstrained,
    /// A model unexpectedly had non-minimization sense.
    ModelSenseNotMinimize,
    /// Variable types violated a downstream requirement.
    Vtype,
    /// Model or solution translation failed.
    Translation,
    /// An index was out of bounds.
    IndexOutOfBounds,
    /// Evaluation against a sample or assignment failed.
    Evaluation,
    /// A sample had the wrong length.
    SampleIncorrectLength,
    /// A sample referenced an unexpected variable.
    SampleUnexpectedVariable,
    /// Sample assignments were incompatible with variable types.
    SampleIncompatibleVtype,
    /// Variable names failed a translator-specific requirement.
    VariableNames,
    /// A transformation pass failed.
    TransformationPass,
    /// An analysis pass failed.
    AnalysisPass,
    /// An if/else control-flow pass failed.
    IfElsePass,
    /// A meta-analysis pass failed.
    MetaAnalysisPass,
    /// Compilation of a transformation/pipeline artifact failed, with no more specific kind.
    Transformation,
    /// Random sampling failed.
    RandomSampling,
    /// Invalid tolerance specified.
    InvalidTolerance,
    /// Model is infeasible.
    Infeasible,
    /// A single link in an external error's context chain.
    External,
    /// A kind tag not recognized by this catalog: a plugin-defined kind, a coarse ABI-level
    /// fallback tag (e.g. `"errored"`, `"panic"`), or a core error variant added after this
    /// facade's catalog was last updated. Carries the raw tag as reported.
    Other(String),
}

impl ErrorKind {
    /// Parses a raw `RichError`/`kind_tag()` string into its typed classification.
    pub(crate) fn from_tag(tag: &str) -> Self {
        use ErrorKind::*;
        match tag {
            "different_environments" => DifferentEnvironments,
            "variable_exists" => VariableExists,
            "variable_not_existing" => VariableNotExisting,
            "variable_name_invalid" => VariableNameInvalid,
            "constraint_name_invalid" => ConstraintNameInvalid,
            "invalid_bounds" => InvalidBounds,
            "invalid_inversion" => InvalidInversion,
            "compression" => Compression,
            "decoding" => Decoding,
            "formatter" => Formatter,
            "unsupported_operation" => UnsupportedOperation,
            "internal" => Internal,
            "dtype" => Dtype,
            "computation" => Computation,
            "no_constraint_for_key" => NoConstraintForKey,
            "duplicate_constraint_name" => DuplicateConstraintName,
            "model_not_quadratic" => ModelNotQuadratic,
            "model_not_unconstrained" => ModelNotUnconstrained,
            "model_sense_not_minimize" => ModelSenseNotMinimize,
            "vtype" => Vtype,
            "translation" => Translation,
            "index_out_of_bounds" => IndexOutOfBounds,
            "evaluation" => Evaluation,
            "sample_incorrect_length" => SampleIncorrectLength,
            "sample_unexpected_variable" => SampleUnexpectedVariable,
            "sample_incompatible_vtype" => SampleIncompatibleVtype,
            "variable_names" => VariableNames,
            "transformation_pass" => TransformationPass,
            "analysis_pass" => AnalysisPass,
            "if_else_pass" => IfElsePass,
            "meta_analysis_pass" => MetaAnalysisPass,
            "transformation" => Transformation,
            "random_sampling" => RandomSampling,
            "invalid_tolerance" => InvalidTolerance,
            "infeasible" => Infeasible,
            "external" => External,
            other => Other(other.to_string()),
        }
    }

    /// The raw tag this kind was parsed from, or would report as (round-trips through
    /// [`ErrorKind::from_tag`] for every recognized variant).
    pub fn as_tag(&self) -> &str {
        use ErrorKind::*;
        match self {
            DifferentEnvironments => "different_environments",
            VariableExists => "variable_exists",
            VariableNotExisting => "variable_not_existing",
            VariableNameInvalid => "variable_name_invalid",
            ConstraintNameInvalid => "constraint_name_invalid",
            InvalidBounds => "invalid_bounds",
            InvalidInversion => "invalid_inversion",
            Compression => "compression",
            Decoding => "decoding",
            Formatter => "formatter",
            UnsupportedOperation => "unsupported_operation",
            Internal => "internal",
            Dtype => "dtype",
            Computation => "computation",
            NoConstraintForKey => "no_constraint_for_key",
            DuplicateConstraintName => "duplicate_constraint_name",
            ModelNotQuadratic => "model_not_quadratic",
            ModelNotUnconstrained => "model_not_unconstrained",
            ModelSenseNotMinimize => "model_sense_not_minimize",
            Vtype => "vtype",
            Translation => "translation",
            IndexOutOfBounds => "index_out_of_bounds",
            Evaluation => "evaluation",
            SampleIncorrectLength => "sample_incorrect_length",
            SampleUnexpectedVariable => "sample_unexpected_variable",
            SampleIncompatibleVtype => "sample_incompatible_vtype",
            VariableNames => "variable_names",
            TransformationPass => "transformation_pass",
            AnalysisPass => "analysis_pass",
            IfElsePass => "if_else_pass",
            MetaAnalysisPass => "meta_analysis_pass",
            Transformation => "transformation",
            RandomSampling => "random_sampling",
            InvalidTolerance => "invalid_tolerance",
            Infeasible => "infeasible",
            External => "external",
            Other(tag) => tag,
        }
    }

    /// `true` for [`ErrorKind::Other`] — a tag this catalog doesn't recognize.
    pub fn is_other(&self) -> bool {
        matches!(self, ErrorKind::Other(_))
    }
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_tag())
    }
}
