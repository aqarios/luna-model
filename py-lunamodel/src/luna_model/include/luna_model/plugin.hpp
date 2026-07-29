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
// Payloads (transform artifacts, analysis results) cross the boundary as **protobuf-encoded
// bytes**, exactly as at the C ABI. The author encodes/decodes with their generated protobuf class
// (the
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
#include <utility>
#include <vector>

#include <luna_model.h>

#include "luna_model/detail/capi.hpp"
#include "luna_model/error.hpp"
#include "luna_model/model.hpp"
#include "luna_model/solution.hpp"

namespace lm {
namespace plugin {

/// A protobuf message — one that can be parsed from bytes and knows its own fully-qualified name.
/// Constrains the typed analysis accessors so passing a non-message type fails with a clear error
/// at the call site rather than deep inside a template.
template <typename T>
concept ProtoMessage = requires(T message, const void *data, int size) {
    { message.ParseFromArray(data, size) } -> std::convertible_to<bool>;
    // `full_name()` is string-like (its exact type — `const std::string&` vs `absl::string_view` —
    // varies by protobuf version), so only require the accessor exists; `type_tag` builds a string.
    T::descriptor()->full_name();
};

/// The type tag luna-model uses for a result message: its protobuf fully-qualified name (e.g.
/// `lunamodel.plugin.MinConstraintValues`). This is derived from the type, so it never has to be
/// written by hand and matches across languages (Rust registers the same FQN). A producer returns
/// it from `AnalysisPass::result_type_tag()`; the typed `require_analysis<Message>` supplies it for
/// a consumer implicitly.
template <ProtoMessage Message> inline std::string type_tag() {
    // Construct explicitly: `full_name()` may be `absl::string_view`, which is not implicitly
    // convertible to `std::string`.
    return std::string(Message::descriptor()->full_name());
}

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

    /// Reads the analysis stored under `name` and decodes it into the protobuf message `Message`.
    /// `std::nullopt` if absent; throws `lm::Error` on a decode failure. The type tag is derived
    /// from `Message` (its fully-qualified name), so only the name is passed:
    /// `ctx.get_analysis<MyResult>(name)`.
    template <ProtoMessage Message>
    std::optional<Message> get_analysis(const std::string &name) const {
        std::optional<std::vector<std::uint8_t>> bytes = get_analysis(name, type_tag<Message>());
        if (!bytes) {
            return std::nullopt;
        }
        return decode<Message>(*bytes, name);
    }

    /// Like `get_analysis<Message>`, but throws `lm::Error` if the analysis is absent. The typed
    /// counterpart to the byte `require_analysis`: `ctx.require_analysis<MyResult>(name)`.
    template <ProtoMessage Message> Message require_analysis(const std::string &name) const {
        return decode<Message>(require_analysis(name, type_tag<Message>()), name);
    }

  private:
    /// Parses `bytes` into a fresh `Message`, throwing `lm::Error` if the wire data is malformed.
    template <ProtoMessage Message>
    static Message decode(const std::vector<std::uint8_t> &bytes, const std::string &name) {
        Message message;
        if (!message.ParseFromArray(bytes.data(), static_cast<int>(bytes.size()))) {
            throw Error(LmStatus_Deserialization, "failed to decode analysis '" + name + "'");
        }
        return message;
    }

    const LmPassContext *raw_;
};

/// Read-only view of the pipeline steps that follow a meta-analysis pass, lent to it during
/// `MetaAnalysisPass::run`. A 1:1 mirror of the `rs-lunamodel` `StepList`/`Step` facade over the
/// `step_list_*` API-table entries: the count, and per step its `LmPassKind`, name, and the analysis
/// names it requires / provides / invalidates. Matching the native semantics, the view starts at the
/// meta pass's OWN position — so index 0 is the meta pass itself, index 1 the next step, and so on.
/// Borrowed for the duration of the call only — do not retain it.
class StepList {
  public:
    explicit StepList(const LmStepList *raw) noexcept : raw_(raw) {}

