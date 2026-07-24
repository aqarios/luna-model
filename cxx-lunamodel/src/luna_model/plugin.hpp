#ifndef LUNA_MODEL_PLUGIN_HPP
#define LUNA_MODEL_PLUGIN_HPP

// Authoring luna-model transpiler-pass **plugins** in C++.
//
// This is the C++ twin of `rs-lunamodel`'s `plugin` module: a header-only SDK for writing passes
// that luna-model calls OUT into (the inverse of the `LunaModelCApi` extension direction). A plugin
// author subclasses `lm::plugin::TransformationPass` / `lm::plugin::AnalysisPass`, and this header
// generates the C-ABI descriptor (`LmPluginTransform` / `LmPluginAnalysis`) and the reversal glue —
// so the author never touches raw handles, the byte sinks, or the `extern "C"` boundary.
//
// Payloads (transform artifacts, analysis results) cross the boundary as **protobuf-encoded bytes**,
// exactly as at the C ABI. The author encodes/decodes with their generated protobuf class (the
// `.proto` schemas ship in the `luna_model` wheel, discoverable via `luna_model.capi.get_proto()`),
// keeping the wire format identical across Rust / C++ / Python plugins.
//
// Lifecycle mirrors the native passes:
//   - forward: the model is lent as a live `LmModel` handle, mutated in place; the author returns
//     the inverse-artifact bytes.
//   - run (analysis): the model is lent read-only; the author returns the serialized result bytes,
//     deposited under `provides()` where later passes (in any language) read it back.
//   - backward: a `static` function on the pass, registered once via `register_backward<T>()`,
//     keyed by the pass id so reversal works even after the pass instance is gone.

#if defined(_MSVC_LANG) ? (_MSVC_LANG < 202002L) : (__cplusplus < 202002L)
#error "luna_model/plugin.hpp requires C++20 or newer"
#endif

#include <concepts>
#include <cstdint>
#include <cstdlib>
#include <memory>
#include <optional>
#include <span>
#include <string>
#include <vector>

#include <luna_model.h>

#include "luna_model/detail/capi.hpp"
#include "luna_model/error.hpp"
#include "luna_model/model.hpp"
#include "luna_model/solution.hpp"

namespace lm {
namespace plugin {

/// Read-only view of the per-pass context lent to a plugin during `forward`/`run`. It exposes the
/// analyses produced earlier in the same run (by built-ins or other plugins, in any language). The
/// context is borrowed for the duration of the call only — do not retain it.
class PassContext {
  public:
    explicit PassContext(const LmPassContext *raw) noexcept : raw_(raw) {}

    /// Returns the serialized (protobuf) bytes of the analysis stored under `name`, verifying its
    /// recorded type tag matches `type_tag`; `std::nullopt` if nothing is stored under `name`.
    /// Throws `lm::Error` if a result is present but was produced by a different type. The caller
    /// decodes the bytes with the matching protobuf class.
    std::optional<std::vector<std::uint8_t>> get_analysis(const std::string &name,
                                                          const std::string &type_tag) const {
        std::vector<std::uint8_t> buf;
        LmStatus status = lm_pass_context_get_analysis(
            raw_, name.c_str(), type_tag.c_str(),
            // Byte sink: append the emitted bytes to `buf`.
            [](void *ctx, const std::uint8_t *data, std::uintptr_t len) {
                if (ctx == nullptr || data == nullptr || len == 0) {
                    return;
                }
                auto *out = static_cast<std::vector<std::uint8_t> *>(ctx);
                out->insert(out->end(), data, data + len);
            },
            &buf);
        if (status == LmStatus_Absent) {
            return std::nullopt;
        }
        lm::detail::check(status);
        return buf;
    }

    /// Like `get_analysis`, but throws `lm::Error` if the analysis is absent. Use for a hard
    /// dependency the pass declared in `requires_analyses()`.
    std::vector<std::uint8_t> require_analysis(const std::string &name,
                                               const std::string &type_tag) const {
        std::optional<std::vector<std::uint8_t>> result = get_analysis(name, type_tag);
        if (!result) {
            throw Error(LmStatus_Absent, "required analysis '" + name + "' is not available");
        }
        return *result;
    }

