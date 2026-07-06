#ifndef LUNA_MODEL_COLUMN_HPP
#define LUNA_MODEL_COLUMN_HPP

#include <cstddef>

#include <luna_model.h>

#include "luna_model/detail/capi.hpp"

namespace lm {

/* Owning handle to a solution column: all values of one variable across the
 * sample rows (frees on destruction). Mirrors the Rust `Column`. */
class Column {
  public:
    /* Adopts ownership of a raw C handle (for C interop). */
    explicit Column(LmColumn *handle) noexcept : handle_(handle) {}

    ~Column() {
        if (handle_ != nullptr) {
            lm_col_free(handle_);
        }
    }

    Column(Column &&other) noexcept : handle_(other.handle_) { other.handle_ = nullptr; }
    Column &operator=(Column &&other) noexcept {
        if (this != &other) {
            if (handle_ != nullptr) {
                lm_col_free(handle_);
            }
            handle_ = other.handle_;
            other.handle_ = nullptr;
        }
        return *this;
    }
    Column(const Column &) = delete;
    Column &operator=(const Column &) = delete;

    /* Number of stored values / sample rows (Rust `Column::len`). */
    std::size_t len() const {
        std::size_t out = 0;
        detail::check(lm_col_len(handle_, &out));
        return out;
    }

    /* Whether the column has no stored values (Rust `Column::is_empty`). */
    bool is_empty() const {
        bool out = false;
        detail::check(lm_col_is_empty(handle_, &out));
        return out;
    }
    bool empty() const { return is_empty(); }

    /* Typed assignment at a row position (Rust `Column::as_assignment`).
     *
     * Throws `lm::Error` (status `Absent`) when `index` is out of range.
     */
    LmAssignment assignment(std::size_t index) const {
        LmAssignment out;
        detail::check(lm_col_assignment(handle_, index, &out));
        return out;
    }

    LmColumn *get() const noexcept { return handle_; }
    LmColumn *release() noexcept {
        LmColumn *h = handle_;
        handle_ = nullptr;
        return h;
    }

  private:
    LmColumn *handle_;
};

} // namespace lm

#endif // LUNA_MODEL_COLUMN_HPP