    /// Number of steps in the view (the meta pass itself plus those following it).
    std::size_t size() const {
        std::uintptr_t out = 0;
        lm::detail::check(lm_step_list_len(raw_, &out));
        return static_cast<std::size_t>(out);
    }

    /// Whether the view is empty.
    bool empty() const { return size() == 0; }

    /// Kind of the step at `index`.
    LmPassKind kind(std::size_t index) const {
        LmPassKind out = LmPassKind_Transformation;
        lm::detail::check(lm_step_list_kind(raw_, index, &out));
        return out;
    }

    /// Human-readable name of the step at `index`.
    std::string name(std::size_t index) const {
        std::string out;
        lm::detail::check(lm_step_list_name(
            raw_, index,
            // String sink: capture the single emitted name into `out`.
            [](void *ctx, const char *s) {
                if (ctx == nullptr || s == nullptr) {
                    return;
                }
                *static_cast<std::string *>(ctx) = s;
            },
            &out));
        return out;
    }

    /// Analysis names the step at `index` requires.
    std::vector<std::string> requires_analyses(std::size_t index) const {
        return collect([&](LmStrSink sink, void *ctx) {
            return lm_step_list_requires(raw_, index, sink, ctx);
        });
    }

    /// Analysis names the step at `index` provides.
    std::vector<std::string> provides(std::size_t index) const {
        return collect([&](LmStrSink sink, void *ctx) {
            return lm_step_list_provides(raw_, index, sink, ctx);
        });
    }

    /// Analysis names the step at `index` invalidates.
    std::vector<std::string> invalidates(std::size_t index) const {
        return collect([&](LmStrSink sink, void *ctx) {
            return lm_step_list_invalidates(raw_, index, sink, ctx);
        });
    }

  private:
    /// Drives a string-emitting `step_list_*` call, collecting the emitted strings into a vector.
    /// The `LmStrSink` is captureless (same style as `PassContext::get_analysis`'s byte sink),
    /// appending each NUL-terminated string to the `std::vector` handed through `ctx`.
    template <typename Call> static std::vector<std::string> collect(Call &&call) {
        std::vector<std::string> out;
        lm::detail::check(call(
            [](void *ctx, const char *s) {
                if (ctx == nullptr || s == nullptr) {
                    return;
                }
                static_cast<std::vector<std::string> *>(ctx)->emplace_back(s);
            },
            &out));
        return out;
    }

    const LmStepList *raw_;
};

/// Base class for a plugin **transformation** pass. Subclass it, implement the pure-virtual
/// methods, and additionally provide a `static` reversal function
///
///     static lm::Solution backward(std::span<const std::uint8_t> artifact, lm::Solution sol);
///     static constexpr const char *ID = "myplugin::my-pass";
///
/// where `ID` equals what `id()` returns (the `ReversiblePass` concept checks this shape). Register
/// the reversal once with `register_backward<T>()`, then hand
/// `make_descriptor(std::make_unique<T>())` to the host.
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

/// Base class for a plugin **composite** pass — a fused transformation and analysis. A single
/// `forward` both mutates the lent model (like a transform) and produces an analysis result
/// deposited under `provides()` (like an analysis), returning `{artifact_bytes, result_bytes}`.
///
/// Like a transformation it is reversible: additionally provide a `static` reversal function and a
/// matching `ID` (the `ReversibleCompositePass` concept checks this shape), register it once with
/// `register_backward<T>()`, then hand `make_descriptor(std::make_unique<T>())` to the host.
///
///     static lm::Solution backward(std::span<const std::uint8_t> artifact, lm::Solution sol);
///     static constexpr const char *ID = "myplugin::my-composite";
class CompositePass {
  public:
    virtual ~CompositePass() = default;

