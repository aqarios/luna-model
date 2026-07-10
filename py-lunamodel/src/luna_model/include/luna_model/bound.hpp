#ifndef LUNA_MODEL_BOUND_HPP
#define LUNA_MODEL_BOUND_HPP

#include <optional>

#include <luna_model.h>

namespace lm {

/* Value type mirroring the Rust `Bound` enum: `Bounded(f64)` or `Unbounded`.
 *
 * Thin wrapper over the C `LmBound` POD; copyable and trivially cheap.
 */
class Bound {
  public:
    /* Rust `Bound::Bounded(value)` — a finite inclusive bound. */
    static Bound bounded(double value) noexcept {
        return Bound(LmBound{LmBoundTag_Bounded, value});
    }

    /* Rust `Bound::Unbounded` — no bound on this side of the interval. */
    static Bound unbounded() noexcept { return Bound(LmBound{LmBoundTag_Unbounded, 0.0}); }

    /* Adopts a raw C value (for C interop). */
    explicit Bound(LmBound raw) noexcept : raw_(raw) {}

    /* Rust `bound.is_bounded()`. */
    bool is_bounded() const noexcept { return raw_.tag == LmBoundTag_Bounded; }

    /* The finite value, or `std::nullopt` when unbounded.
     * This is the C++ spelling of pattern-matching the Rust enum. */
    std::optional<double> value() const noexcept {
        if (raw_.tag == LmBoundTag_Bounded) {
            return raw_.value;
        }
        return std::nullopt;
    }

    /* Matches Rust `PartialEq`: unbounded == unbounded, bounded compares values. */
    bool operator==(const Bound &other) const noexcept {
        if (raw_.tag != other.raw_.tag) {
            return false;
        }
        return raw_.tag == LmBoundTag_Unbounded || raw_.value == other.raw_.value;
    }
    bool operator!=(const Bound &other) const noexcept { return !(*this == other); }

    /* The raw C value (for passing to lm_bounds_new etc.). */
    LmBound get() const noexcept { return raw_; }

  private:
    LmBound raw_;
};

} // namespace lm

#endif // LUNA_MODEL_BOUND_HPP
