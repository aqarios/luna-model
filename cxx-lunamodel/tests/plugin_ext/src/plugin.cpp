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
#include <string>
#include <vector>

// LUNA_MODEL_PYTHON_EXTENSION is set by the build (target_compile_definitions in CMakeLists.txt), so
// it also lands in compile_commands.json where clangd/editors can see it.
#include <luna_model/plugin.hpp>

#include <nanobind/nanobind.h>
#include <nanobind/stl/string.h>  // std::string <-> str for the interop function signatures

#include <luna_model/nanobind.hpp>  // type casters: lm::Model <-> luna_model.Model, etc.

#include "plugin.pb.h"              // generated from proto/plugin.proto
#include "luna_model/plugin.pb.h"   // generated from the wheel's luna_model/plugin.proto (get_proto)

namespace nb = nanobind;

namespace {

// Analysis slot names: the key the producer publishes under and the consumer reads back. The
// result's type tag is derived from the message type (its protobuf full name), never hand-written.
constexpr const char *VAR_COUNT_NAME = "cxxplugin::var-count";

// A built-in luna-model analysis this plugin consumes; its result is the protobuf
// `lunamodel.plugin.MinConstraintValues`, whose wire schema ships in the wheel (get_proto()).
constexpr const char *MIN_CONSTRAINT_NAME = "luna_model::min-value-for-constraint";

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
    std::string result_type_tag() const override {
        return lm::plugin::type_tag<luna_cxx_plugin::VariableCount>();
    }

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
        const auto count = ctx.require_analysis<luna_cxx_plugin::VariableCount>(VAR_COUNT_NAME);

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

// --- TagWithMinConstraintPass: a C++ transform that consumes a BUILT-IN (Rust) analysis ---
//
// Requires luna-model's built-in `min-value-for-constraint` analysis and decodes its result with
// the C++ class generated from the wheel's shared `luna_model/plugin.proto`. This is the real
// cross-language path: a Rust analysis result read by a C++ plugin over the C ABI via one schema.

class TagWithMinConstraintPass : public lm::plugin::TransformationPass {
  public:
    static constexpr const char *ID = "cxxplugin::tag-with-min-constraint";

    std::string id() const override { return ID; }
    std::string name() const override { return "cxx tag with min-constraint"; }
    std::vector<std::string> requires_analyses() const override { return {MIN_CONSTRAINT_NAME}; }

    std::vector<std::uint8_t> forward(lm::Model &model,
                                      const lm::plugin::PassContext &ctx) override {
        const auto values =
            ctx.require_analysis<lunamodel::plugin::MinConstraintValues>(MIN_CONSTRAINT_NAME);

        const std::string original = model.name();
        model.set_name("min-constraints:" + std::to_string(values.vals().size()));

        luna_cxx_plugin::RenameArtifact artifact;
        artifact.set_original_name(original);
        return to_bytes(artifact);
    }

    static lm::Solution backward(std::span<const std::uint8_t>, lm::Solution solution) {
        return solution;
    }
};

// --- RenameAndCountPass: a COMPOSITE pass (fused transform + analysis) ---
//
// One `forward` both renames the model (transform half) AND publishes the variable count under
// VAR_COUNT_NAME (analysis half) — the same slot VarCountAnalysis uses, so the existing
// TagWithCountPass consumer reads it back unchanged. It returns `{artifact_bytes, result_bytes}`:
// the inverse RenameArtifact for reversal, plus the VariableCount result. Reversible (identity on
// solution, since a rename does not touch the solution space).

class RenameAndCountPass : public lm::plugin::CompositePass {
  public:
    static constexpr const char *ID = "lm_cxx_plugin_ext::rename-and-count";

    std::string id() const override { return ID; }
    std::string name() const override { return "cxx rename and count"; }
    std::string provides() const override { return VAR_COUNT_NAME; }
    std::string result_type_tag() const override {
        return lm::plugin::type_tag<luna_cxx_plugin::VariableCount>();
    }

    std::pair<std::vector<std::uint8_t>, std::vector<std::uint8_t>>
    forward(lm::Model &model, const lm::plugin::PassContext &) override {
        // Transform half: rename the model, capturing the original name for reversal.
        const std::string original = model.name();
        model.set_name("renamed-and-counted:" + original);

        // Analysis half: count participating variables, published under VAR_COUNT_NAME.
        luna_cxx_plugin::VariableCount result;
        result.set_count(static_cast<std::uint64_t>(model.num_variables()));

        // Inverse artifact for the transform half.
        luna_cxx_plugin::RenameArtifact artifact;
        artifact.set_original_name(original);

        return {to_bytes(artifact), to_bytes(result)};
    }

    static lm::Solution backward(std::span<const std::uint8_t>, lm::Solution solution) {
        return solution;  // a rename leaves the solution unchanged
    }
};

// --- CountFollowingMeta: a META-ANALYSIS pass (reads no model) ---
//
// Unlike an analysis it introspects the pipeline steps that FOLLOW it (via the StepList facade) and
// publishes how many there are. The step view starts at the meta pass's OWN position — index 0 is
// this pass itself — so a `[CountFollowingMeta, consumer]` pipeline reports two steps. It reuses the
// VariableCount message (its `count` field carries the step count) and publishes under
// VAR_COUNT_NAME, so the existing TagWithCountPass consumer reads it back unchanged. Not reversible
// (no id / backward / register_backward).

class CountFollowingMeta : public lm::plugin::MetaAnalysisPass {
  public:
    std::string name() const override { return "cxx count following"; }
    std::string provides() const override { return VAR_COUNT_NAME; }
    std::string result_type_tag() const override {
        return lm::plugin::type_tag<luna_cxx_plugin::VariableCount>();
    }

