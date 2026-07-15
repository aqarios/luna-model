#ifndef LUNA_MODEL_HIGHER_ORDER_HPP
#define LUNA_MODEL_HIGHER_ORDER_HPP

#include <cstddef>
#include <cstdint>
#include <vector>

#include <luna_model.h>

#include "luna_model/detail/capi.hpp"

namespace lm {

/* One higher-order contribution: the participating variable indices and the
 * bias. C++ view of `LmHigherOrderItem` with owned storage. */
struct HigherOrderItem {
    std::vector<std::uint32_t> indices;
    double bias;
};

/* Owning handle to a LunaModel higher-order term store (frees on destruction).
 *
 * Mirrors the Rust `HigherOrder`.
 */
class HigherOrder {
  public:
    /* Creates an empty higher-order term store (Rust `HigherOrder::default()`). */
    HigherOrder() : handle_(lm_higher_order_default()) {}

    /* Creates a store with a pre-allocated capacity
     * (Rust `HigherOrder::with_capacity(cap)`). */
    static HigherOrder with_capacity(std::size_t cap) {
        return HigherOrder(lm_higher_order_with_capacity(cap));
    }

    /* Adopts ownership of a raw C handle (for C interop). */
    explicit HigherOrder(LmHigherOrder *handle) noexcept : handle_(handle) {}

    ~HigherOrder() {
        if (handle_ != nullptr) {
            lm_higher_order_free(handle_);
        }
    }

    HigherOrder(HigherOrder &&other) noexcept : handle_(other.handle_) { other.handle_ = nullptr; }
    HigherOrder &operator=(HigherOrder &&other) noexcept {
        if (this != &other) {
            if (handle_ != nullptr) {
                lm_higher_order_free(handle_);
            }
            handle_ = other.handle_;
            other.handle_ = nullptr;
        }
        return *this;
    }
    HigherOrder(const HigherOrder &) = delete;
    HigherOrder &operator=(const HigherOrder &) = delete;

    /* Returns the number of explicitly stored entries (Rust `ho.len()`). */
    std::size_t len() const {
        std::size_t out = 0;
        detail::check(lm_higher_order_len(handle_, &out));
        return out;
    }

    /* Returns whether all stored biases sum to zero (Rust `ho.is_zero()`). */
    bool is_zero() const {
        bool out = false;
        detail::check(lm_higher_order_is_zero(handle_, &out));
        return out;
    }

    /* Returns whether no effective contribution is present (Rust `ho.is_empty()`). */
    bool is_empty() const {
        bool out = false;
        detail::check(lm_higher_order_is_empty(handle_, &out));
        return out;
    }

    /* Returns the degree of the term store (Rust `ho.degree()`). */
    std::size_t degree() const {
        std::size_t out = 0;
        detail::check(lm_higher_order_degree(handle_, &out));
        return out;
    }

    /* Returns the `(indices, bias)` items (Rust `ho.iter_contrib()`). */
    std::vector<HigherOrderItem> items() const {
        LmHigherOrderItemBuffer buf = {nullptr, 0, 0};
        detail::check(lm_higher_order_items(handle_, &buf));
        std::vector<HigherOrderItem> out;
        out.reserve(buf.len);
        for (std::size_t i = 0; i < buf.len; ++i) {
            const LmU32Buffer &idx = buf.data[i].indices;
            out.push_back(HigherOrderItem{
                std::vector<std::uint32_t>(idx.data, idx.data + idx.len),
                buf.data[i].bias,
            });
        }
        /* Copies were taken above, so freeing the buffer (and its nested
         * index buffers) is safe. */
        lm_higher_order_item_buffer_free(buf);
        return out;
    }

    /* Returns the bias for a variable tuple (Rust `ho[indices]`). */
    double operator()(const std::vector<std::uint32_t> &indices) const {
        double out = 0.0;
        detail::check(lm_higher_order_index(handle_, indices.data(), indices.size(), &out));
        return out;
    }

    /* Adds an `(indices, bias)` contribution (Rust `ho += (indices, bias)`). */
    void add_entry(const std::vector<std::uint32_t> &indices, double bias) {
        detail::check(lm_higher_order_add_entry(handle_, indices.data(), indices.size(), bias));
    }

    void clean() { detail::check(lm_higher_order_clean(handle_)); }

    /* Adds another higher-order store in place (Rust `ho += &other`). */
    HigherOrder &operator+=(const HigherOrder &other) {
        detail::check(lm_higher_order_add_higher_order(handle_, other.handle_));
        return *this;
    }

    /* Scales every bias in place (Rust `ho *= bias`). */
    HigherOrder &operator*=(double bias) {
        detail::check(lm_higher_order_mul_bias(handle_, bias));
        return *this;
    }

    /* Returns a negated copy (Rust `-&ho`). */
    HigherOrder operator-() const {
        LmHigherOrder *out = nullptr;
        detail::check(lm_higher_order_neg(handle_, &out));
        return HigherOrder(out);
    }

    /* Sets the bias for a variable tuple (Rust `ho[indices] = bias`). */
    void set_bias(const std::vector<std::uint32_t> &indices, double bias) {
        detail::check(lm_higher_order_set_bias(handle_, indices.data(), indices.size(), bias));
    }

    /* Rust `PartialEq`. */
    bool operator==(const HigherOrder &other) const {
        bool out = false;
        detail::check(lm_higher_order_eq(handle_, other.handle_, &out));
        return out;
    }
    bool operator!=(const HigherOrder &other) const { return !(*this == other); }

    /* Rust `Clone`. */
    HigherOrder clone() const {
        LmHigherOrder *out = nullptr;
        detail::check(lm_higher_order_clone(handle_, &out));
        return HigherOrder(out);
    }

    LmHigherOrder *get() const noexcept { return handle_; }
    LmHigherOrder *release() noexcept {
        LmHigherOrder *h = handle_;
        handle_ = nullptr;
        return h;
    }

  private:
    LmHigherOrder *handle_;
};

} // namespace lm

#endif // LUNA_MODEL_HIGHER_ORDER_HPP
