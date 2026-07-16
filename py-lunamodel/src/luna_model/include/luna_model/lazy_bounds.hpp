#ifndef LUNA_MODEL_LAZY_BOUNDS_HPP
#define LUNA_MODEL_LAZY_BOUNDS_HPP

#include <optional>

#include <luna_model.h>

#include "luna_model/bound.hpp"
#include "luna_model/bounds.hpp"
#include "luna_model/detail/capi.hpp"

namespace lm {

/* Owning handle to lazily-resolved variable bounds (frees on destruction).
 *
 * A `LazyBounds` defers picking concrete numeric bounds until it is
 * concretized against a variable type. */
class LazyBounds {
  public:
    /* Creates lazy bounds from optional lower/upper endpoints; `std::nullopt`
     * leaves that side to be filled in at concretization time. */
    LazyBounds(std::optional<Bound> lower = std::nullopt,
               std::optional<Bound> upper = std::nullopt) {
        LmBound lo{};
        LmBound hi{};
        if (lower) {
            lo = lower->get();
        }
        if (upper) {
            hi = upper->get();
        }
        LmLazyBounds *out = nullptr;
        detail::check(lm_lazy_bounds_new(lower ? &lo : nullptr, upper ? &hi : nullptr, &out));
        handle_ = out;
    }

    /* Creates the default lazy bounds. */
    static LazyBounds default_bounds() { return LazyBounds(lm_lazy_bounds_default()); }

    /* Adopts ownership of a raw C handle (for C interop). */
    explicit LazyBounds(LmLazyBounds *handle) noexcept : handle_(handle) {}

    ~LazyBounds() {
        if (handle_ != nullptr) {
            lm_lazy_bounds_free(handle_);
        }
    }

    LazyBounds(LazyBounds &&other) noexcept : handle_(other.handle_) { other.handle_ = nullptr; }
    LazyBounds &operator=(LazyBounds &&other) noexcept {
        if (this != &other) {
            if (handle_ != nullptr) {
                lm_lazy_bounds_free(handle_);
            }
            handle_ = other.handle_;
            other.handle_ = nullptr;
        }
        return *this;
    }
    LazyBounds(const LazyBounds &) = delete;
    LazyBounds &operator=(const LazyBounds &) = delete;

    /* The explicit lower endpoint, or `std::nullopt` if unspecified. */
    std::optional<Bound> lower() const {
        LmBound out{};
        LmStatus st = lm_lazy_bounds_lower(handle_, &out);
        if (st == LmStatus_Absent) {
            return std::nullopt;
        }
        detail::check(st);
        return Bound(out);
    }

    /* The explicit upper endpoint, or `std::nullopt` if unspecified. */
    std::optional<Bound> upper() const {
        LmBound out{};
        LmStatus st = lm_lazy_bounds_upper(handle_, &out);
        if (st == LmStatus_Absent) {
            return std::nullopt;
        }
        detail::check(st);
        return Bound(out);
    }

    /* Resolves the lazy bounds into concrete bounds for a variable type. */
    Bounds concretize(LmVtype vtype) const {
        LmBounds *out = nullptr;
        detail::check(lm_lazy_bounds_concretize(handle_, vtype, &out));
        return Bounds(out);
    }

    /* Clones into an independent handle. */
    LazyBounds clone() const {
        LmLazyBounds *out = nullptr;
        detail::check(lm_lazy_bounds_clone(handle_, &out));
        return LazyBounds(out);
    }

    LmLazyBounds *get() const noexcept { return handle_; }
    LmLazyBounds *release() noexcept {
        LmLazyBounds *h = handle_;
        handle_ = nullptr;
        return h;
    }

  private:
    LmLazyBounds *handle_;
};

} // namespace lm

#endif // LUNA_MODEL_LAZY_BOUNDS_HPP
