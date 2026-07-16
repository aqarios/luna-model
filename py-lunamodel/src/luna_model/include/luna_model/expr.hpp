#ifndef LUNA_MODEL_EXPRESSION_HPP
#define LUNA_MODEL_EXPRESSION_HPP

#include <cstddef>
#include <cstdint>
#include <optional>
#include <tuple>
#include <vector>

#include <luna_model.h>

#include "luna_model/detail/capi.hpp"
#include "luna_model/env.hpp"
#include "luna_model/higher_order.hpp"
#include "luna_model/linear.hpp"
#include "luna_model/quadratic.hpp"

namespace lm {

class Vref; // defined in vref.hpp; cross-type bodies are defined in ops.hpp

/* Owning handle to a LunaModel expression (frees on destruction).
 *
 * Mirrors the Rust `Expression`.
 */
class Expression {
  public:
    /* Creates an empty expression with a fresh environment
     * (Rust `Expression::default()`). */
    Expression() : handle_(lm_expr_default()) {}

    /* Creates an empty expression bound to `env` (Rust `Expression::empty(env)`). */
    explicit Expression(const Env &env) {
        LmExpr *out = nullptr;
        detail::check(lm_expr_new(env.get(), &out));
        handle_ = out;
    }

    /* Creates a constant expression bound to `env`
     * (Rust `Expression::constant(env, value)`). */
    Expression(const Env &env, double value) {
        LmExpr *out = nullptr;
        detail::check(lm_expr_new_constant(env.get(), value, &out));
        handle_ = out;
    }

    /* Creates a constant expression from a double (Rust `Expression::from(f64)`). */
    static Expression from_double(double value) {
        LmExpr *out = nullptr;
        detail::check(lm_expr_from_double(value, &out));
        return Expression(out);
    }

    /* Creates a constant expression from a size_t (Rust `Expression::from(usize)`). */
    static Expression from_size_t(std::size_t value) {
        LmExpr *out = nullptr;
        detail::check(lm_expr_from_size_t(value, &out));
        return Expression(out);
    }

    /* Creates an expression from a linear term store
     * (Rust `Expression::from(&linear)`). The store is copied, not consumed. */
    static Expression from_linear(const Linear &linear) {
        LmExpr *out = nullptr;
        detail::check(lm_expr_from_linear(linear.get(), &out));
        return Expression(out);
    }

    /* Creates an expression from a quadratic term store
     * (Rust `Expression::from(&quadratic)`). The store is copied, not consumed. */
    static Expression from_quadratic(const Quadratic &quadratic) {
        LmExpr *out = nullptr;
        detail::check(lm_expr_from_quadratic(quadratic.get(), &out));
        return Expression(out);
    }

    /* Creates an expression from a higher-order term store
     * (Rust `Expression::from(&ho)`). The store is copied, not consumed. */
    static Expression from_higher_order(const HigherOrder &higher_order) {
        LmExpr *out = nullptr;
        detail::check(lm_expr_from_higher_order(higher_order.get(), &out));
        return Expression(out);
    }

