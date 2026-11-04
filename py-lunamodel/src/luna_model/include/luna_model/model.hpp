#ifndef LUNA_MODEL_MODEL_HPP
#define LUNA_MODEL_MODEL_HPP

#include <cstddef>
#include <cstdint>
#include <optional>
#include <string>
#include <vector>

#include <luna_model.h>

#include "luna_model/constraint_collection.hpp"
#include "luna_model/detail/capi.hpp"
#include "luna_model/env.hpp"
#include "luna_model/error.hpp"
#include "luna_model/expr.hpp"
#include "luna_model/lazy_bounds.hpp"
#include "luna_model/result_view.hpp"
#include "luna_model/sample_view.hpp"
#include "luna_model/solution.hpp"
#include "luna_model/specs.hpp"
#include "luna_model/vref.hpp"

namespace lm {

/* Owning handle to a model: an objective expression, a constraint collection,
 * an optimization sense, and a shared environment (frees on destruction). */
class Model {
  public:
    /* Creates an empty unnamed minimization model. */
    Model() : handle_(lm_model_default()) {
        if (handle_ == nullptr) {
            throw Error(LmStatus_Panic, "failed to allocate a LunaModel model");
        }
    }

    /* Creates a new model. `std::nullopt` selects the default name ("unnamed")
     * / sense (minimize). */
    explicit Model(std::optional<std::string> name, std::optional<LmSense> sense = std::nullopt) {
        LmModel *out = nullptr;
        detail::check(
            lm_model_new(name ? name->c_str() : nullptr, sense ? &*sense : nullptr, &out));
        handle_ = out;
    }

    /* Creates a model over an existing environment. */
    static Model with_env(const Env &env, std::optional<std::string> name = std::nullopt,
                          std::optional<LmSense> sense = std::nullopt) {
        LmModel *out = nullptr;
        detail::check(lm_model_with_env(name ? name->c_str() : nullptr, sense ? &*sense : nullptr,
                                        env.get(), &out));
        return Model(out);
    }

    /* Adopts ownership of a raw C handle (for C interop). */
    explicit Model(LmModel *handle) noexcept : handle_(handle) {}

    ~Model() {
        if (handle_ != nullptr) {
            lm_model_free(handle_);
        }
    }

    Model(Model &&other) noexcept : handle_(other.handle_) { other.handle_ = nullptr; }
    Model &operator=(Model &&other) noexcept {
        if (this != &other) {
            if (handle_ != nullptr) {
                lm_model_free(handle_);
            }
            handle_ = other.handle_;
            other.handle_ = nullptr;
        }
        return *this;
    }
    Model(const Model &) = delete;
    Model &operator=(const Model &) = delete;

    // --- naming / sense ---

    /* The model name. */
    std::string name() const {
        LmBuffer buf = {nullptr, 0, 0};
        detail::check(lm_model_name(handle_, &buf));
        std::string out(reinterpret_cast<const char *>(buf.data), buf.len);
        lm_buffer_free(buf);
        return out;
    }

    /* Sets the model name. */
    void set_name(const std::string &name) {
        detail::check(lm_model_set_name(handle_, name.c_str()));
    }

    /* The optimization sense. */
    LmSense sense() const {
        LmSense out;
        detail::check(lm_model_sense(handle_, &out));
        return out;
    }

    /* Sets the optimization sense. */
    void set_sense(LmSense sense) { detail::check(lm_model_set_sense(handle_, sense)); }

    // --- environment / variables ---

    /* A handle to the model's shared environment. */
    Env env() const {
        LmEnv *out = nullptr;
        detail::check(lm_model_env(handle_, &out));
        return Env(out);
    }

    /* The number of variables in the model environment. */
    std::size_t num_variables() const {
        std::size_t out = 0;
        detail::check(lm_model_num_variables(handle_, &out));
        return out;
    }

    /* The distinct variable types used by the model. */
    std::vector<LmVtype> vtypes() const {
        LmVtypeBuffer buf = {nullptr, 0, 0};
        detail::check(lm_model_vtypes(handle_, &buf));
        std::vector<LmVtype> out(buf.data, buf.data + buf.len);
        lm_vtype_buffer_free(buf);
        return out;
    }

    /* All variable references of the model. */
    std::vector<Vref> vars() const {
        LmVrefBuffer buf = {nullptr, 0, 0};
        detail::check(lm_model_vars(handle_, &buf));
        std::vector<Vref> out;
        out.reserve(buf.len);
        for (std::size_t i = 0; i < buf.len; ++i) {
            out.emplace_back(buf.data[i]); // Vref adopts the handle,
            buf.data[i] = nullptr;         // so the buffer must not free it.
        }
        lm_vref_buffer_free(buf);
        return out;
    }

    /* A reference to the variable named `name`. */
    Vref var(const std::string &name) const {
        LmVref *out = nullptr;
        detail::check(lm_model_var(handle_, name.c_str(), &out));
        return Vref(out);
    }

    /* Adds a variable to the model environment. `bounds == nullptr` uses the
     * default bounds for `vtype`. */
    Vref add_var(const std::string &name, LmVtype vtype, const LazyBounds *bounds = nullptr) {
        LmVref *out = nullptr;
        detail::check(
            lm_model_add_var(handle_, name.c_str(), vtype, bounds ? bounds->get() : nullptr, &out));
        return Vref(out);
    }

