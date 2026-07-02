#ifndef LUNA_MODEL_VREF_HPP
#define LUNA_MODEL_VREF_HPP

#include <cstddef>
#include <optional>

#include <luna_model.h>

#include "luna_model/bounds.hpp"
#include "luna_model/detail/capi.hpp"
#include "luna_model/env.hpp"
#include "luna_model/expr.hpp"
#include "luna_model/funcs.h"
#include "luna_model/types.h"
#include "luna_model/variable.hpp"

namespace lm {

/* Owning handle to a LunaModel VarRef (frees on destruction).
 *
 * Mirrors the Rust `VarRef`.
 */
class Vref {
  public:
    /* Creates a new vref (Rust `VarRef::new(id, env)`). */
    explicit Vref(std::uint32_t id, Env env) {
        LmVref *out = nullptr;
        detail::check(lm_vref_new(id, env.get(), &out));
        handle_ = out;
    }

    /* Adopts ownership of a raw C handle (for C interop). */
    explicit Vref(LmVref *handle) noexcept : handle_(handle) {}

    ~Vref() {
        if (handle_ != nullptr) {
            lm_vref_free(handle_);
        }
    }

    /* Returns the id of the vref. */
    std::uint32_t id() const {
        std::uint32_t out;
        detail::check(lm_vref_id(handle_, &out));
        return out;
    }

    /* Returns true if the referenced variable still exists in its environment, false otherwise. */
    bool check_living() const {
        bool out;
        detail::check(lm_vref_check_living(handle_, &out));
        return out;
    }

    /* Returns the variable name (Rust `vref.name()`). */
    std::string name() const {
        LmBuffer buf = {nullptr, 0, 0};
        detail::check(lm_vref_name(handle_, &buf));
        std::string out(reinterpret_cast<const char *>(buf.data), buf.len);
        lm_buffer_free(buf);
        return out;
    }

    /* Returns the variable vtype (Rust `vref.vtype()`). */
    LmVtype vtype() const {
        LmVtype out;
        detail::check(lm_vref_vtype(handle_, &out));
        return out;
    }

    /* Returns the variable bounds (Rust `vref.bounds()`). */
    Bounds bounds() const {
        LmBounds *out = nullptr;
        detail::check(lm_vref_bounds(handle_, &out));
        return Bounds(out);
    }

    /* Returns the vref's inverted companion if one exists (Rust `vref.inverted()`). */
    std::optional<Vref> inverted() const {
        LmVref *out = nullptr;
        detail::check(lm_vref_inverted(handle_, &out));
        if (out == nullptr) {
            return std::nullopt;
        }
        return Vref(out);
    }

    /* Returns a stable hash derived from the variable name (Rust `vref.hash()`). */
    std::uint64_t hash() const {
        std::uint64_t out = 0;
        detail::check(lm_vref_hash(handle_, &out));
        return out;
    }

    /* Returns a stable hash derived from the variable name (Rust `vref.hash()`). */
    Variable to_var() const {
        LmVar *out = nullptr;
        detail::check(lm_vref_to_var(handle_, &out));
        return Variable(out);
    }

    /* Compares the variable index and the semantic contents of the environment
     * (Rust `PartialEq`). */
    bool operator==(const Vref &other) const {
        bool out = false;
        detail::check(lm_vref_eq(handle_, other.handle_, &out));
        return out;
    }
    bool operator!=(const Vref &other) const { return !(*this == other); }

    Expression pow(std::size_t n) const {
        LmExpr *out = nullptr;
        detail::check(lm_vref_pow(handle_, n, &out));
        return Expression(out);
    }

    bool eval(double value, std::optional<double> tol) const {
        bool out = false;
        detail::check(lm_vref_eval(handle_, value, tol ? &tol.value() : nullptr, &out));
        return out;
    }

    LmVref *get() const { return handle_; }

  private:
    LmVref *handle_;
};

inline Expression operator*(const Vref &lhs, const Vref &rhs) {
    LmExpr *out = nullptr;
    detail::check(lm_vref_mul_vref(lhs.get(), rhs.get(), &out));
    return Expression(out);
}

inline Expression operator*(const Vref &lhs, double rhs) {
    LmExpr *out = nullptr;
    detail::check(lm_vref_mul_bias(lhs.get(), rhs, &out));
    return Expression(out);
}

inline Expression operator*(double lhs, const Vref &rhs) { return rhs * lhs; }

inline Expression operator+(const Vref &lhs, const Vref &rhs) {
    LmExpr *out = nullptr;
    detail::check(lm_vref_add_vref(lhs.get(), rhs.get(), &out));
    return Expression(out);
}

inline Expression operator+(const Vref &lhs, double rhs) {
    LmExpr *out = nullptr;
    detail::check(lm_vref_add_bias(lhs.get(), rhs, &out));
    return Expression(out);
}

inline Expression operator+(double lhs, const Vref &rhs) { return rhs + lhs; }

inline Expression operator-(const Vref &lhs, const Vref &rhs) {
    LmExpr *out = nullptr;
    detail::check(lm_vref_sub_vref(lhs.get(), rhs.get(), &out));
    return Expression(out);
}

inline Expression operator-(const Vref &lhs, double rhs) {
    LmExpr *out = nullptr;
    detail::check(lm_vref_sub_bias(lhs.get(), rhs, &out));
    return Expression(out);
}

inline Expression operator-(const Vref &lhs) {
    LmExpr *out = nullptr;
    detail::check(lm_vref_neg(lhs.get(), &out));
    return Expression(out);
}

inline Vref operator!(const Vref &lhs) {
    LmVref *out = nullptr;
    detail::check(lm_vref_not(lhs.get(), &out));
    return Vref(out);
}

// TODO: use a C-API function here also.
// inline Expression operator-(double lhs, const Vref &rhs) { return rhs + lhs; }

} // namespace lm

template <> struct std::hash<lm::Vref> {
    std::size_t operator()(const lm::Vref &v) const { return v.hash(); }
};

#endif // LUNA_MODEL_VREF_HPP