  private:
    const LmPassContext *raw_;
};

/// Base class for a plugin **transformation** pass. Subclass it, implement the pure-virtual methods,
/// and additionally provide a `static` reversal function
///
///     static lm::Solution backward(std::span<const std::uint8_t> artifact, lm::Solution sol);
///     static constexpr const char *ID = "myplugin::my-pass";
///
/// where `ID` equals what `id()` returns (the `ReversiblePass` concept checks this shape). Register
/// the reversal once with `register_backward<T>()`, then hand `make_descriptor(std::make_unique<T>())`
/// to the host.
class TransformationPass {
  public:
    virtual ~TransformationPass() = default;

    /// Stable id routing reversal to this pass's `backward` (e.g. `"myplugin::relabel"`).
    virtual std::string id() const = 0;
    /// Human-readable pass name.
    virtual std::string name() const = 0;
    /// Names of analyses this pass reads via the `PassContext` (empty by default).
    virtual std::vector<std::string> requires_analyses() const { return {}; }
    /// Names of analyses this pass invalidates (empty by default).
    virtual std::vector<std::string> invalidates() const { return {}; }
    /// Mutates the lent `model` in place and returns the protobuf-encoded inverse artifact.
    virtual std::vector<std::uint8_t> forward(Model &model, const PassContext &ctx) = 0;
};

/// Base class for a plugin **analysis** pass. Its serialized result is deposited under `provides()`
/// in the run's analysis store, where later passes — including other plugins, in any language —
/// read it back by that name (tag-checked against `result_type_tag()`).
class AnalysisPass {
  public:
    virtual ~AnalysisPass() = default;