    /* Adds a variable, retrying with an encoding fallback on name clashes. */
    Vref add_var_with_fallback(const std::string &name, LmVtype vtype,
                               const LazyBounds *bounds = nullptr) {
        LmVref *out = nullptr;
        detail::check(lm_model_add_var_with_fallback(handle_, name.c_str(), vtype,
                                                     bounds ? bounds->get() : nullptr, &out));
        return Vref(out);
    }

    // --- objective / constraints ---

    /* The objective expression. */
    Expression objective() const {
        LmExpr *out = nullptr;
        detail::check(lm_model_objective(handle_, &out));
        return Expression(out);
    }

    /* Sets the objective expression and optionally updates the model sense. */
    void set_objective(const Expression &expr, std::optional<LmSense> sense = std::nullopt) {
        detail::check(lm_model_set_objective(handle_, expr.get(), sense ? &*sense : nullptr));
    }

    /* A copy of the constraint collection. */
    ConstraintCollection constraints() const {
        LmCC *out = nullptr;
        detail::check(lm_model_constraints(handle_, &out));
        return ConstraintCollection(out);
    }

    /* Replaces the constraint collection. */
    void set_constraints(const ConstraintCollection &cc) {
        detail::check(lm_model_set_constraints(handle_, cc.get()));
    }

    /* Substitutes `replacement` for `target` in the objective and constraints. */
    void substitute(const Vref &target, const Expression &replacement) {
        detail::check(lm_model_substitute(handle_, target.get(), replacement.get()));
    }

    /* The subset of constraints that `sample` violates, within `tol`. */
    ConstraintCollection violated_constraints(const SampleView &sample,
                                              std::optional<double> tol = std::nullopt) const {
        LmCC *out = nullptr;
        detail::check(
            lm_model_violated_constraints(handle_, sample.get(), tol ? &*tol : nullptr, &out));
        return ConstraintCollection(out);
    }

    /* Evaluates the model against a single sample row, returning that row's
     * enriched result view. `tol` is forwarded to constraint comparisons. */
    ResultView evaluate_sample(const SampleView &sample,
                               std::optional<double> tol = std::nullopt) const {
        LmResultView *out = nullptr;
        detail::check(lm_model_evaluate_sample(handle_, sample.get(), tol ? &*tol : nullptr, &out));
        return ResultView(out);
    }

    // --- specification ---

    /* The structural specification of the model. */
    Specs specs() const {
        LmSpecs *out = nullptr;
        detail::check(lm_model_specs(handle_, &out));
        return Specs(out);
    }

    /* Whether the model satisfies `specs`. */
    bool satisfies(const Specs &specs) const {
        bool out = false;
        detail::check(lm_model_satisfies(handle_, specs.get(), &out));
        return out;
    }

    // --- evaluation ---

    /* Evaluates the model for a solution. */
    Solution evaluate_solution(const Solution &solution) const {
        LmSol *out = nullptr;
        detail::check(lm_model_evaluate_solution(handle_, solution.get(), &out));
        return Solution(out);
    }

    /* Evaluates the model for a solution with a feasibility tolerance.
     * `std::nullopt` selects the default tolerance. */
    Solution evaluate_solution_with_tol(const Solution &solution, std::optional<double> tol) const {
        LmSol *out = nullptr;
        detail::check(lm_model_evaluate_solution_with_tol(handle_, solution.get(),
                                                          tol ? &*tol : nullptr, &out));
        return Solution(out);
    }

    // --- copying / comparison / serialization ---

    /* Clones into a new handle that shares the environment. */
    Model clone() const {
        LmModel *out = nullptr;
        detail::check(lm_model_clone(handle_, &out));
        return Model(out);
    }

    /* Deep-clones into a fully independent model with its own environment. */
    Model deep_clone() const {
        LmModel *out = nullptr;
        detail::check(lm_model_deep_clone(handle_, &out));
        return Model(out);
    }

    /* Structural equality including environment identity. */
    bool operator==(const Model &other) const {
        bool out = false;
        detail::check(lm_model_eq(handle_, other.handle_, &out));
        return out;
    }
    bool operator!=(const Model &other) const { return !(*this == other); }

    /* Content equality, ignoring environment identity. */
    bool equal_contents(const Model &other) const {
        bool out = false;
        detail::check(lm_model_equal_contents(handle_, other.handle_, &out));
        return out;
    }

    /* Serializes the model to bytes. */
    std::vector<std::uint8_t> serialize() const {
        LmBuffer buf = {nullptr, 0, 0};
        detail::check(lm_model_serialize(handle_, &buf));
        std::vector<std::uint8_t> bytes(buf.data, buf.data + buf.len);
        lm_buffer_free(buf);
        return bytes;
    }

    static Model deserialize(const std::uint8_t *data, std::size_t len) {
        LmModel *out = nullptr;
        detail::check(lm_model_deserialize(data, len, &out));
        return Model(out);
    }
    static Model deserialize(const std::vector<std::uint8_t> &bytes) {
        return deserialize(bytes.data(), bytes.size());
    }

    LmModel *get() const noexcept { return handle_; }
    LmModel *release() noexcept {
        LmModel *h = handle_;
        handle_ = nullptr;
        return h;
    }

  private:
    LmModel *handle_;
};

} // namespace lm

#endif // LUNA_MODEL_MODEL_HPP
