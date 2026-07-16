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
#include "luna_model/sample_view.hpp"
#include "luna_model/solution.hpp"
#include "luna_model/specs.hpp"
#include "luna_model/vref.hpp"

namespace lm {

/* Owning handle to a LunaModel model (frees on destruction).
 *
 * Mirrors the Rust `Model`: move-only, with `clone()` for the explicit
 * (environment-sharing) copy and `deep_clone()` for a fully independent one.
 */
class Model {
  public:
    /* Creates an empty unnamed minimization model (Rust `Model::default()`). */
    Model() : handle_(lm_model_default()) {
        if (handle_ == nullptr) {
            throw Error(LmStatus_Panic, "failed to allocate a LunaModel model");
        }
    }

    /* Creates a new model (Rust `Model::new(name, sense)`).
     *
     * `std::nullopt` selects the default name ("unnamed") / sense (minimize).
     */
    explicit Model(std::optional<std::string> name, std::optional<LmSense> sense = std::nullopt) {
        LmModel *out = nullptr;
        detail::check(
            lm_model_new(name ? name->c_str() : nullptr, sense ? &*sense : nullptr, &out));
        handle_ = out;
    }

    Model(std::optional<std::string> name, std::optional<LmSense> sense, const Env &env) {
        LmModel *out = nullptr;
        detail::check(lm_model_with_env(name ? name->c_str() : nullptr, sense ? &*sense : nullptr,
                                        env.get(), &out));
        handle_ = out;
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

    /* Returns the model name (Rust `model.name`). */
    std::string name() const {
        LmBuffer buf = {nullptr, 0, 0};
        detail::check(lm_model_name(handle_, &buf));
        std::string out(reinterpret_cast<const char *>(buf.data), buf.len);
        lm_buffer_free(buf);
        return out;
    }

    /* Sets the model name (Rust `model.name = ...`). */
    void set_name(const std::string &name) {
        detail::check(lm_model_set_name(handle_, name.c_str()));
    }

    /* Returns the optimization sense (Rust `model.sense`). */
    LmSense sense() const {
        LmSense out;
        detail::check(lm_model_sense(handle_, &out));
        return out;
    }

    /* Sets the optimization sense (Rust `model.sense = ...`). */
    void set_sense(LmSense sense) { detail::check(lm_model_set_sense(handle_, sense)); }

    /* Returns the number of variables in the model environment. */
    std::size_t num_variables() const {
        std::size_t out = 0;
        detail::check(lm_model_num_variables(handle_, &out));
        return out;
    }

    /* Returns the distinct variable types used by the model. */
    std::vector<LmVtype> vtypes() const {
        LmVtypeBuffer buf = {nullptr, 0, 0};
        detail::check(lm_model_vtypes(handle_, &buf));
        std::vector<LmVtype> out(buf.data, buf.data + buf.len);
        lm_vtype_buffer_free(buf);
        return out;
    }

    bool satisfies(const Specs &specs) const {
        bool out = false;
        detail::check(lm_model_satisfies(handle_, specs.get(), &out));
        return out;
    }

    Vref add_var(const std::string &name, LmVtype vtype, const LazyBounds *bounds = nullptr) {
        LmVref *out = nullptr;
        detail::check(
            lm_model_add_var(handle_, name.c_str(), vtype, bounds ? bounds->get() : nullptr, &out));
        return Vref(out);
    }

    Vref add_var_with_fallback(const std::string &name, LmVtype vtype,
                               const LazyBounds *bounds = nullptr) {
        LmVref *out = nullptr;
        detail::check(lm_model_add_var_with_fallback(handle_, name.c_str(), vtype,
                                                     bounds ? bounds->get() : nullptr, &out));
        return Vref(out);
    }

    Env env() const {
        LmEnv *out = nullptr;
        detail::check(lm_model_env(handle_, &out));
        return Env(out);
    }

    Specs specs() const {
        LmSpecs *out = nullptr;
        detail::check(lm_model_specs(handle_, &out));
        return Specs(out);
    }

    std::vector<Vref> vars() const {
        LmVrefBuffer buffer = {nullptr, 0, 0};
        detail::check(lm_model_vars(handle_, &buffer));
        std::vector<Vref> out;
        out.reserve(buffer.len);
        for (std::size_t index = 0; index < buffer.len; ++index) {
            out.emplace_back(buffer.data[index]);
            buffer.data[index] = nullptr;
        }
        lm_vref_buffer_free(buffer);
        return out;
    }

    Vref var(const std::string &name) const {
        LmVref *out = nullptr;
        detail::check(lm_model_var(handle_, name.c_str(), &out));
        return Vref(out);
    }

    Expression objective() const {
        LmExpr *out = nullptr;
        detail::check(lm_model_objective(handle_, &out));
        return Expression(out);
    }

    ConstraintCollection constraints() const {
        LmCC *out = nullptr;
        detail::check(lm_model_constraints(handle_, &out));
        return ConstraintCollection(out);
    }

    void set_objective(const Expression &objective, std::optional<LmSense> sense = std::nullopt) {
        detail::check(
            lm_model_set_objective(handle_, objective.get(), sense ? &*sense : nullptr));
    }

    void set_constraints(const ConstraintCollection &constraints) {
        detail::check(lm_model_set_constraints(handle_, constraints.get()));
    }

    /* Evaluates the model for a solution (Rust `model.evaluate_solution(&sol)`). */
    Solution evaluate_solution(const Solution &solution) const {
        LmSol *out = nullptr;
        detail::check(lm_model_evaluate_solution(handle_, solution.get(), &out));
        return Solution(out);
    }

    /* Evaluates the model for a solution with a feasibility tolerance
     * (Rust `model.evaluate_solution_with_tol(&sol, tol)`).
     *
     * `std::nullopt` selects the default tolerance.
     */
    Solution evaluate_solution_with_tol(const Solution &solution, std::optional<double> tol) const {
        LmSol *out = nullptr;
        detail::check(lm_model_evaluate_solution_with_tol(handle_, solution.get(),
                                                          tol ? &*tol : nullptr, &out));
        return Solution(out);
    }

    ConstraintCollection violated_constraints(const SampleView &sample,
                                              std::optional<double> tol = std::nullopt) const {
        LmCC *out = nullptr;
        detail::check(
            lm_model_violated_constraints(handle_, sample.get(), tol ? &*tol : nullptr, &out));
        return ConstraintCollection(out);
    }

    void substitute(const Vref &target, const Expression &replacement) {
        detail::check(lm_model_substitute(handle_, target.get(), replacement.get()));
    }

    /* Clones into a new handle that shares the environment (Rust `Clone`). */
    Model clone() const {
        LmModel *out = nullptr;
        detail::check(lm_model_clone(handle_, &out));
        return Model(out);
    }

    /* Deep-clones into a fully independent model with its own environment
     * (Rust `model.deep_clone()`). */
    Model deep_clone() const {
        LmModel *out = nullptr;
        detail::check(lm_model_deep_clone(handle_, &out));
        return Model(out);
    }

    /* Structural equality including environment identity (Rust `PartialEq`). */
    bool operator==(const Model &other) const {
        bool out = false;
        detail::check(lm_model_eq(handle_, other.handle_, &out));
        return out;
    }
    bool operator!=(const Model &other) const { return !(*this == other); }

    /* Content equality, ignoring environment identity
     * (Rust `model.equal_contents(&other)`). */
    bool equal_contents(const Model &other) const {
        bool out = false;
        detail::check(lm_model_equal_contents(handle_, other.handle_, &out));
        return out;
    }

    /* Serialize the model to bytes. */
    std::vector<std::uint8_t> serialize() const {
        LmBuffer buf = {nullptr, 0, 0};
        detail::check(lm_model_serialize(handle_, &buf));
        std::vector<std::uint8_t> bytes(buf.data, buf.data + buf.len);
        lm_buffer_free(buf);
        return bytes;
    }

    /* Deserialize a model from bytes. */
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
