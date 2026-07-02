#ifndef LUNA_MODEL_OPS_HPP
#define LUNA_MODEL_OPS_HPP

/*
 * Operator overloads spanning multiple wrapper types (mirrors the Rust
 * `ops/` module). Lives below `expr.hpp` and `vref.hpp` in the include
 * graph so each of those headers only needs a forward declaration of the
 * other type; the bodies that require both complete types are defined here.
 */

#include <cstddef>

#include <luna_model.h>

#include "luna_model/detail/capi.hpp"
#include "luna_model/expr.hpp"
#include "luna_model/vref.hpp"

namespace lm {

/* ///////////////////////////////////////////////////////////////////// */
/* Out-of-line member definitions (declared in expr.hpp / vref.hpp).     */
/* ///////////////////////////////////////////////////////////////////// */

inline Expression &Expression::operator+=(const Vref &rhs) {
    detail::check(lm_expr_add_vref(handle_, rhs.get()));
    return *this;
}

inline Expression Vref::pow(std::size_t n) const {
    LmExpr *out = nullptr;
    detail::check(lm_vref_pow(handle_, n, &out));
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
