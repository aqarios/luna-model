#ifndef LUNA_MODEL_LINEAR_HPP
#define LUNA_MODEL_LINEAR_HPP

#include <cstddef>
#include <cstdint>
#include <vector>

#include <luna_model.h>

#include "luna_model/detail/capi.hpp"

namespace lm {

/* Owning handle to a LunaModel linear term store (frees on destruction).
 *
 * Mirrors the Rust `Linear`.
 */
class Linear {
  public:
    /* Creates an empty linear term store (Rust `Linear::default()`). */
    Linear() : handle_(lm_linear_default()) {}

    /* Creates a linear store with a single variable contribution
     * (Rust `Linear::for_var(var, bias)`). */
    static Linear for_var(std::uint32_t var, double bias) {
        LmLinear *out = nullptr;
        detail::check(lm_linear_for_var(var, bias, &out));
        return Linear(out);
    }

    /* Adopts ownership of a raw C handle (for C interop). */
    explicit Linear(LmLinear *handle) noexcept : handle_(handle) {}

    ~Linear() {
        if (handle_ != nullptr) {
            lm_linear_free(handle_);
        }
    }

    Linear(Linear &&other) noexcept : handle_(other.handle_) { other.handle_ = nullptr; }
    Linear &operator=(Linear &&other) noexcept {
        if (this != &other) {
            if (handle_ != nullptr) {
                lm_linear_free(handle_);
            }
            handle_ = other.handle_;
            other.handle_ = nullptr;
        }
        return *this;
    }
    Linear(const Linear &) = delete;
    Linear &operator=(const Linear &) = delete;

    /* Returns the number of explicitly stored entries (Rust `linear.len()`). */
    std::size_t len() const {
        std::size_t out = 0;
        detail::check(lm_linear_len(handle_, &out));
        return out;
    }

    /* Returns whether all stored biases sum to zero (Rust `linear.is_zero()`). */
    bool is_zero() const {
        bool out = false;
        detail::check(lm_linear_is_zero(handle_, &out));
        return out;
    }

    /* Returns whether the store contains no explicit terms
     * (Rust `linear.is_empty()`). */
    bool is_empty() const {
        bool out = false;
        detail::check(lm_linear_is_empty(handle_, &out));
        return out;
    }

    /* Returns the `(variable, bias)` items (Rust `linear.iter()`). */
    std::vector<LmLinearItem> items() const {
        LmLinearItemBuffer buf = {nullptr, 0, 0};
        detail::check(lm_linear_items(handle_, &buf));
        std::vector<LmLinearItem> out(buf.data, buf.data + buf.len);
        lm_linear_item_buffer_free(buf);
        return out;
    }

    /* Returns the bias for a variable index (Rust `linear[idx]`). */
    double operator[](std::uint32_t idx) const {
        double out = 0.0;
        detail::check(lm_linear_index(handle_, idx, &out));
        return out;
    }

    /* Adds a `(variable, bias)` contribution (Rust `linear += (id, bias)`). */
    void add_entry(std::uint32_t id, double bias) {
        detail::check(lm_linear_add_entry(handle_, id, bias));
    }

    /* Adds another linear store in place (Rust `linear += &other`). */
    Linear &operator+=(const Linear &other) {
        detail::check(lm_linear_add_linear(handle_, other.handle_));
        return *this;
    }

    /* Scales every bias in place (Rust `linear *= bias`). */
    Linear &operator*=(double bias) {
        detail::check(lm_linear_mul_bias(handle_, bias));
        return *this;
    }

    /* Returns a negated copy (Rust `-&linear`). */
    Linear operator-() const {
        LmLinear *out = nullptr;
        detail::check(lm_linear_neg(handle_, &out));
        return Linear(out);
    }

    /* Sets the bias for a variable index (Rust `linear[id] = bias`). */
    void set_bias(std::uint32_t id, double bias) {
        detail::check(lm_linear_set_bias(handle_, id, bias));
    }

    /* Rust `PartialEq`. */
    bool operator==(const Linear &other) const {
        bool out = false;
        detail::check(lm_linear_eq(handle_, other.handle_, &out));
        return out;
    }
    bool operator!=(const Linear &other) const { return !(*this == other); }

    /* Rust `Clone`. */
    Linear clone() const {
        LmLinear *out = nullptr;
        detail::check(lm_linear_clone(handle_, &out));
        return Linear(out);
    }

    LmLinear *get() const noexcept { return handle_; }
    LmLinear *release() noexcept {
        LmLinear *h = handle_;
        handle_ = nullptr;
        return h;
    }

  private:
    LmLinear *handle_;
};

} // namespace lm

#endif // LUNA_MODEL_LINEAR_HPP