    /// Stable id routing reversal to this pass's `backward` (e.g. `"myplugin::relabel-and-count"`).
    virtual std::string id() const = 0;
    /// Human-readable pass name.
    virtual std::string name() const = 0;
    /// Name the analysis result is stored under; consumers read it back with this name.
    virtual std::string provides() const = 0;
    /// Stable type tag of the produced result, verified when a consumer decodes it.
    virtual std::string result_type_tag() const = 0;
    /// Names of analyses this pass reads via the `PassContext` (empty by default).
    virtual std::vector<std::string> requires_analyses() const { return {}; }
    /// Names of analyses this pass invalidates (empty by default).
    virtual std::vector<std::string> invalidates() const { return {}; }
    /// Mutates the lent `model` in place and returns `{inverse-artifact bytes, result bytes}`, both
    /// protobuf-encoded.
    virtual std::pair<std::vector<std::uint8_t>, std::vector<std::uint8_t>>
    forward(Model &model, const PassContext &ctx) = 0;
};

/// Base class for a plugin **meta-analysis** pass — an analysis *about the pipeline itself*. Unlike
/// an ordinary analysis it reads no model; instead it introspects the steps that follow it (via the
/// lent `StepList`, whose index 0 is this pass itself) and produces a result deposited under
/// `provides()`, where later passes read it back (tag-checked against `result_type_tag()`). It is
/// NOT reversible — so there is no `id()` (the descriptor reuses the name) and no `backward`/
/// `register_backward`; just hand `make_descriptor(std::make_unique<T>())` to the host.
class MetaAnalysisPass {
  public:
    virtual ~MetaAnalysisPass() = default;

    /// Human-readable pass name.
    virtual std::string name() const = 0;
    /// Name the result is stored under; consumers read it back with this name.
    virtual std::string provides() const = 0;
    /// Stable type tag of the produced result, verified when a consumer decodes it.
    virtual std::string result_type_tag() const = 0;
    /// Computes the result from the read-only view of the following steps and returns its
    /// protobuf-encoded bytes.
    virtual std::vector<std::uint8_t> run(const StepList &steps) = 0;
};

// Control-flow needs a small definition-order dance: `ControlFlowPlan::add_*` call the
// `make_descriptor` overloads, `ControlFlowPass::run` returns a `ControlFlowPlan`, and
// `make_descriptor(std::unique_ptr<ControlFlowPass>)` uses both. Resolve it by forward-declaring the
// pass here, defining the plan builder then the pass base, and defining the plan's `add_*`
// out-of-line further down — once every `make_descriptor` overload is in scope.
class ControlFlowPass;

/// The pipeline of steps a control-flow pass selects, built up with the `add_*` methods. A 1:1
/// facade over the native `ControlFlowPlan`, across the C ABI: the host owns the plan behind an
/// opaque `LmControlFlowPlan` handle, and each `add_*` wraps a pass (of any kind — including a
/// nested control-flow pass) into the next step, exactly as building a native pipeline. This object
/// owns the handle and frees it on destruction, unless `release()` hands it back to the host.
/// Reversible steps still register their own backward handler, as any plugin pass does.
///
/// Non-copyable; move-only.
class ControlFlowPlan {
  public:
    /// Creates a new, empty plan named `name`.
    explicit ControlFlowPlan(const std::string &name) {
        lm::detail::check(lm_control_flow_plan_new(name.c_str(), &raw_));
    }

    ControlFlowPlan(const ControlFlowPlan &) = delete;
    ControlFlowPlan &operator=(const ControlFlowPlan &) = delete;
    ControlFlowPlan(ControlFlowPlan &&other) noexcept : raw_(other.raw_) { other.raw_ = nullptr; }
    ControlFlowPlan &operator=(ControlFlowPlan &&other) noexcept {
        if (this != &other) {
            if (raw_ != nullptr) {
                lm_control_flow_plan_free(raw_);
            }
            raw_ = other.raw_;
            other.raw_ = nullptr;
        }
        return *this;
    }

