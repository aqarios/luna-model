// A C++ luna-model plugin, authored with the header-only `lm::plugin` SDK and exposed to Python
// with nanobind. It proves scope 2b: passes written in C++ that the host `PassManager` (driven from
// Python) runs and reverses, including one pass that consumes another's analysis.
//
// The module mirrors the pyo3 SDK's Python contract exactly, so these C++ passes plug into the same
// host code as pyo3 and pure-Python passes:
//   - each pass object exposes `_luna_plugin_{transform,analysis}_capsule()`;
//   - the module exposes `__luna_register__()` (invoked by `luna_model.register`) which binds the
//     C API table and registers the transforms' reversal handlers.
//
// Payloads cross as raw bytes (the byte-level C ABI): this test round-trips them within the module,
// so it uses trivial encodings. Cross-language protobuf wire-sharing is covered separately (the
// Rust/C++/Python analysis-sharing test).

#include <cstdint>
#include <memory>
#include <span>
#include <stdexcept>
#include <string>
#include <vector>

#define LUNA_MODEL_PYTHON_EXTENSION
#include <luna_model/plugin.hpp>

#include <nanobind/nanobind.h>

#include "plugin.pb.h"  // generated from proto/plugin.proto (protobuf_generate)

namespace nb = nanobind;

namespace {

// Analysis name + result type tag the producer publishes and the consumer requires.
constexpr const char *VAR_COUNT_NAME = "cxxplugin::var-count";
constexpr const char *VAR_COUNT_TAG = "cxxplugin::var-count.v1";

// Serialize a protobuf message into the byte payload the C ABI carries.
template <typename Message>
std::vector<std::uint8_t> to_bytes(const Message &message) {
    const std::string wire = message.SerializeAsString();
    return std::vector<std::uint8_t>(wire.begin(), wire.end());
}

// --- RenamePass: a reversible transform that prefixes the model name ---
//
// forward stores the original name as the inverse artifact; a rename does not touch the solution
// space, so backward returns the solution unchanged (proving the reversal path is invoked + wired).

class RenamePass : public lm::plugin::TransformationPass {
  public:
    static constexpr const char *ID = "cxxplugin::rename";

    std::string id() const override { return ID; }
    std::string name() const override { return "cxx rename"; }

    std::vector<std::uint8_t> forward(lm::Model &model, const lm::plugin::PassContext &) override {
        const std::string original = model.name();
        model.set_name("renamed:" + original);

        luna_cxx_plugin::RenameArtifact artifact;
        artifact.set_original_name(original);
        return to_bytes(artifact);
    }

    static lm::Solution backward(std::span<const std::uint8_t>, lm::Solution solution) {
        return solution;  // a rename leaves the solution unchanged
    }
};

// --- VarCountAnalysis: publishes the model's variable count under VAR_COUNT_NAME ---

class VarCountAnalysis : public lm::plugin::AnalysisPass {
  public:
    std::string id() const override { return "cxxplugin::var-count-analysis"; }
    std::string name() const override { return "cxx variable count"; }
    std::string provides() const override { return VAR_COUNT_NAME; }
    std::string result_type_tag() const override { return VAR_COUNT_TAG; }

    std::vector<std::uint8_t> run(const lm::Model &model, const lm::plugin::PassContext &) override {
        luna_cxx_plugin::VariableCount result;
        result.set_count(static_cast<std::uint64_t>(model.num_variables()));
        return to_bytes(result);
    }
};

// --- TagWithCountPass: a transform that CONSUMES VarCountAnalysis ---
//
// Requires the analysis, reads the count via `require_analysis`, and renames the model to embed it —
// proving cross-pass analysis consumption + dependency ordering. Reversible (identity on solution).

class TagWithCountPass : public lm::plugin::TransformationPass {
  public:
    static constexpr const char *ID = "cxxplugin::tag-with-count";

    std::string id() const override { return ID; }
    std::string name() const override { return "cxx tag with count"; }
    std::vector<std::string> requires_analyses() const override { return {VAR_COUNT_NAME}; }

    std::vector<std::uint8_t> forward(lm::Model &model,
                                      const lm::plugin::PassContext &ctx) override {
        const std::vector<std::uint8_t> raw = ctx.require_analysis(VAR_COUNT_NAME, VAR_COUNT_TAG);
        luna_cxx_plugin::VariableCount count;
        if (!count.ParseFromArray(raw.data(), static_cast<int>(raw.size()))) {
            throw std::runtime_error("failed to decode VariableCount analysis result");
        }

        const std::string original = model.name();
        model.set_name(original + ":vars=" + std::to_string(count.count()));

        luna_cxx_plugin::RenameArtifact artifact;
        artifact.set_original_name(original);
        return to_bytes(artifact);
    }

    static lm::Solution backward(std::span<const std::uint8_t>, lm::Solution solution) {
        return solution;
    }
};

// --- Python-facing shells ---
//
// Each shell exposes the capsule method the host reads. `make_descriptor` takes ownership of a fresh
// pass instance per call, so the shells build one each time (they hold no per-instance state here).

nb::object rename_capsule(const RenamePass &) {
    return nb::steal(
        lm::plugin::python::transform_capsule(lm::plugin::make_descriptor(std::make_unique<RenamePass>())));
}

nb::object var_count_capsule(const VarCountAnalysis &) {
    return nb::steal(lm::plugin::python::analysis_capsule(
        lm::plugin::make_descriptor(std::make_unique<VarCountAnalysis>())));
}

nb::object tag_with_count_capsule(const TagWithCountPass &) {
    return nb::steal(lm::plugin::python::transform_capsule(
        lm::plugin::make_descriptor(std::make_unique<TagWithCountPass>())));
}

}  // namespace

NB_MODULE(luna_cxx_plugin_ext, m) {
    // Bind the host C API table so the passes' `lm_*` calls dispatch at run time.
    if (import_luna_model() != 0) {
        throw nb::python_error();
    }

    nb::class_<RenamePass>(m, "RenamePass")
        .def(nb::init<>())
        .def(lm::plugin::python::TRANSFORM_METHOD, &rename_capsule);

    nb::class_<VarCountAnalysis>(m, "VarCountAnalysis")
        .def(nb::init<>())
        .def(lm::plugin::python::ANALYSIS_METHOD, &var_count_capsule);

    nb::class_<TagWithCountPass>(m, "TagWithCountPass")
        .def(nb::init<>())
        .def(lm::plugin::python::TRANSFORM_METHOD, &tag_with_count_capsule);

    // Invoked by `luna_model.register(module)`: install the transforms' reversal handlers.
    m.def("__luna_register__", []() {
        if (import_luna_model() != 0) {
            throw nb::python_error();
        }
        lm::plugin::register_backward<RenamePass>();
        lm::plugin::register_backward<TagWithCountPass>();
    });
}
