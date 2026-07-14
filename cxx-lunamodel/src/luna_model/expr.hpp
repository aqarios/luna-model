#ifndef LUNA_MODEL_EXPRESSION_HPP
#define LUNA_MODEL_EXPRESSION_HPP

#include <cstddef>
#include <cstdint>
#include <map>
#include <optional>
#include <string>
#include <tuple>
#include <utility>
#include <vector>

#include <luna_model.h>

#include "luna_model/detail/capi.hpp"
#include "luna_model/env.hpp"
#include "luna_model/higher_order.hpp"
#include "luna_model/linear.hpp"
#include "luna_model/quadratic.hpp"

namespace lm {

class Vref;      // defined in vref.hpp; cross-type bodies are defined in ops.hpp
class Solution;  // defined in solution.hpp; `evaluate` is defined there

/* Owning handle to a symbolic expression over an environment's variables
 * (frees on destruction). */
class Expression {
  public:
    /* Creates an empty expression with a fresh environment. */
    Expression() : handle_(lm_expr_default()) {}

    /* Creates an empty expression bound to `env`. */
    explicit Expression(const Env &env) {
        LmExpr *out = nullptr;
        detail::check(lm_expr_new(env.get(), &out));
        handle_ = out;
    }

    /* Creates a constant expression bound to `env`. */
    Expression(const Env &env, double value) {
        LmExpr *out = nullptr;
        detail::check(lm_expr_new_constant(env.get(), value, &out));
        handle_ = out;
    }

    /* Creates a constant expression from a double. */
    static Expression from_double(double value) {
        LmExpr *out = nullptr;
        detail::check(lm_expr_from_double(value, &out));
        return Expression(out);
    }

    /* Creates a constant expression from a size_t. */
    static Expression from_size_t(std::size_t value) {
        LmExpr *out = nullptr;
        detail::check(lm_expr_from_size_t(value, &out));
        return Expression(out);
    }

    /* Creates an expression from a single variable. Defined in ops.hpp. */
    static Expression from_vref(const Vref &var);

    /* Creates an expression from a linear term store. The store is copied. */
    static Expression from_linear(const Linear &linear) {
        LmExpr *out = nullptr;
        detail::check(lm_expr_from_linear(linear.get(), &out));
        return Expression(out);
    }

    /* Creates an expression from a quadratic term store. The store is copied. */
    static Expression from_quadratic(const Quadratic &quadratic) {
        LmExpr *out = nullptr;
        detail::check(lm_expr_from_quadratic(quadratic.get(), &out));
        return Expression(out);
    }

    /* Creates an expression from a higher-order term store. The store is copied. */
    static Expression from_higher_order(const HigherOrder &higher_order) {
        LmExpr *out = nullptr;
        detail::check(lm_expr_from_higher_order(higher_order.get(), &out));
        return Expression(out);
    }

    /* Creates a quadratic expression from a dense row-major coefficient matrix
     * of `num_vars * num_vars` entries, bound to `env`. */
    static Expression from_dense_quadratic(const std::vector<double> &dense, std::size_t num_vars,
                                           const Env &env,
                                           std::optional<double> offset = std::nullopt) {
        LmExpr *out = nullptr;
        detail::check(lm_expr_from_dense_quadratic(dense.data(), dense.size(), num_vars,
                                                   offset ? &*offset : nullptr, env.get(), &out));
        return Expression(out);
    }

    /* Deep-clones many expressions of the same environment into one shared
     * deep-cloned environment. */
    static std::vector<Expression> deep_clone_many(const std::vector<const Expression *> &exprs) {
        std::vector<Expression> out;
        if (exprs.empty()) {
            return out;
        }
        Env source = exprs.front()->env();
        LmEnvId source_id = source.id();
        Env target = source.deep_clone();
        out.reserve(exprs.size());
        for (const Expression *expr : exprs) {
            LmEnvId eid = expr->env().id();
            if (eid.hi != source_id.hi || eid.lo != source_id.lo) {
                throw Error(LmStatus_InvalidArgument,
                            "expressions belong to different environments");
            }
            out.push_back(expr->deep_clone(target));
        }
        return out;
    }

    /* Adopts ownership of a raw C handle (for C interop). */
    explicit Expression(LmExpr *handle) noexcept : handle_(handle) {}

