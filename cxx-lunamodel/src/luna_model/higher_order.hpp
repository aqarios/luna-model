#ifndef LUNA_MODEL_HIGHER_ORDER_HPP
#define LUNA_MODEL_HIGHER_ORDER_HPP

#include <cstddef>
#include <cstdint>
#include <vector>

#include <luna_model.h>

#include "luna_model/detail/capi.hpp"

namespace lm {

/* One higher-order contribution: the participating variable indices and the
 * bias. Owning C++ view of `LmHigherOrderItem`. */
struct HigherOrderItem {
    std::vector<std::uint32_t> indices;
    double bias;
};

/* Owning handle to a higher-order term store (frees on destruction). */
class HigherOrder {
  public:
    /* Creates an empty higher-order term store. */
    HigherOrder() : handle_(lm_higher_order_default()) {}

    /* Creates a store with a pre-allocated capacity. */
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

    /* Number of explicitly stored entries. */
    std::size_t len() const {
        std::size_t out = 0;
        detail::check(lm_higher_order_len(handle_, &out));
        return out;
    }

    /* Whether all stored biases sum to zero. */
    bool is_zero() const {
        bool out = false;
        detail::check(lm_higher_order_is_zero(handle_, &out));
        return out;
    }

    /* Whether no effective contribution is present. */
    bool is_empty() const {
        bool out = false;
        detail::check(lm_higher_order_is_empty(handle_, &out));
        return out;
    }

    /* The degree of the term store. */
    std::size_t degree() const {
        std::size_t out = 0;
        detail::check(lm_higher_order_degree(handle_, &out));
        return out;
    }

    /* The `(indices, bias)` items. */
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

    /* The bias for a variable tuple. */
    double operator()(const std::vector<std::uint32_t> &indices) const {
        double out = 0.0;
        detail::check(lm_higher_order_index(handle_, indices.data(), indices.size(), &out));
        return out;
    }

    /* Adds an `(indices, bias)` contribution. */
    void add_entry(const std::vector<std::uint32_t> &indices, double bias) {
        detail::check(lm_higher_order_add_entry(handle_, indices.data(), indices.size(), bias));
    }

    /* Adds another higher-order store in place. */
    HigherOrder &operator+=(const HigherOrder &other) {
        detail::check(lm_higher_order_add_higher_order(handle_, other.handle_));
        return *this;
    }

    /* Scales every bias in place. */
    HigherOrder &operator*=(double bias) {
        detail::check(lm_higher_order_mul_bias(handle_, bias));
        return *this;
    }

    /* Returns a negated copy. */
    HigherOrder operator-() const {
        LmHigherOrder *out = nullptr;
        detail::check(lm_higher_order_neg(handle_, &out));
        return HigherOrder(out);
    }

    /* Sets the bias for a variable tuple. */
    void set_bias(const std::vector<std::uint32_t> &indices, double bias) {
        detail::check(lm_higher_order_set_bias(handle_, indices.data(), indices.size(), bias));
    }

    /* Drops stored terms whose bias is zero. */
    void clean() { detail::check(lm_higher_order_clean(handle_)); }

    bool operator==(const HigherOrder &other) const {
        bool out = false;
        detail::check(lm_higher_order_eq(handle_, other.handle_, &out));
        return out;
    }
    bool operator!=(const HigherOrder &other) const { return !(*this == other); }

    /* Clones into an independent handle. */
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
