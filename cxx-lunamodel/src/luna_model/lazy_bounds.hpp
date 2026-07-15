#ifndef LUNA_MODEL_LAZY_BOUNDS_HPP
#define LUNA_MODEL_LAZY_BOUNDS_HPP

#include <optional>

#include <luna_model.h>

#include "luna_model/bound.hpp"
#include "luna_model/bounds.hpp"
#include "luna_model/detail/capi.hpp"
#include "luna_model/error.hpp"
#include "luna_model/types.h"

namespace lm {

class LazyBounds {
  public:
    LazyBounds() : handle_(lm_lazy_bounds_default()) {
        if (handle_ == nullptr) {
            throw Error(LmStatus_Panic, "failed to allocate LunaModel lazy bounds");
        }
    }

    LazyBounds(std::optional<Bound> lower, std::optional<Bound> upper) {
        const std::optional<LmBound> lower_raw = lower ? std::optional(lower->get()) : std::nullopt;
        const std::optional<LmBound> upper_raw = upper ? std::optional(upper->get()) : std::nullopt;
        LmLazyBounds *out = nullptr;
        detail::check(lm_lazy_bounds_new(lower_raw ? &*lower_raw : nullptr,
                                         upper_raw ? &*upper_raw : nullptr, &out));
        handle_ = out;
    }

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

    LazyBounds clone() const {
        LmLazyBounds *out = nullptr;
        detail::check(lm_lazy_bounds_clone(handle_, &out));
        return LazyBounds(out);
    }

    Bounds concretize(LmVtype vtype) const {
        LmBounds *out = nullptr;
        detail::check(lm_lazy_bounds_concretize(handle_, vtype, &out));
        return Bounds(out);
    }

    std::optional<Bound> lower() const { return endpoint(lm_lazy_bounds_lower); }
    std::optional<Bound> upper() const { return endpoint(lm_lazy_bounds_upper); }

    LmLazyBounds *get() const noexcept { return handle_; }
    LmLazyBounds *release() noexcept {
        LmLazyBounds *handle = handle_;
        handle_ = nullptr;
        return handle;
    }

  private:
    using EndpointGetter = LmStatus (*)(const LmLazyBounds *, LmBound *);

    std::optional<Bound> endpoint(EndpointGetter getter) const {
        LmBound out;
        const LmStatus status = getter(handle_, &out);
        if (status == LmStatus_Absent) {
            return std::nullopt;
        }
        detail::check(status);
        return Bound(out);
    }

    LmLazyBounds *handle_;
};

} // namespace lm
  //
#endif // LUNA_MODEL_LAZY_BOUNDS_HPP
