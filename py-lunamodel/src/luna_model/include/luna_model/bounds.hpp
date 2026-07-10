#ifndef LUNA_MODEL_BOUNDS_HPP
#define LUNA_MODEL_BOUNDS_HPP

#include <luna_model.h>
#include <optional>

#include "luna_model/bound.hpp"
#include "luna_model/detail/capi.hpp"
#include "luna_model/funcs.h"

namespace lm {

/* Owning handle to a LunaModel bounds (frees on destruction).
 *
 * Mirrors the Rust `Bounds { lower: Bound, upper: Bound }`.
 */
class Bounds {
  public:
    /* Rust `Bounds::new(lower, upper)`. */
    Bounds(Bound lower, Bound upper) {
        LmBounds *out = nullptr;
        detail::check(lm_bounds_new(lower.get(), upper.get(), &out));
        handle_ = out;
    }

    /* Rust `Bounds::binary()` — [0, 1]. */
    static Bounds binary() {
        LmBounds *out = nullptr;
        detail::check(lm_bounds_binary(&out));
        return Bounds(out);
    }

    /* Rust `Bounds::spin()` — [-1, 1]. */
    static Bounds spin() {
        LmBounds *out = nullptr;
        detail::check(lm_bounds_spin(&out));
        return Bounds(out);
    }

    /* Rust `Bounds::integer()`. */
    static Bounds integer() {
        LmBounds *out = nullptr;
        detail::check(lm_bounds_integer(&out));
        return Bounds(out);
    }

    /* Rust `Bounds::real()`. */
    static Bounds real() {
        LmBounds *out = nullptr;
        detail::check(lm_bounds_real(&out));
        return Bounds(out);
    }

    /* Rust `Bounds::default_for(&vtype)` — the default bounds for a variable type. */
    static Bounds default_for(LmVtype vtype) {
        LmBounds *out = nullptr;
        detail::check(lm_bounds_default_for(vtype, &out));
        return Bounds(out);
    }

    /* Adopts ownership of a raw C handle (for C interop). */
    explicit Bounds(LmBounds *handle) noexcept : handle_(handle) {}

    ~Bounds() {
        if (handle_ != nullptr) {
            lm_bounds_free(handle_);
        }
    }

    Bounds(Bounds &&other) noexcept : handle_(other.handle_) { other.handle_ = nullptr; }
    Bounds &operator=(Bounds &&other) noexcept {
        if (this != &other) {
            if (handle_ != nullptr) {
                lm_bounds_free(handle_);
            }
            handle_ = other.handle_;
            other.handle_ = nullptr;
        }
        return *this;
    }
    Bounds(const Bounds &) = delete;
    Bounds &operator=(const Bounds &) = delete;

    /* Rust `bounds.lower` — a copy of the lower bound. */
    Bound lower() const {
        LmBound out;
        detail::check(lm_bounds_lower(handle_, &out));
        return Bound(out);
    }

    /* Rust `bounds.upper` — a copy of the upper bound. */
    Bound upper() const {
        LmBound out;
        detail::check(lm_bounds_upper(handle_, &out));
        return Bound(out);
    }

    /* Rust `Clone`. */
    Bounds clone() const {
        LmBounds *out = nullptr;
        detail::check(lm_bounds_clone(handle_, &out));
        return Bounds(out);
    }

    bool eval(double value, std::optional<double> tol) const {
        bool out = false;
        detail::check(lm_bounds_eval(handle_, value, tol ? &tol.value() : nullptr, &out));
        return out;
    }

    /* Rust `PartialEq` — compares both endpoints.
     * (No lm_bounds_eq in the C API; composed from the endpoint getters.) */
    bool operator==(const Bounds &other) const {
        return lower() == other.lower() && upper() == other.upper();
    }
    bool operator!=(const Bounds &other) const { return !(*this == other); }

    LmBounds *get() const noexcept { return handle_; }
    LmBounds *release() noexcept {
        LmBounds *h = handle_;
        handle_ = nullptr;
        return h;
    }

  private:
    LmBounds *handle_;
};

} // namespace lm

#endif // LUNA_MODEL_BOUNDS_HPP
