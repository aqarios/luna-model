#ifndef LUNA_MODEL_OPS_HPP
#define LUNA_MODEL_OPS_HPP

/*
 * Operator overloads spanning multiple wrapper types (mirrors the Rust
 * `ops/` module). Lives below `expr.hpp` and `vref.hpp` in the include
 * graph so each of those headers only needs a forward declaration of the
 * other type; the bodies that require both complete types are defined here.
 */

#include <cstddef>
#include <vector>

#include <luna_model.h>

#include "luna_model/detail/capi.hpp"
#include "luna_model/expr.hpp"
#include "luna_model/funcs.h"
#include "luna_model/types.h"
#include "luna_model/vref.hpp"

namespace lm {

namespace detail {
/* Adopts every handle of an owned vref buffer into `Vref`s, then releases
 * the outer array. Nulled slots are skipped by the buffer free. */
inline std::vector<Vref> adopt_vref_buffer(LmVrefBuffer buf) {
    std::vector<Vref> out;
    out.reserve(buf.len);
    for (std::size_t i = 0; i < buf.len; ++i) {
        out.emplace_back(buf.data[i]);
        buf.data[i] = nullptr;
    }
    lm_vref_buffer_free(buf);
    return out;
}
} // namespace detail

/* ///////////////////////////////////////////////////////////////////// */
/* Out-of-line member definitions (declared in expr.hpp / vref.hpp).     */
/* ///////////////////////////////////////////////////////////////////// */

inline std::vector<Vref> Expression::vars() const {
    LmVrefBuffer buf = {nullptr, 0, 0};
    detail::check(lm_expr_vars(handle_, &buf));
    return detail::adopt_vref_buffer(buf);
}

inline std::vector<Vref> Expression::complete_vars() const {
    LmVrefBuffer buf = {nullptr, 0, 0};
    detail::check(lm_expr_complete_vars(handle_, &buf));
    return detail::adopt_vref_buffer(buf);
}

inline Expression Vref::pow(std::size_t n) const {
    LmExpr *out = nullptr;
    detail::check(lm_vref_pow(handle_, n, &out));
    return Expression(out);
}

/* //////////////////////////////// */
/* (In-Place) Expression operations */
/* //////////////////////////////// */

inline Expression &Expression::operator+=(double rhs) {
    detail::check(lm_expr_iadd_bias(handle_, rhs));
    return *this;
}

inline Expression &Expression::operator+=(const Vref &rhs) {
    detail::check(lm_expr_iadd_vref(handle_, rhs.get()));
    return *this;
}

inline Expression &Expression::operator+=(const Expression &rhs) {
    detail::check(lm_expr_iadd_expr(handle_, rhs.get()));
    return *this;
}

inline Expression &Expression::operator-=(double rhs) {
    detail::check(lm_expr_isub_bias(handle_, rhs));
    return *this;
}

inline Expression &Expression::operator-=(const Vref &rhs) {
    detail::check(lm_expr_isub_vref(handle_, rhs.get()));
    return *this;
}

inline Expression &Expression::operator-=(const Expression &rhs) {
    detail::check(lm_expr_isub_expr(handle_, rhs.get()));
    return *this;
}

inline Expression &Expression::operator*=(double rhs) {
    detail::check(lm_expr_imul_bias(handle_, rhs));
    return *this;
}

inline Expression &Expression::operator*=(const Vref &rhs) {
    detail::check(lm_expr_imul_vref(handle_, rhs.get()));
    return *this;
}

inline Expression &Expression::operator*=(const Expression &rhs) {
    detail::check(lm_expr_imul_expr(handle_, rhs.get()));
    return *this;
}

inline Expression operator+(const Expression &lhs, double rhs) {
    LmExpr *out = nullptr;
    detail::check(lm_expr_add_bias(lhs.get(), rhs, &out));
    return Expression(out);
}

inline Expression operator+(const Expression &lhs, const Vref &rhs) {
    LmExpr *out = nullptr;
    detail::check(lm_expr_add_vref(lhs.get(), rhs.get(), &out));
    return Expression(out);
}

inline Expression operator+(const Expression &lhs, const Expression &rhs) {
    LmExpr *out = nullptr;
    detail::check(lm_expr_add_expr(lhs.get(), rhs.get(), &out));
    return Expression(out);
}

inline Expression operator-(const Expression &lhs, double rhs) {
    LmExpr *out = nullptr;
    detail::check(lm_expr_sub_bias(lhs.get(), rhs, &out));
    return Expression(out);
}

inline Expression operator-(const Expression &lhs, const Vref &rhs) {
    LmExpr *out = nullptr;
    detail::check(lm_expr_sub_vref(lhs.get(), rhs.get(), &out));
    return Expression(out);
}

inline Expression operator-(const Expression &lhs, const Expression &rhs) {
    LmExpr *out = nullptr;
    detail::check(lm_expr_sub_expr(lhs.get(), rhs.get(), &out));
    return Expression(out);
}

inline Expression operator*(const Expression &lhs, double rhs) {
    LmExpr *out = nullptr;
    detail::check(lm_expr_mul_bias(lhs.get(), rhs, &out));
    return Expression(out);
}

inline Expression operator*(const Expression &lhs, const Vref &rhs) {
    LmExpr *out = nullptr;
    detail::check(lm_expr_mul_vref(lhs.get(), rhs.get(), &out));
    return Expression(out);
}

inline Expression operator*(const Expression &lhs, const Expression &rhs) {
    LmExpr *out = nullptr;
    detail::check(lm_expr_mul_expr(lhs.get(), rhs.get(), &out));
    return Expression(out);
}

/* -expr → Expression. */
inline Expression operator-(const Expression &lhs) {
    LmExpr *out = nullptr;
    detail::check(lm_expr_neg(lhs.get(), &out));
    return Expression(out);
}

inline std::tuple<Expression, Expression> Expression::separate(std::vector<Vref> &vars) const {
    std::vector<const LmVref *> raw;
    raw.reserve(vars.size());

    for (const auto &var : vars) {
        raw.push_back(var.get());
    }

    LmExpr *first = nullptr;
    LmExpr *second = nullptr;

    detail::check(lm_expr_separate(handle_, raw.data(), raw.size(), &first, &second));

    return {Expression(first), Expression(second)};
}

inline Expression Expression::substitute(const Vref &vref, const Expression &replacement) const {
    LmExpr *out = nullptr;
    detail::check(lm_expr_substitute(handle_, vref.get(), replacement.get(), &out));
    return Expression(out);
}

/* ///////////////////////////////////////////////////////////////////// */
/* Vref ops (Rust `ops/vref/`).                                          */
/* ///////////////////////////////////////////////////////////////////// */

/* vref * vref → Expression (Rust `&a * &b`). */
inline Expression operator*(const Vref &lhs, const Vref &rhs) {
    LmExpr *out = nullptr;
    detail::check(lm_vref_mul_vref(lhs.get(), rhs.get(), &out));
    return Expression(out);
}

/* vref * scalar → Expression (Rust `&v * bias`). */
inline Expression operator*(const Vref &lhs, double rhs) {
    LmExpr *out = nullptr;
    detail::check(lm_vref_mul_bias(lhs.get(), rhs, &out));
    return Expression(out);
}

/* scalar * vref — multiplication commutes, so delegate. */
inline Expression operator*(double lhs, const Vref &rhs) { return rhs * lhs; }

/* vref + vref → Expression (Rust `&a + &b`). */
inline Expression operator+(const Vref &lhs, const Vref &rhs) {
    LmExpr *out = nullptr;
    detail::check(lm_vref_add_vref(lhs.get(), rhs.get(), &out));
    return Expression(out);
}

/* vref + scalar → Expression (Rust `&v + bias`). */
inline Expression operator+(const Vref &lhs, double rhs) {
    LmExpr *out = nullptr;
    detail::check(lm_vref_add_bias(lhs.get(), rhs, &out));
    return Expression(out);
}

/* scalar + vref — addition commutes, so delegate. */
inline Expression operator+(double lhs, const Vref &rhs) { return rhs + lhs; }

/* vref - vref → Expression (Rust `&a - &b`). */
inline Expression operator-(const Vref &lhs, const Vref &rhs) {
    LmExpr *out = nullptr;
    detail::check(lm_vref_sub_vref(lhs.get(), rhs.get(), &out));
    return Expression(out);
}

/* vref - scalar → Expression (Rust `&v - bias`). */
inline Expression operator-(const Vref &lhs, double rhs) {
    LmExpr *out = nullptr;
    detail::check(lm_vref_sub_bias(lhs.get(), rhs, &out));
    return Expression(out);
}

/* scalar - vref → Expression (Rust `bias - v`). */
inline Expression operator-(double lhs, const Vref &rhs) {
    LmExpr *out = nullptr;
    detail::check(lm_vref_rsub_bias(rhs.get(), lhs, &out));
    return Expression(out);
}

/* -vref → Expression (Rust `-&v`). */
inline Expression operator-(const Vref &lhs) {
    LmExpr *out = nullptr;
    detail::check(lm_vref_neg(lhs.get(), &out));
    return Expression(out);
}

/* !vref → Vref, the inverted binary companion (Rust `!&v`). */
inline Vref operator!(const Vref &lhs) {
    LmVref *out = nullptr;
    detail::check(lm_vref_not(lhs.get(), &out));
    return Vref(out);
}

} // namespace lm

#endif // LUNA_MODEL_OPS_HPP