    ~Expression() {
        if (handle_ != nullptr) {
            lm_expr_free(handle_);
        }
    }

    Expression(Expression &&other) noexcept : handle_(other.handle_) { other.handle_ = nullptr; }
    Expression &operator=(Expression &&other) noexcept {
        if (this != &other) {
            if (handle_ != nullptr) {
                lm_expr_free(handle_);
            }
            handle_ = other.handle_;
            other.handle_ = nullptr;
        }
        return *this;
    }
    Expression(const Expression &) = delete;
    Expression &operator=(const Expression &) = delete;

    /* A handle to the expression's shared environment. */
    Env env() const {
        LmEnv *out = nullptr;
        detail::check(lm_expr_env(handle_, &out));
        return Env(out);
    }

    /* The constant offset. */
    double offset() const {
        double out = 0.0;
        detail::check(lm_expr_offset(handle_, &out));
        return out;
    }

    /* A copy of the complete linear term store. */
    Linear linear_terms() const {
        LmLinear *out = nullptr;
        detail::check(lm_expr_linear(handle_, &out));
        return Linear(out);
    }

    /* A copy of the quadratic term store, if any. */
    std::optional<Quadratic> quadratic_terms() const {
        LmQuadratic *out = nullptr;
        detail::check(lm_expr_quadratic(handle_, &out));
        if (out == nullptr) {
            return std::nullopt;
        }
        return Quadratic(out);
    }

    /* A copy of the higher-order term store, if any. */
    std::optional<HigherOrder> higher_order_terms() const {
        LmHigherOrder *out = nullptr;
        detail::check(lm_expr_higher_order(handle_, &out));
        if (out == nullptr) {
            return std::nullopt;
        }
        return HigherOrder(out);
    }

    /* The linear bias for a single variable index. */
    double linear(std::uint32_t idx) const { return linear_terms()[idx]; }

    /* The quadratic bias for a variable pair (0 if no quadratic terms). */
    double quadratic(std::uint32_t u, std::uint32_t v) const {
        auto q = quadratic_terms();
        return q ? (*q)(u, v) : 0.0;
    }

    /* The higher-order bias for a variable tuple (0 if no higher-order terms). */
    double higher_order(const std::vector<std::uint32_t> &vars) const {
        auto h = higher_order_terms();
        return h ? (*h)(vars) : 0.0;
    }

    /* The number of distinct variables. */
    std::size_t num_vars() const {
        std::size_t out = 0;
        detail::check(lm_expr_num_vars(handle_, &out));
        return out;
    }

    /* The degree of the expression. */
    std::size_t degree() const {
        std::size_t out = 0;
        detail::check(lm_expr_degree(handle_, &out));
        return out;
    }

    /* Whether the expression is a plain constant. */
    bool is_constant() const {
        bool out = false;
        detail::check(lm_expr_is_constant(handle_, &out));
        return out;
    }

    /* Whether at least one quadratic term is present. */
    bool has_quadratic() const {
        bool out = false;
        detail::check(lm_expr_has_quadratic(handle_, &out));
        return out;
    }

    /* Whether at least one higher-order term is present. */
    bool has_higher_order() const {
        bool out = false;
        detail::check(lm_expr_has_higher_order(handle_, &out));
        return out;
    }

    /* The distinct variable types used by the expression. */
    std::vector<LmVtype> vtypes() const {
        LmVtypeBuffer buf = {nullptr, 0, 0};
        detail::check(lm_expr_vtypes(handle_, &buf));
        std::vector<LmVtype> out(buf.data, buf.data + buf.len);
        lm_vtype_buffer_free(buf);
        return out;
    }

    /* Evaluates the expression against a name-keyed sample. */
    double evaluate_sample(const std::map<std::string, double> &sample) const {
        std::vector<const char *> names;
        std::vector<double> values;
        names.reserve(sample.size());
        values.reserve(sample.size());
        for (const auto &[name, value] : sample) {
            names.push_back(name.c_str());
            values.push_back(value);
        }
        double out = 0.0;
        detail::check(
            lm_expr_evaluate_sample(handle_, names.data(), values.data(), names.size(), &out));
        return out;
    }

    /* Evaluates the expression across every sample row of `sol`.
     * Defined in solution.hpp, where `Solution` is complete. */
    std::vector<double> evaluate(const Solution &sol) const;