    static Expression from_dense_quadratic(const std::vector<double> &dense, std::size_t num_vars,
                                           std::optional<double> offset, const Env &env) {
        LmExpr *out = nullptr;
        detail::check(lm_expr_from_dense_quadratic(dense.data(), dense.size(), num_vars,
                                                   offset ? &*offset : nullptr, env.get(), &out));
        return Expression(out);
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

    /* Returns a handle to the expression's shared environment
     * (Rust `expr.env`). */
    Env env() const {
        LmEnv *out = nullptr;
        detail::check(lm_expr_env(handle_, &out));
        return Env(out);
    }

    /* Returns the constant offset (Rust `expr.offset`). */
    double offset() const {
        double out = 0.0;
        detail::check(lm_expr_offset(handle_, &out));
        return out;
    }

    /* Replaces the constant offset (Rust `Expression::set_offset`). */
    void set_offset(double value) { detail::check(lm_expr_set_offset(handle_, value)); }

    /* Adds raw degree-specific terms without changing environment identity. */
    void add_linear_term(std::uint32_t variable, double bias) {
        detail::check(lm_expr_add_linear_term(handle_, variable, bias));
    }
    void add_quadratic_term(std::uint32_t u, std::uint32_t v, double bias) {
        detail::check(lm_expr_add_quadratic_term(handle_, u, v, bias));
    }
    void add_higher_order_term(const std::vector<std::uint32_t> &variables, double bias) {
        detail::check(
            lm_expr_add_higher_order_term(handle_, variables.data(), variables.size(), bias));
    }

    /* Returns a copy of the linear terms (Rust `expr.linear`). */
    Linear linear() const {
        LmLinear *out = nullptr;
        detail::check(lm_expr_linear(handle_, &out));
        return Linear(out);
    }

    /* Returns a copy of the quadratic terms if any (Rust `expr.quadratic`). */
    std::optional<Quadratic> quadratic() const {
        LmQuadratic *out = nullptr;
        detail::check(lm_expr_quadratic(handle_, &out));
        if (out == nullptr) {
            return std::nullopt;
        }
        return Quadratic(out);
    }

    /* Returns a copy of the higher-order terms if any (Rust `expr.higher_order`). */
    std::optional<HigherOrder> higher_order() const {
        LmHigherOrder *out = nullptr;
        detail::check(lm_expr_higher_order(handle_, &out));
        if (out == nullptr) {
            return std::nullopt;
        }
        return HigherOrder(out);
    }

    /* Returns the number of distinct variables (Rust `expr.num_vars()`). */
    std::size_t num_vars() const {
        std::size_t out = 0;
        detail::check(lm_expr_num_vars(handle_, &out));
        return out;
    }

    /* Returns the degree of the expression (Rust `expr.degree()`). */
    std::size_t degree() const {
        std::size_t out = 0;
        detail::check(lm_expr_degree(handle_, &out));
        return out;
    }

    /* Returns whether the expression is a plain constant
     * (Rust `expr.is_constant()`). */
    bool is_constant() const {
        bool out = false;
        detail::check(lm_expr_is_constant(handle_, &out));
        return out;
    }

    /* Returns whether at least one quadratic term is present
     * (Rust `expr.has_quadratic()`). */
    bool has_quadratic() const {
        bool out = false;
        detail::check(lm_expr_has_quadratic(handle_, &out));
        return out;
    }

    /* Returns whether at least one higher-order term is present
     * (Rust `expr.has_higher_order()`). */
    bool has_higher_order() const {
        bool out = false;
        detail::check(lm_expr_has_higher_order(handle_, &out));
        return out;
    }

    /* Returns the distinct variable types used by the expression
     * (Rust `expr.vtypes()`). */
    std::vector<LmVtype> vtypes() const {
        LmVtypeBuffer buf = {nullptr, 0, 0};
        detail::check(lm_expr_vtypes(handle_, &buf));
        std::vector<LmVtype> out(buf.data, buf.data + buf.len);
        lm_vtype_buffer_free(buf);
        return out;
    }

    /* Returns the distinct variables used by the expression
     * (Rust `expr.vars()`). Defined in ops.hpp. */
    std::vector<Vref> vars() const;

    /* Returns the distinct variables including inverted binary companions
     * (Rust `expr.complete_vars()`). Defined in ops.hpp. */
    std::vector<Vref> complete_vars() const;

    /* expr += bias, in place. Defined in ops.hpp. */
    Expression &operator+=(double rhs);
    /* expr += vref, in place. Defined in ops.hpp. */
    Expression &operator+=(const Vref &rhs);
    /* expr += expr, in place. Defined in ops.hpp. */
    Expression &operator+=(const Expression &rhs);
    /* expr -= bias, in place. Defined in ops.hpp. */
    Expression &operator-=(double rhs);
    /* expr -= vref, in place. Defined in ops.hpp. */
    Expression &operator-=(const Vref &rhs);
    /* expr -= expr, in place. Defined in ops.hpp. */
    Expression &operator-=(const Expression &rhs);
    /* expr *= bias, in place. Defined in ops.hpp. */
    Expression &operator*=(double rhs);
    /* expr *= vref, in place. Defined in ops.hpp. */
    Expression &operator*=(const Vref &rhs);
    /* expr *= expr, in place. Defined in ops.hpp. */
    Expression &operator*=(const Expression &rhs);
    Expression pow(std::size_t n) const {
        LmExpr *out = nullptr;
        detail::check(lm_expr_pow(handle_, n, &out));
        return Expression(out);
    }

    void ipow(std::size_t n) { detail::check(lm_expr_ipow(handle_, n)); }

    std::tuple<Expression, Expression> separate(std::vector<Vref> &vars) const;
    Expression substitute(const Vref &vref, const Expression &replacement) const;

    /* Clones into a new handle that shares the environment (Rust `Clone`). */
    Expression clone() const {
        LmExpr *out = nullptr;
        detail::check(lm_expr_clone(handle_, &out));
        return Expression(out);
    }

    /* Deep-clones into an independent expression rebound to `env`
     * (Rust `expr.deep_clone(env)`). */
    Expression deep_clone(const Env &env) const {
        LmExpr *out = nullptr;
        detail::check(lm_expr_deep_clone(handle_, env.get(), &out));
        return Expression(out);
    }

    static std::vector<Expression>
    deep_clone_many(const std::vector<const Expression *> &expressions) {
        std::vector<const LmExpr *> inputs;
        inputs.reserve(expressions.size());
        for (const Expression *expression : expressions) {
            inputs.push_back(expression->get());
        }
        std::vector<LmExpr *> outputs(expressions.size(), nullptr);
        detail::check(lm_expr_deep_clone_many(inputs.data(), inputs.size(), outputs.data()));
        std::vector<Expression> out;
        out.reserve(outputs.size());
        for (LmExpr *output : outputs) {
            out.emplace_back(output);
        }
        return out;
    }

    double evaluate_sample_quick(const std::vector<double> &lookup) const {
        double out = 0.0;
        detail::check(lm_expr_evaluate_sample_quick(handle_, lookup.data(), lookup.size(), &out));
        return out;
    }

    /* Structural equality including environment identity (Rust `PartialEq`). */
    bool operator==(const Expression &other) const {
        bool out = false;
        detail::check(lm_expr_eq(handle_, other.handle_, &out));
        return out;
    }
    bool operator!=(const Expression &other) const { return !(*this == other); }

    /* Content equality, ignoring environment identity
     * (Rust `expr.equal_contents(&other)`). */
    bool equal_contents(const Expression &other) const {
        bool out = false;
        detail::check(lm_expr_equal_contents(handle_, other.handle_, &out));
        return out;
    }

    /* Serialize the expression to bytes. */
    std::vector<std::uint8_t> serialize() const {
        LmBuffer buf = {nullptr, 0, 0};
        detail::check(lm_expr_serialize(handle_, &buf));
        std::vector<std::uint8_t> bytes(buf.data, buf.data + buf.len);
        lm_buffer_free(buf);
        return bytes;
    }

    /* Deserialize an expression from bytes, rebinding it to `env`. */
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
