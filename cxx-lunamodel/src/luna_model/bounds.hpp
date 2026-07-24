#ifndef LUNA_MODEL_BOUNDS_HPP
#define LUNA_MODEL_BOUNDS_HPP

#include <luna_model.h>
#include <optional>

#include "luna_model/bound.hpp"
#include "luna_model/detail/capi.hpp"

namespace lm {

/* Owning handle to a concrete `[lower, upper]` bounds pair (frees on
 * destruction). */
class Bounds {
  public:
    /* Bounds with explicit lower and upper endpoints. */
    Bounds(Bound lower, Bound upper) {
        LmBounds *out = nullptr;
        detail::check(lm_bounds_new(lower.get(), upper.get(), &out));
        handle_ = out;
    }

    /* The binary domain, [0, 1]. */
    static Bounds binary() {
        LmBounds *out = nullptr;
        detail::check(lm_bounds_binary(&out));
        return Bounds(out);
    }

    /* The spin domain, [-1, 1]. */
    static Bounds spin() {
        LmBounds *out = nullptr;
        detail::check(lm_bounds_spin(&out));
        return Bounds(out);
    }

    /* The default integer domain. */
    static Bounds integer() {
        LmBounds *out = nullptr;
        detail::check(lm_bounds_integer(&out));
        return Bounds(out);
    }

    /* The default real domain. */
    static Bounds real() {
        LmBounds *out = nullptr;
        detail::check(lm_bounds_real(&out));
        return Bounds(out);
    }

    /* The default bounds for a variable type. */
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

    /* A copy of the lower endpoint. */
    Bound lower() const {
        LmBound out;
        detail::check(lm_bounds_lower(handle_, &out));
        return Bound(out);
    }

    /* A copy of the upper endpoint. */
    Bound upper() const {
        LmBound out;
        detail::check(lm_bounds_upper(handle_, &out));
        return Bound(out);
    }

    /* Clones into an independent handle. */
    Bounds clone() const {
        LmBounds *out = nullptr;
        detail::check(lm_bounds_clone(handle_, &out));
        return Bounds(out);
    }

    /* Whether `value` lies within the interval, within an optional tolerance. */
    bool eval(double value, std::optional<double> tol) const {
        bool out = false;
        detail::check(lm_bounds_eval(handle_, value, tol ? &tol.value() : nullptr, &out));
        return out;
    }

    /* Endpoint-wise equality. */
    bool operator==(const Bounds &other) const {
        bool out = false;
        detail::check(lm_bounds_eq(handle_, other.handle_, &out));
        return out;
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