    /// Frees an unconsumed plan (e.g. if the pass throws after building one); once `release()`d the
    /// host owns the handle and this is a no-op.
    ~ControlFlowPlan() {
        if (raw_ != nullptr) {
            lm_control_flow_plan_free(raw_);
        }
    }

    /// Appends a transformation pass as the next step.
    void add_transform(std::unique_ptr<TransformationPass> pass);
    /// Appends an analysis pass as the next step.
    void add_analysis(std::unique_ptr<AnalysisPass> pass);
    /// Appends a composite pass as the next step.
    void add_composite(std::unique_ptr<CompositePass> pass);
    /// Appends a (nested) control-flow pass as the next step.
    void add_control_flow(std::unique_ptr<ControlFlowPass> pass);
    /// Appends a meta-analysis pass as the next step.
    void add_meta_analysis(std::unique_ptr<MetaAnalysisPass> pass);

    /// Relinquishes ownership of the raw handle to the host (which then owns and frees it). Used by
    /// the `run` trampoline to hand the plan back across the ABI.
    LmControlFlowPlan *release() noexcept {
        LmControlFlowPlan *plan = raw_;
        raw_ = nullptr;
        return plan;
    }

  private:
    LmControlFlowPlan *raw_ = nullptr;
};

/// Base class for a plugin **control-flow** pass. At `run`, the pass inspects the read-only model
/// (and the analyses available so far) and builds a `ControlFlowPlan`: the pipeline of steps to
/// execute next. It is NOT itself reversible — the steps it selects carry their own reversal — so
/// there is no `id()` (the descriptor reuses the name) and no `backward`/`register_backward`; just
/// hand `make_descriptor(std::make_unique<T>())` to the host.
class ControlFlowPass {
  public:
    virtual ~ControlFlowPass() = default;

