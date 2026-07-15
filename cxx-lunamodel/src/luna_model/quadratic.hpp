#ifndef LUNA_MODEL_QUADRATIC_HPP
#define LUNA_MODEL_QUADRATIC_HPP

#include <cstddef>
#include <cstdint>
#include <vector>

#include <luna_model.h>

#include "luna_model/detail/capi.hpp"

namespace lm {

/* Owning handle to a LunaModel quadratic term store (frees on destruction).
 *
 * Mirrors the Rust `Quadratic`.
 */
class Quadratic {
  public:
    /* Creates an empty quadratic term store (Rust `Quadratic::default()`). */
    Quadratic() : handle_(lm_quadratic_default()) {}

    /* Adopts ownership of a raw C handle (for C interop). */
    explicit Quadratic(LmQuadratic *handle) noexcept : handle_(handle) {}

    ~Quadratic() {
        if (handle_ != nullptr) {
            lm_quadratic_free(handle_);
        }
    }

    Quadratic(Quadratic &&other) noexcept : handle_(other.handle_) { other.handle_ = nullptr; }
    Quadratic &operator=(Quadratic &&other) noexcept {
        if (this != &other) {
            if (handle_ != nullptr) {
                lm_quadratic_free(handle_);
            }
            handle_ = other.handle_;
            other.handle_ = nullptr;
        }
        return *this;
    }
    Quadratic(const Quadratic &) = delete;
    Quadratic &operator=(const Quadratic &) = delete;

    /* Returns the number of explicitly stored entries (Rust `quadratic.len()`). */
    std::size_t len() const {
        std::size_t out = 0;
        detail::check(lm_quadratic_len(handle_, &out));
        return out;
    }

    /* Returns whether all stored biases sum to zero (Rust `quadratic.is_zero()`). */
    bool is_zero() const {
        bool out = false;
        detail::check(lm_quadratic_is_zero(handle_, &out));
        return out;
    }

    /* Returns whether the store contains no explicit terms
     * (Rust `quadratic.is_empty()`). */
    bool is_empty() const {
        bool out = false;
        detail::check(lm_quadratic_is_empty(handle_, &out));
        return out;
    }

    /* Returns the `(u, v, bias)` items (Rust `quadratic.iter_flat()`). */
    std::vector<LmQuadraticItem> items() const {
        LmQuadraticItemBuffer buf = {nullptr, 0, 0};
        detail::check(lm_quadratic_items(handle_, &buf));
        std::vector<LmQuadraticItem> out(buf.data, buf.data + buf.len);
        lm_quadratic_item_buffer_free(buf);
        return out;
    }

    /* Returns the bias for a variable pair (Rust `quadratic[(u, v)]`). */
    double operator()(std::uint32_t u, std::uint32_t v) const {
        double out = 0.0;
        detail::check(lm_quadratic_index(handle_, u, v, &out));
        return out;
    }

    /* Adds a `(u, v, bias)` contribution (Rust `quadratic += (u, v, bias)`). */
    void add_entry(std::uint32_t u, std::uint32_t v, double bias) {
        detail::check(lm_quadratic_add_entry(handle_, u, v, bias));
    }

    void clean() { detail::check(lm_quadratic_clean(handle_)); }

    void push_back_empty(std::uint32_t index) {
        detail::check(lm_quadratic_push_back_empty(handle_, index));
    }

    void insert_empty(std::size_t position, std::uint32_t index) {
        detail::check(lm_quadratic_insert_empty(handle_, position, index));
    }

    /* Adds another quadratic store in place (Rust `quadratic += &other`). */
    Quadratic &operator+=(const Quadratic &other) {
        detail::check(lm_quadratic_add_quadratic(handle_, other.handle_));
        return *this;
    }

    /* Scales every bias in place (Rust `quadratic *= bias`). */
    Quadratic &operator*=(double bias) {
        detail::check(lm_quadratic_mul_bias(handle_, bias));
        return *this;
    }

    /* Returns a negated copy (Rust `-&quadratic`). */
    Quadratic operator-() const {
        LmQuadratic *out = nullptr;
        detail::check(lm_quadratic_neg(handle_, &out));
        return Quadratic(out);
    }

    /* Sets the bias for a variable pair (Rust `quadratic[(u, v)] = bias`). */
    void set_bias(std::uint32_t u, std::uint32_t v, double bias) {
        detail::check(lm_quadratic_set_bias(handle_, u, v, bias));
    }

    /* Rust `PartialEq`. */
    bool operator==(const Quadratic &other) const {
        bool out = false;
        detail::check(lm_quadratic_eq(handle_, other.handle_, &out));
        return out;
    }
    bool operator!=(const Quadratic &other) const { return !(*this == other); }

    /* Rust `Clone`. */
    Quadratic clone() const {
        LmQuadratic *out = nullptr;
        detail::check(lm_quadratic_clone(handle_, &out));
        return Quadratic(out);
    }

    LmQuadratic *get() const noexcept { return handle_; }
    LmQuadratic *release() noexcept {
        LmQuadratic *h = handle_;
        handle_ = nullptr;
        return h;
    }

  private:
    LmQuadratic *handle_;
};

} // namespace lm

#endif // LUNA_MODEL_QUADRATIC_HPP