    /// Stable pass id (e.g. `"myplugin::coeff-widths-analysis"`).
    virtual std::string id() const = 0;
    /// Human-readable pass name.
    virtual std::string name() const = 0;
    /// Name the result is stored under; consumers read it back with this name.
    virtual std::string provides() const = 0;
    /// Stable type tag of the produced result, verified when a consumer decodes it.
    virtual std::string result_type_tag() const = 0;
    /// Names of analyses this pass reads via the `PassContext` (empty by default).
    virtual std::vector<std::string> requires_analyses() const { return {}; }
    /// Computes the result from the lent read-only `model` and returns its protobuf-encoded bytes.
    virtual std::vector<std::uint8_t> run(const Model &model, const PassContext &ctx) = 0;
};

namespace detail {

/// Boxed transform state behind the descriptor's `user_data`, caching the id/name strings so their
/// trampolines can hand out stable borrowed pointers.
struct TransformState {
    std::unique_ptr<TransformationPass> pass;
    std::string id;
    std::string name;
};

/// Boxed analysis state, caching the id/name/provides/result-tag strings.
struct AnalysisState {
    std::unique_ptr<AnalysisPass> pass;
    std::string id;
    std::string name;
    std::string provides;
    std::string result_type_tag;
};

/// Signature of the author's `static backward`. Reversal takes ownership of the incoming solution
/// and returns the mapped-back one.
using BackwardFn = Solution (*)(std::span<const std::uint8_t>, Solution);

}  // namespace detail

/// A transformation pass that can be reversed: it provides a stable `ID` and a matching `static
/// backward`. `register_backward<T>()` requires this so the reversal wiring is checked at compile
/// time with a readable error rather than deep in a template.
template <typename T>
concept ReversiblePass =
    std::derived_from<T, TransformationPass> &&
    requires(std::span<const std::uint8_t> artifact, Solution solution) {
        { T::ID } -> std::convertible_to<const char *>;
        { T::backward(artifact, std::move(solution)) } -> std::same_as<Solution>;
    };

// The captureless lambdas below are assigned to the descriptor's C callback slots; each carries no
// state and dispatches through `user_data`, so nothing is captured across the ABI.

/// Builds the C-ABI descriptor for a plugin transformation pass. The host copies the returned
/// struct, takes ownership of its `user_data`, and releases it via `free` when the pass is dropped.
inline LmPluginTransform make_descriptor(std::unique_ptr<TransformationPass> pass) {
    std::string id = pass->id();
    std::string name = pass->name();
    auto *state = new detail::TransformState{std::move(pass), std::move(id), std::move(name)};

    return LmPluginTransform{
        .user_data = state,
        .id = [](void *ud) -> const char * {
            return static_cast<detail::TransformState *>(ud)->id.c_str();
        },
        .name = [](void *ud) -> const char * {
            return static_cast<detail::TransformState *>(ud)->name.c_str();
        },
        .requires_analyses =
            [](void *ud, LmStrSink sink, void *sink_ctx) {
                for (const auto &s :
                     static_cast<detail::TransformState *>(ud)->pass->requires_analyses()) {
                    sink(sink_ctx, s.c_str());
                }
            },
        .invalidates =
            [](void *ud, LmStrSink sink, void *sink_ctx) {
                for (const auto &s :
                     static_cast<detail::TransformState *>(ud)->pass->invalidates()) {
                    sink(sink_ctx, s.c_str());
                }
            },
        .forward =
            [](void *ud, LmModel *model, const LmPassContext *ctx, LmByteSink sink,
               void *sink_ctx) -> LmStatus {
            if (model == nullptr) {
                return LmStatus_NullPointer;
            }
            auto *st = static_cast<detail::TransformState *>(ud);
            // Adopt the lent handle so the pass can mutate it via the ordinary `lm_*` API, then
            // relinquish it (do NOT free — the host owns it) whether or not `forward` throws.
            Model borrowed(model);
            try {
                PassContext pass_ctx(ctx);
                std::vector<std::uint8_t> bytes = st->pass->forward(borrowed, pass_ctx);
                borrowed.release();
                sink(sink_ctx, bytes.data(), bytes.size());
                return LmStatus_Ok;
            } catch (...) {
                borrowed.release();
                return LmStatus_Errored;
            }
        },
        .free = [](void *ud) { delete static_cast<detail::TransformState *>(ud); },
    };
}

/// Builds the C-ABI descriptor for a plugin analysis pass. The host copies the returned struct,
/// takes ownership of its `user_data`, and releases it via `free` when the pass is dropped.
inline LmPluginAnalysis make_descriptor(std::unique_ptr<AnalysisPass> pass) {
    std::string id = pass->id();
    std::string name = pass->name();
    std::string provides = pass->provides();
    std::string result_type_tag = pass->result_type_tag();
    auto *state = new detail::AnalysisState{std::move(pass), std::move(id), std::move(name),
                                            std::move(provides), std::move(result_type_tag)};

    return LmPluginAnalysis{
        .user_data = state,
        .id = [](void *ud) -> const char * {
            return static_cast<detail::AnalysisState *>(ud)->id.c_str();
        },
        .name = [](void *ud) -> const char * {
            return static_cast<detail::AnalysisState *>(ud)->name.c_str();
        },
        .provides = [](void *ud) -> const char * {
            return static_cast<detail::AnalysisState *>(ud)->provides.c_str();
        },
        .result_type_tag = [](void *ud) -> const char * {
            return static_cast<detail::AnalysisState *>(ud)->result_type_tag.c_str();
        },
        .requires_analyses =
            [](void *ud, LmStrSink sink, void *sink_ctx) {
                for (const auto &s :
                     static_cast<detail::AnalysisState *>(ud)->pass->requires_analyses()) {
                    sink(sink_ctx, s.c_str());
                }
            },
        .run =
            [](void *ud, const LmModel *model, const LmPassContext *ctx, LmByteSink sink,
               void *sink_ctx) -> LmStatus {
            if (model == nullptr) {
                return LmStatus_NullPointer;
            }
            auto *st = static_cast<detail::AnalysisState *>(ud);
            // The read-only handle is lent; adopt it to read via `lm_*`, then relinquish w/o free.
            Model borrowed(const_cast<LmModel *>(model));
            try {
                PassContext pass_ctx(ctx);
                std::vector<std::uint8_t> bytes = st->pass->run(borrowed, pass_ctx);
                borrowed.release();
                sink(sink_ctx, bytes.data(), bytes.size());
                return LmStatus_Ok;
            } catch (...) {
                borrowed.release();
                return LmStatus_Errored;
            }
        },
        .free = [](void *ud) { delete static_cast<detail::AnalysisState *>(ud); },
    };
}

/// Registers `T`'s reversal handler with the host, keyed by `T::ID`. Call once per pass type
/// (typically from the plugin's registration hook). `T` must provide
/// `static constexpr const char *ID` and a matching `static lm::Solution backward(...)`.
///
/// The handler must stay valid for as long as any record produced by the pass may be reversed
/// (typically the process lifetime), so the boxed function pointer is intentionally never freed.
template <ReversiblePass T>
void register_backward() {
    auto *boxed = new detail::BackwardFn(&T::backward);
    LmBackwardFn dispatch = [](void *user_data, const std::uint8_t *artifact, std::uintptr_t len,
                               LmSol *solution, LmSol **out_solution) -> LmStatus {
        detail::BackwardFn impl = *static_cast<detail::BackwardFn *>(user_data);
        // View the host's artifact buffer directly (valid for this call — no copy).
        std::span<const std::uint8_t> bytes =
            (artifact != nullptr && len > 0) ? std::span<const std::uint8_t>(artifact, len)
                                             : std::span<const std::uint8_t>{};
        // Adopt the incoming solution up front so it is freed on any early return.
        Solution incoming(solution);
        try {
            Solution result = impl(bytes, std::move(incoming));
            *out_solution = result.release();
            return LmStatus_Ok;
        } catch (...) {
            return LmStatus_Errored;
        }
    };
    lm::detail::check(
        lm_register_plugin_backward(LmPassKind_Transformation, T::ID, dispatch, boxed));
}

#ifdef LUNA_MODEL_PYTHON_EXTENSION
// Python exposure. Available only when building a Python **extension** (the umbrella defines
// `LUNA_MODEL_PYTHON_EXTENSION`, pulls in `<Python.h>`, and dispatches the `lm_*` calls through the
// imported `_C_API` table). A C++ pass reaches the host `PassManager` by exposing a zero-argument
// method returning one of these capsules — `_luna_plugin_transform_capsule` / the analysis twin —
// and by registering its reversal handlers from a module-level `__luna_register__()` hook (which
// calls `import_luna_model()` + `register_backward<T>()`). The capsule protocol below is
// byte-compatible with the one the pyo3 SDK produces, so both plug into the same host code.
namespace python {

/// Capsule name / method name the host expects for a transformation pass. Must stay in lockstep
/// with `crates/capi-pyffi/src/plugin.rs`.
inline constexpr const char *TRANSFORM_CAPSULE_NAME = "luna_model.capi.PluginTransform.v1";
inline constexpr const char *TRANSFORM_METHOD = "_luna_plugin_transform_capsule";
/// Capsule name / method name the host expects for an analysis pass.
inline constexpr const char *ANALYSIS_CAPSULE_NAME = "luna_model.capi.PluginAnalysis.v1";
inline constexpr const char *ANALYSIS_METHOD = "_luna_plugin_analysis_capsule";

namespace detail {

inline void free_transform_capsule(PyObject *capsule) {
    // Free only the descriptor storage. The host takes ownership of `user_data` when it copies the
    // descriptor out (`read_plugin_transform`), releasing it via `free` when the pass drops — so it
    // must NOT be freed here (mirrors the Rust carrier's no-op drop of a `Copy` descriptor).
    std::free(PyCapsule_GetPointer(capsule, TRANSFORM_CAPSULE_NAME));
}

inline void free_analysis_capsule(PyObject *capsule) {
    std::free(PyCapsule_GetPointer(capsule, ANALYSIS_CAPSULE_NAME));
}

}  // namespace detail

/// Wraps a transformation descriptor **by value** in the capsule the host reads from the
/// `_luna_plugin_transform_capsule()` method. Returns a new reference, or `nullptr` with a Python
/// exception set. Ownership of `descriptor.user_data` transfers to the host on read, so build a
/// fresh descriptor (`make_descriptor(...)`) per call.
inline PyObject *transform_capsule(const LmPluginTransform &descriptor) {
    auto *stored = static_cast<LmPluginTransform *>(std::malloc(sizeof(LmPluginTransform)));
    if (stored == nullptr) {
        return PyErr_NoMemory();
    }
    *stored = descriptor;
    PyObject *capsule =
        PyCapsule_New(stored, TRANSFORM_CAPSULE_NAME, &detail::free_transform_capsule);
    if (capsule == nullptr) {
        std::free(stored);
    }
    return capsule;
}

/// Wraps an analysis descriptor by value in the capsule the host reads from
/// `_luna_plugin_analysis_capsule()`. See `transform_capsule` for ownership.
inline PyObject *analysis_capsule(const LmPluginAnalysis &descriptor) {
    auto *stored = static_cast<LmPluginAnalysis *>(std::malloc(sizeof(LmPluginAnalysis)));
    if (stored == nullptr) {
        return PyErr_NoMemory();
    }
    *stored = descriptor;
    PyObject *capsule =
        PyCapsule_New(stored, ANALYSIS_CAPSULE_NAME, &detail::free_analysis_capsule);
    if (capsule == nullptr) {
        std::free(stored);
    }
    return capsule;
}

}  // namespace python
#endif  // LUNA_MODEL_PYTHON_EXTENSION

}  // namespace plugin
}  // namespace lm

#endif  // LUNA_MODEL_PLUGIN_HPP
