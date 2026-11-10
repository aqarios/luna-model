#ifndef LUNA_MODEL_ERROR_HPP
#define LUNA_MODEL_ERROR_HPP

#include <stdexcept>
#include <string>
#include <string_view>
#include <utility>

#include <luna_model.h>

namespace lm {

/* A closed classification of an Error's domain/business-logic kind, mirroring
 * `LunaModelError::kind_tag()` on the Rust side, with a catch-all for anything unrecognized.
 *
 * The C ABI reports a domain error's identity as an open string tag (`RichError::kind`) rather
 * than a closed enum, deliberately: plugins mint their own kinds (e.g.
 * `"myplugin::infeasible_subproblem"`) without requiring an ABI version bump, and the core can
 * add new native error variants the same way. `ErrorKind` gives callers of this SDK the best of
 * both: exhaustive, compiler-checked `switch` matching for every native LunaModel error kind known
 * at the time this catalog was last kept in sync with `LunaModelError::kind_tag()`, while `Other`
 * absorbs anything else (a plugin-defined kind, a coarse ABI-level fallback tag such as
 * `"errored"`/`"panic"`, or a newer core variant this catalog doesn't recognize yet) without
 * losing the raw tag — still available via `Error::kind_tag()` — or breaking forward
 * compatibility. */
enum class ErrorKind {
    DifferentEnvironments,
    VariableExists,
    VariableNotExisting,
    VariableNameInvalid,
    ConstraintNameInvalid,
    InvalidBounds,
    InvalidInversion,
    InvertedBinaryCreation,
    Compression,
    Decoding,
    Formatter,
    UnsupportedOperation,
    Internal,
    Dtype,
    Computation,
    NoConstraintForKey,
    DuplicateConstraintName,
    ModelNotQuadratic,
    ModelNotUnconstrained,
    ModelSenseNotMinimize,
    Vtype,
    Translation,
    IndexOutOfBounds,
    Evaluation,
    SampleIncorrectLength,
    SampleUnexpectedVariable,
    SampleIncompatibleVtype,
    VariableNames,
    TransformationPass,
    AnalysisPass,
    IfElsePass,
    MetaAnalysisPass,
    Transformation,
    RandomSampling,
    InvalidTolerance,
    Infeasible,
    External,
    /* A kind tag not recognized by this catalog: a plugin-defined kind, a coarse ABI-level
     * fallback tag (e.g. "errored", "panic"), or a core error variant added after this SDK's
     * catalog was last updated. The raw tag is still available via `Error::kind_tag()`. */
    Other,
};

namespace detail {
/* Parses a raw `RichError`/`kind_tag()` string into its typed classification. */
inline ErrorKind error_kind_from_tag(std::string_view tag) {
    if (tag == "different_environments") return ErrorKind::DifferentEnvironments;
    if (tag == "variable_exists") return ErrorKind::VariableExists;
    if (tag == "variable_not_existing") return ErrorKind::VariableNotExisting;
    if (tag == "variable_name_invalid") return ErrorKind::VariableNameInvalid;
    if (tag == "constraint_name_invalid") return ErrorKind::ConstraintNameInvalid;
    if (tag == "invalid_bounds") return ErrorKind::InvalidBounds;
    if (tag == "invalid_inversion") return ErrorKind::InvalidInversion;
    if (tag == "inverted_binary_creation") return ErrorKind::InvertedBinaryCreation;
    if (tag == "compression") return ErrorKind::Compression;
    if (tag == "decoding") return ErrorKind::Decoding;
    if (tag == "formatter") return ErrorKind::Formatter;
    if (tag == "unsupported_operation") return ErrorKind::UnsupportedOperation;
    if (tag == "internal") return ErrorKind::Internal;
    if (tag == "dtype") return ErrorKind::Dtype;
    if (tag == "computation") return ErrorKind::Computation;
    if (tag == "no_constraint_for_key") return ErrorKind::NoConstraintForKey;
    if (tag == "duplicate_constraint_name") return ErrorKind::DuplicateConstraintName;
    if (tag == "model_not_quadratic") return ErrorKind::ModelNotQuadratic;
    if (tag == "model_not_unconstrained") return ErrorKind::ModelNotUnconstrained;
    if (tag == "model_sense_not_minimize") return ErrorKind::ModelSenseNotMinimize;
    if (tag == "vtype") return ErrorKind::Vtype;
    if (tag == "translation") return ErrorKind::Translation;
    if (tag == "index_out_of_bounds") return ErrorKind::IndexOutOfBounds;
    if (tag == "evaluation") return ErrorKind::Evaluation;
    if (tag == "sample_incorrect_length") return ErrorKind::SampleIncorrectLength;
    if (tag == "sample_unexpected_variable") return ErrorKind::SampleUnexpectedVariable;
    if (tag == "sample_incompatible_vtype") return ErrorKind::SampleIncompatibleVtype;
    if (tag == "variable_names") return ErrorKind::VariableNames;
    if (tag == "transformation_pass") return ErrorKind::TransformationPass;
    if (tag == "analysis_pass") return ErrorKind::AnalysisPass;
    if (tag == "if_else_pass") return ErrorKind::IfElsePass;
    if (tag == "meta_analysis_pass") return ErrorKind::MetaAnalysisPass;
    if (tag == "transformation") return ErrorKind::Transformation;
    if (tag == "random_sampling") return ErrorKind::RandomSampling;
    if (tag == "invalid_tolerance") return ErrorKind::InvalidTolerance;
    if (tag == "infeasible") return ErrorKind::Infeasible;
    if (tag == "external") return ErrorKind::External;
    return ErrorKind::Other;
}
} // namespace detail

/* Exception carrying a C API status code, the LunaModel error message, and (when available) the
 * structured error's raw kind tag / typed classification. */
class Error : public std::runtime_error {
  public:
    Error(LmStatus status, std::string message, std::string kind_tag = std::string())
        : std::runtime_error(std::move(message)), status_(status),
          kind_tag_(std::move(kind_tag)) {}

    LmStatus status() const noexcept { return status_; }

    /* The raw, open string tag identifying this error's specific origin (e.g.
     * "variable_not_existing", or a plugin-defined "myplugin::infeasible_subproblem"). Empty if
     * no structured error was available at all. Prefer `kind()` for a typed classification. */
    const std::string &kind_tag() const noexcept { return kind_tag_; }

    /* This error's domain/business-logic kind, typed and switchable — see `ErrorKind` for why
     * it's a closed enum with an `Other` fallback rather than the raw tag directly. */
    ErrorKind kind() const noexcept { return detail::error_kind_from_tag(kind_tag_); }

  private:
    LmStatus status_;
    std::string kind_tag_;
};

} // namespace lm

#endif // LUNA_MODEL_ERROR_HPP