    /// Human-readable pass name (also serves as the id — a control-flow pass is not reversible).
    virtual std::string name() const = 0;
    /// Names of analyses this pass reads via the `PassContext` (empty by default).
    virtual std::vector<std::string> requires_analyses() const { return {}; }
    /// Names of analyses this pass provides (empty by default).
    virtual std::vector<std::string> provides() const { return {}; }
    /// Names of analyses this pass invalidates (empty by default).
    virtual std::vector<std::string> invalidates() const { return {}; }
    /// Builds the plan of steps to execute next from the lent read-only `model` and `ctx`.
    virtual ControlFlowPlan run(const Model &model, const PassContext &ctx) = 0;
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

/// Boxed composite state, caching the id/name/provides/result-tag strings.
struct CompositeState {
    std::unique_ptr<CompositePass> pass;
    std::string id;
    std::string name;
    std::string provides;
    std::string result_type_tag;
};

/// Boxed meta-analysis state, caching the name/provides/result-tag strings. No `id` — a
/// meta-analysis is not reversible, so the descriptor's `id` reuses the cached `name`.
struct MetaAnalysisState {
    std::unique_ptr<MetaAnalysisPass> pass;
    std::string name;
    std::string provides;
    std::string result_type_tag;
};

/// Boxed control-flow state, caching only the name string. No `id`/`provides`/`result_type_tag` — a
/// control-flow pass is not reversible (the descriptor's `id` reuses the cached `name`) and produces
/// a plan, not a serialized result.
struct ControlFlowState {
    std::unique_ptr<ControlFlowPass> pass;
    std::string name;
};

/// Signature of the author's `static backward`. Reversal takes ownership of the incoming solution
/// and returns the mapped-back one.
using BackwardFn = Solution (*)(std::span<const std::uint8_t>, Solution);

} // namespace detail

/// A transformation pass that can be reversed: it provides a stable `ID` and a matching `static
/// backward`. `register_backward<T>()` requires this so the reversal wiring is checked at compile
/// time with a readable error rather than deep in a template.
template <typename T>
concept ReversiblePass = std::derived_from<T, TransformationPass> &&
                         requires(std::span<const std::uint8_t> artifact, Solution solution) {
                             { T::ID } -> std::convertible_to<const char *>;
                             {
                                 T::backward(artifact, std::move(solution))
                             } -> std::same_as<Solution>;
                         };

/// A composite pass that can be reversed — the composite twin of [`ReversiblePass`]. Composites are
/// reversible too, so `register_backward<T>()` accepts them via this overload.
template <typename T>
concept ReversibleCompositePass =
    std::derived_from<T, CompositePass> &&
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
        .forward = [](void *ud, LmModel *model, const LmPassContext *ctx, LmByteSink sink,
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
        .run = [](void *ud, const LmModel *model, const LmPassContext *ctx, LmByteSink sink,
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

/// Builds the C-ABI descriptor for a plugin composite pass. The host copies the returned struct,
/// takes ownership of its `user_data`, and releases it via `free` when the pass is dropped.
inline LmPluginComposite make_descriptor(std::unique_ptr<CompositePass> pass) {
    std::string id = pass->id();
    std::string name = pass->name();
    std::string provides = pass->provides();
    std::string result_type_tag = pass->result_type_tag();
    auto *state = new detail::CompositeState{std::move(pass), std::move(id), std::move(name),
                                             std::move(provides), std::move(result_type_tag)};

    return LmPluginComposite{
        .user_data = state,
        .id = [](void *ud) -> const char * {
            return static_cast<detail::CompositeState *>(ud)->id.c_str();
        },
        .name = [](void *ud) -> const char * {
            return static_cast<detail::CompositeState *>(ud)->name.c_str();
        },
        .provides = [](void *ud) -> const char * {
            return static_cast<detail::CompositeState *>(ud)->provides.c_str();
        },
        .result_type_tag = [](void *ud) -> const char * {
            return static_cast<detail::CompositeState *>(ud)->result_type_tag.c_str();
        },
        .requires_analyses =
            [](void *ud, LmStrSink sink, void *sink_ctx) {
                for (const auto &s :
                     static_cast<detail::CompositeState *>(ud)->pass->requires_analyses()) {
                    sink(sink_ctx, s.c_str());
                }
            },
        .invalidates =
            [](void *ud, LmStrSink sink, void *sink_ctx) {
                for (const auto &s :
                     static_cast<detail::CompositeState *>(ud)->pass->invalidates()) {
                    sink(sink_ctx, s.c_str());
                }
            },
        .forward = [](void *ud, LmModel *model, const LmPassContext *ctx, LmByteSink artifact_sink,
                      void *artifact_ctx, LmByteSink result_sink, void *result_ctx) -> LmStatus {
            if (model == nullptr) {
                return LmStatus_NullPointer;
            }
            auto *st = static_cast<detail::CompositeState *>(ud);
            // Adopt the lent handle so the pass can mutate it via the ordinary `lm_*` API, then
            // relinquish it (do NOT free — the host owns it) whether or not `forward` throws.
            Model borrowed(model);
            try {
                PassContext pass_ctx(ctx);
                auto [artifact, result] = st->pass->forward(borrowed, pass_ctx);
                borrowed.release();
                // Emit the two payloads independently: inverse artifact, then analysis result.
                artifact_sink(artifact_ctx, artifact.data(), artifact.size());
                result_sink(result_ctx, result.data(), result.size());
                return LmStatus_Ok;
            } catch (...) {
                borrowed.release();
                return LmStatus_Errored;
            }
        },
        .free = [](void *ud) { delete static_cast<detail::CompositeState *>(ud); },
    };
}

/// Builds the C-ABI descriptor for a plugin meta-analysis pass. The host copies the returned struct,
/// takes ownership of its `user_data`, and releases it via `free` when the pass is dropped.
inline LmPluginMetaAnalysis make_descriptor(std::unique_ptr<MetaAnalysisPass> pass) {
    std::string name = pass->name();
    std::string provides = pass->provides();
    std::string result_type_tag = pass->result_type_tag();
    auto *state = new detail::MetaAnalysisState{std::move(pass), std::move(name), std::move(provides),
                                                std::move(result_type_tag)};

    return LmPluginMetaAnalysis{
        .user_data = state,
        // A meta-analysis is not reversible, so its id never routes reversal; reuse the cached name
        // (records identify meta passes by name), matching the Rust SDK.
        .id = [](void *ud) -> const char * {
            return static_cast<detail::MetaAnalysisState *>(ud)->name.c_str();
        },
        .name = [](void *ud) -> const char * {
            return static_cast<detail::MetaAnalysisState *>(ud)->name.c_str();
        },
        .provides = [](void *ud) -> const char * {
            return static_cast<detail::MetaAnalysisState *>(ud)->provides.c_str();
        },
        .result_type_tag = [](void *ud) -> const char * {
            return static_cast<detail::MetaAnalysisState *>(ud)->result_type_tag.c_str();
        },
        .run = [](void *ud, const LmStepList *steps, LmByteSink sink, void *sink_ctx) -> LmStatus {
            auto *st = static_cast<detail::MetaAnalysisState *>(ud);
            // The step view is lent for the call only; wrap it and run the safe `run`.
            try {
                StepList view(steps);
                std::vector<std::uint8_t> bytes = st->pass->run(view);
                sink(sink_ctx, bytes.data(), bytes.size());
                return LmStatus_Ok;
            } catch (...) {
                return LmStatus_Errored;
            }
        },
        .free = [](void *ud) { delete static_cast<detail::MetaAnalysisState *>(ud); },
    };
}

/// Builds the C-ABI descriptor for a plugin control-flow pass. The host copies the returned struct,
/// takes ownership of its `user_data`, and releases it via `free` when the pass is dropped.
inline LmPluginControlFlow make_descriptor(std::unique_ptr<ControlFlowPass> pass) {
    std::string name = pass->name();
    auto *state = new detail::ControlFlowState{std::move(pass), std::move(name)};

    return LmPluginControlFlow{
        .user_data = state,
        // A control-flow pass is not reversible, so its id never routes reversal; reuse the cached
        // name (records identify control-flow passes by name), matching the Rust SDK.
        .id = [](void *ud) -> const char * {
            return static_cast<detail::ControlFlowState *>(ud)->name.c_str();
        },
        .name = [](void *ud) -> const char * {
            return static_cast<detail::ControlFlowState *>(ud)->name.c_str();
        },
        .requires_analyses =
            [](void *ud, LmStrSink sink, void *sink_ctx) {
                for (const auto &s :
                     static_cast<detail::ControlFlowState *>(ud)->pass->requires_analyses()) {
                    sink(sink_ctx, s.c_str());
                }
            },
        .provides =
            [](void *ud, LmStrSink sink, void *sink_ctx) {
                for (const auto &s :
                     static_cast<detail::ControlFlowState *>(ud)->pass->provides()) {
                    sink(sink_ctx, s.c_str());
                }
            },
        .invalidates =
            [](void *ud, LmStrSink sink, void *sink_ctx) {
                for (const auto &s :
                     static_cast<detail::ControlFlowState *>(ud)->pass->invalidates()) {
                    sink(sink_ctx, s.c_str());
                }
            },
        .run = [](void *ud, const LmModel *model, const LmPassContext *ctx,
                  LmControlFlowPlan **out_plan) -> LmStatus {
            if (model == nullptr) {
                return LmStatus_NullPointer;
            }
            auto *st = static_cast<detail::ControlFlowState *>(ud);
            // The read-only handle is lent; adopt it to read via `lm_*`, then relinquish w/o free.
            Model borrowed(const_cast<LmModel *>(model));
            try {
                PassContext pass_ctx(ctx);
                ControlFlowPlan plan = st->pass->run(borrowed, pass_ctx);
                borrowed.release();
                // Hand ownership of the built plan to the host through the out-parameter, only on
                // the success path (leave `out_plan` untouched if `run` threw).
                *out_plan = plan.release();
                return LmStatus_Ok;
            } catch (...) {
                borrowed.release();
                return LmStatus_Errored;
            }
        },
        .free = [](void *ud) { delete static_cast<detail::ControlFlowState *>(ud); },
    };
}

// `ControlFlowPlan::add_*` are defined out-of-line here, now that every `make_descriptor` overload
// (including the control-flow one above) is in scope: each builds the matching descriptor and
// appends it as the next step, checking the status.
inline void ControlFlowPlan::add_transform(std::unique_ptr<TransformationPass> pass) {
    LmPluginTransform desc = make_descriptor(std::move(pass));
    lm::detail::check(lm_control_flow_plan_add_transform(raw_, &desc));
}

inline void ControlFlowPlan::add_analysis(std::unique_ptr<AnalysisPass> pass) {
    LmPluginAnalysis desc = make_descriptor(std::move(pass));
    lm::detail::check(lm_control_flow_plan_add_analysis(raw_, &desc));
}

inline void ControlFlowPlan::add_composite(std::unique_ptr<CompositePass> pass) {
    LmPluginComposite desc = make_descriptor(std::move(pass));
    lm::detail::check(lm_control_flow_plan_add_composite(raw_, &desc));
}

inline void ControlFlowPlan::add_control_flow(std::unique_ptr<ControlFlowPass> pass) {
    LmPluginControlFlow desc = make_descriptor(std::move(pass));
    lm::detail::check(lm_control_flow_plan_add_control_flow(raw_, &desc));
}

inline void ControlFlowPlan::add_meta_analysis(std::unique_ptr<MetaAnalysisPass> pass) {
    LmPluginMetaAnalysis desc = make_descriptor(std::move(pass));
    lm::detail::check(lm_control_flow_plan_add_meta_analysis(raw_, &desc));
}

namespace detail {

/// Shared reversal registration: boxes the author's `backward` and installs a dispatch trampoline
/// under `kind`/`id`. Backs both `register_backward` overloads (transformation and composite —
/// both reversible kinds share this exact solution-consume/produce shape).
inline void register_backward(LmPassKind kind, const char *id, BackwardFn backward) {
    auto *boxed = new BackwardFn(backward);
    LmBackwardFn dispatch = [](void *user_data, const std::uint8_t *artifact, std::uintptr_t len,
                               LmSol *solution, LmSol **out_solution) -> LmStatus {
        BackwardFn impl = *static_cast<BackwardFn *>(user_data);
        // View the host's artifact buffer directly (valid for this call — no copy).
        std::span<const std::uint8_t> bytes = (artifact != nullptr && len > 0)
                                                  ? std::span<const std::uint8_t>(artifact, len)
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
    lm::detail::check(lm_register_plugin_backward(kind, id, dispatch, boxed));
}

} // namespace detail

/// Registers `T`'s reversal handler with the host, keyed by `T::ID`. Call once per pass type
/// (typically from the plugin's registration hook). `T` must provide
/// `static constexpr const char *ID` and a matching `static lm::Solution backward(...)`. Overloaded
/// for both transformation and composite passes (the reversible kinds).
///
/// The handler must stay valid for as long as any record produced by the pass may be reversed
/// (typically the process lifetime), so the boxed function pointer is intentionally never freed.
template <ReversiblePass T> void register_backward() {
    detail::register_backward(LmPassKind_Transformation, T::ID, &T::backward);
}

/// Composite overload of [`register_backward`] — composites are reversible too.
template <ReversibleCompositePass T> void register_backward() {
    detail::register_backward(LmPassKind_Composite, T::ID, &T::backward);
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
/// Capsule name / method name the host expects for a composite pass.
inline constexpr const char *COMPOSITE_CAPSULE_NAME = "luna_model.capi.PluginComposite.v1";
inline constexpr const char *COMPOSITE_METHOD = "_luna_plugin_composite_capsule";
/// Capsule name / method name the host expects for a meta-analysis pass.
inline constexpr const char *META_ANALYSIS_CAPSULE_NAME = "luna_model.capi.PluginMetaAnalysis.v1";
inline constexpr const char *META_ANALYSIS_METHOD = "_luna_plugin_meta_analysis_capsule";
/// Capsule name / method name the host expects for a control-flow pass.
inline constexpr const char *CONTROL_FLOW_CAPSULE_NAME = "luna_model.capi.PluginControlFlow.v1";
inline constexpr const char *CONTROL_FLOW_METHOD = "_luna_plugin_control_flow_capsule";

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

inline void free_composite_capsule(PyObject *capsule) {
    std::free(PyCapsule_GetPointer(capsule, COMPOSITE_CAPSULE_NAME));
}

inline void free_meta_analysis_capsule(PyObject *capsule) {
    std::free(PyCapsule_GetPointer(capsule, META_ANALYSIS_CAPSULE_NAME));
}

inline void free_control_flow_capsule(PyObject *capsule) {
    std::free(PyCapsule_GetPointer(capsule, CONTROL_FLOW_CAPSULE_NAME));
}

} // namespace detail

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

/// Wraps a composite descriptor by value in the capsule the host reads from
/// `_luna_plugin_composite_capsule()`. See `transform_capsule` for ownership.
inline PyObject *composite_capsule(const LmPluginComposite &descriptor) {
    auto *stored = static_cast<LmPluginComposite *>(std::malloc(sizeof(LmPluginComposite)));
    if (stored == nullptr) {
        return PyErr_NoMemory();
    }
    *stored = descriptor;
    PyObject *capsule =
        PyCapsule_New(stored, COMPOSITE_CAPSULE_NAME, &detail::free_composite_capsule);
    if (capsule == nullptr) {
        std::free(stored);
    }
    return capsule;
}

/// Wraps a meta-analysis descriptor by value in the capsule the host reads from
/// `_luna_plugin_meta_analysis_capsule()`. See `transform_capsule` for ownership.
inline PyObject *meta_analysis_capsule(const LmPluginMetaAnalysis &descriptor) {
    auto *stored = static_cast<LmPluginMetaAnalysis *>(std::malloc(sizeof(LmPluginMetaAnalysis)));
    if (stored == nullptr) {
        return PyErr_NoMemory();
    }
    *stored = descriptor;
    PyObject *capsule =
        PyCapsule_New(stored, META_ANALYSIS_CAPSULE_NAME, &detail::free_meta_analysis_capsule);
    if (capsule == nullptr) {
        std::free(stored);
    }
    return capsule;
}

/// Wraps a control-flow descriptor by value in the capsule the host reads from
/// `_luna_plugin_control_flow_capsule()`. See `transform_capsule` for ownership.
inline PyObject *control_flow_capsule(const LmPluginControlFlow &descriptor) {
    auto *stored = static_cast<LmPluginControlFlow *>(std::malloc(sizeof(LmPluginControlFlow)));
    if (stored == nullptr) {
        return PyErr_NoMemory();
    }
    *stored = descriptor;
    PyObject *capsule =
        PyCapsule_New(stored, CONTROL_FLOW_CAPSULE_NAME, &detail::free_control_flow_capsule);
    if (capsule == nullptr) {
        std::free(stored);
    }
    return capsule;
}

} // namespace python
#endif // LUNA_MODEL_PYTHON_EXTENSION

} // namespace plugin
} // namespace lm

#endif // LUNA_MODEL_PLUGIN_HPP