    std::vector<std::uint8_t> run(const lm::plugin::StepList &steps) override {
        luna_cxx_plugin::VariableCount result;
        result.set_count(static_cast<std::uint64_t>(steps.size()));
        return to_bytes(result);
    }
};

// --- PickBranchPass: a CONTROL-FLOW pass (reads no payload, produces a plan) ---
//
// Unlike the other kinds it returns neither an artifact nor a serialized result: at `run` it reads
// the read-only model/context and builds a `ControlFlowPlan` — the pipeline of steps to execute
// next — here a single transform step reusing the existing RenamePass. The engine then runs that
// plan, so the model ends up renamed exactly as if RenamePass had run directly. Not itself
// reversible (its steps carry their own reversal — RenamePass's backward is registered below), so it
// has no id / backward / register_backward.

class PickBranchPass : public lm::plugin::ControlFlowPass {
  public:
    std::string name() const override { return "cxx pick branch"; }

    lm::plugin::ControlFlowPlan run(const lm::Model &, const lm::plugin::PassContext &) override {
        lm::plugin::ControlFlowPlan plan("chosen");
        plan.add_transform(std::make_unique<RenamePass>());
        return plan;
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

nb::object tag_with_min_constraint_capsule(const TagWithMinConstraintPass &) {
    return nb::steal(lm::plugin::python::transform_capsule(
        lm::plugin::make_descriptor(std::make_unique<TagWithMinConstraintPass>())));
}

nb::object rename_and_count_capsule(const RenameAndCountPass &) {
    return nb::steal(lm::plugin::python::composite_capsule(
        lm::plugin::make_descriptor(std::make_unique<RenameAndCountPass>())));
}

nb::object count_following_meta_capsule(const CountFollowingMeta &) {
    return nb::steal(lm::plugin::python::meta_analysis_capsule(
        lm::plugin::make_descriptor(std::make_unique<CountFollowingMeta>())));
}

nb::object pick_branch_capsule(const PickBranchPass &) {
    return nb::steal(lm::plugin::python::control_flow_capsule(
        lm::plugin::make_descriptor(std::make_unique<PickBranchPass>())));
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

    nb::class_<TagWithMinConstraintPass>(m, "TagWithMinConstraintPass")
        .def(nb::init<>())
        .def(lm::plugin::python::TRANSFORM_METHOD, &tag_with_min_constraint_capsule);

    nb::class_<RenameAndCountPass>(m, "RenameAndCountPass")
        .def(nb::init<>())
        .def(lm::plugin::python::COMPOSITE_METHOD, &rename_and_count_capsule);

    nb::class_<CountFollowingMeta>(m, "CountFollowingMeta")
        .def(nb::init<>())
        .def(lm::plugin::python::META_ANALYSIS_METHOD, &count_following_meta_capsule);

    nb::class_<PickBranchPass>(m, "PickBranchPass")
        .def(nb::init<>())
        .def(lm::plugin::python::CONTROL_FLOW_METHOD, &pick_branch_capsule);

    // Plain (non-plugin) C++ functions that receive / return luna_model Python objects directly,
    // via the nanobind type casters (luna_model/nanobind.hpp) — the general Python<->native interop.
    m.def("model_variable_count",
          [](lm::Model model) { return static_cast<std::uint64_t>(model.num_variables()); });
    m.def("make_named_model", [](const std::string &name) {
        lm::Model model;
        model.set_name(name);
        return model;
    });

    // Identity round-trips exercising the interop casters for every transferable type: each
    // compile-checks both directions (from_python on the argument, from_cpp on the return), and a
    // couple are round-tripped from Python in the tests.
    m.def("roundtrip_solution", [](lm::Solution s) { return s; });
    m.def("roundtrip_environment", [](lm::Env e) { return e; });
    m.def("roundtrip_expression", [](lm::Expression e) { return e; });
    m.def("roundtrip_constraint", [](lm::Constraint c) { return c; });
    m.def("roundtrip_constraint_collection", [](lm::ConstraintCollection cc) { return cc; });
    m.def("roundtrip_variable", [](lm::Vref v) { return v; });
    m.def("roundtrip_bounds", [](lm::LazyBounds b) { return b; });
    m.def("roundtrip_specs", [](lm::Specs s) { return s; });

    // Enum interop: Python luna_model.Vtype <-> C LmVtype (via the enum casters).
    m.def("vtype_value", [](LmVtype v) { return static_cast<int>(v); });
    m.def("binary_vtype", []() { return LmVtype_Binary; });

    // Invoked by `luna_model.register(module)`: install the transforms' reversal handlers.
    m.def("__luna_register__", []() {
        if (import_luna_model() != 0) {
            throw nb::python_error();
        }
        lm::plugin::register_backward<RenamePass>();
        lm::plugin::register_backward<TagWithCountPass>();
        lm::plugin::register_backward<TagWithMinConstraintPass>();
        lm::plugin::register_backward<RenameAndCountPass>();
    });
}