    /* The distinct variables used by the expression. Defined in ops.hpp. */
    std::vector<Vref> vars() const;
    /* The distinct variables including inverted binary companions. ops.hpp. */
    std::vector<Vref> complete_vars() const;
    /* Whether the expression references `var`. Defined in ops.hpp. */
    bool contains(const Vref &var) const;
    /* `(variable, bias)` linear items. Defined in ops.hpp. */
    std::vector<std::pair<Vref, double>> linear_items() const;
    /* `(u, v, bias)` quadratic items. Defined in ops.hpp. */
    std::vector<std::tuple<Vref, Vref, double>> quadratic_items() const;
    /* `(variables, bias)` higher-order items. Defined in ops.hpp. */
    std::vector<std::pair<std::vector<Vref>, double>> higher_order_items() const;
    /* All contributions as `(variables, bias)`, including the constant offset
     * as an empty-variable entry. Defined in ops.hpp. */
    std::vector<std::pair<std::vector<Vref>, double>> items() const;

    /* In-place arithmetic. Defined in ops.hpp. The free binary operators
     * (`a + b` etc.) are also declared in ops.hpp. */
    Expression &operator+=(double rhs);
    Expression &operator+=(const Vref &rhs);
    Expression &operator+=(const Expression &rhs);
    Expression &operator-=(double rhs);
    Expression &operator-=(const Vref &rhs);
    Expression &operator-=(const Expression &rhs);
    Expression &operator*=(double rhs);
    Expression &operator*=(const Vref &rhs);
    Expression &operator*=(const Expression &rhs);

    /* Raises the expression to the power `n`. */
    Expression pow(std::size_t n) const {
        LmExpr *out = nullptr;
        detail::check(lm_expr_pow(handle_, n, &out));
        return Expression(out);
    }

    /* Raises the expression to the power `n` in place. */
    void ipow(std::size_t n) { detail::check(lm_expr_ipow(handle_, n)); }

    /* Splits the expression into the part involving `vars` and the rest.
     * Defined in ops.hpp. */
    std::tuple<Expression, Expression> separate(std::vector<Vref> &vars) const;

    /* Substitutes `replacement` for `vref` throughout the expression.
     * Defined in ops.hpp. */
    Expression substitute(const Vref &vref, const Expression &replacement) const;

    /* Clones into a new handle that shares the environment. */
    Expression clone() const {
        LmExpr *out = nullptr;
        detail::check(lm_expr_clone(handle_, &out));
        return Expression(out);
    }

    /* Deep-clones into an independent expression rebound to `env`. */
    Expression deep_clone(const Env &env) const {
        LmExpr *out = nullptr;
        detail::check(lm_expr_deep_clone(handle_, env.get(), &out));
        return Expression(out);
    }

    /* Structural equality including environment identity. */
    bool operator==(const Expression &other) const {
        bool out = false;
        detail::check(lm_expr_eq(handle_, other.handle_, &out));
        return out;
    }
    bool operator!=(const Expression &other) const { return !(*this == other); }

    /* Content equality, ignoring environment identity. */
    bool equal_contents(const Expression &other) const {
        bool out = false;
        detail::check(lm_expr_equal_contents(handle_, other.handle_, &out));
        return out;
    }

    /* Serializes the expression to bytes. */
    std::vector<std::uint8_t> serialize() const {
        LmBuffer buf = {nullptr, 0, 0};
        detail::check(lm_expr_serialize(handle_, &buf));
        std::vector<std::uint8_t> bytes(buf.data, buf.data + buf.len);
        lm_buffer_free(buf);
        return bytes;
    }

    /* Deserializes an expression from bytes, rebinding it to `env`. */
    static Expression deserialize(const std::uint8_t *data, std::size_t len, const Env &env) {
        LmExpr *out = nullptr;
        detail::check(lm_expr_deserialize(data, len, env.get(), &out));
        return Expression(out);
    }
    static Expression deserialize(const std::vector<std::uint8_t> &bytes, const Env &env) {
        return deserialize(bytes.data(), bytes.size(), env);
    }

    LmExpr *get() const noexcept { return handle_; }
    LmExpr *release() noexcept {
        LmExpr *h = handle_;
        handle_ = nullptr;
        return h;
    }

  private:
    LmExpr *handle_;
};

} // namespace lm

#endif // LUNA_MODEL_EXPRESSION_HPP
