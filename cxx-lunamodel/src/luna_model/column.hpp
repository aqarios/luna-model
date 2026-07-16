#ifndef LUNA_MODEL_COLUMN_HPP
#define LUNA_MODEL_COLUMN_HPP

#include <cstddef>
#include <memory>
#include <optional>
#include <vector>

#include <luna_model.h>

#include "luna_model/detail/capi.hpp"

namespace lm {

/* Owning handle to a solution column: all values of one variable across the
 * sample rows (frees on destruction). */
class Column {
  public:
    static Column with(LmAssignment assignment) {
        LmColumn *out = nullptr;
        detail::check(lm_col_with(assignment, &out));
        return Column(out);
    }

    static Column binary(const std::vector<double> &data) {
        return from_values(LmVtype_Binary, data);
    }
    static Column spin(const std::vector<double> &data) { return from_values(LmVtype_Spin, data); }
    static Column integer(const std::vector<double> &data) {
        return from_values(LmVtype_Integer, data);
    }
    static Column real(const std::vector<double> &data) { return from_values(LmVtype_Real, data); }

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

    /* Number of stored values / sample rows. */
    std::size_t len() const {
        std::size_t out = 0;
        detail::check(lm_col_len(handle_, &out));
        return out;
    }

    /* Whether the column has no stored values. */
    bool is_empty() const {
        bool out = false;
        detail::check(lm_col_is_empty(handle_, &out));
        return out;
    }
    bool empty() const { return is_empty(); }

    /* The variable type of the column. */
    LmVtype vtype() const {
        LmVtype out;
        detail::check(lm_col_vtype(handle_, &out));
        return out;
    }

    /* Typed assignment at a row position.
     *
     * Throws `lm::Error` (status `Absent`) when `index` is out of range.
     */
    LmAssignment assignment(std::size_t index) const {
        LmAssignment out;
        detail::check(lm_col_assignment(handle_, index, &out));
        return out;
    }

    std::vector<LmAssignment> assignments() const {
        std::vector<LmAssignment> out;
        out.reserve(len());
        for (std::size_t index = 0; index < len(); ++index) {
            out.push_back(assignment(index));
        }
        return out;
    }

    std::vector<double> values() const {
        const double *data = nullptr;
        std::size_t size = 0;
        detail::check(lm_col_values(handle_, &data, &size));
        return std::vector<double>(data, data + size);
    }

    void push(LmAssignment assignment) { detail::check(lm_col_push(handle_, assignment)); }

    void try_push(double value, std::optional<double> tol = std::nullopt) {
        detail::check(lm_col_try_push(handle_, value, tol ? &*tol : nullptr));
    }

    void remove(std::size_t index) { detail::check(lm_col_remove(handle_, index)); }

    Column extract(std::size_t row) const {
        LmColumn *out = nullptr;
        detail::check(lm_col_extract(handle_, row, &out));
        return Column(out);
    }

    Column filter_by_mask(const std::vector<bool> &mask) const {
        std::unique_ptr<bool[]> values(new bool[mask.size()]);
        for (std::size_t index = 0; index < mask.size(); ++index) {
            values[index] = mask[index];
        }
        LmColumn *out = nullptr;
        detail::check(lm_col_filter_by_mask(handle_, values.get(), mask.size(), &out));
        return Column(out);
    }

    Column clone() const {
        LmColumn *out = nullptr;
        detail::check(lm_col_clone(handle_, &out));
        return Column(out);
    }

    bool operator==(const Column &other) const {
        bool out = false;
        detail::check(lm_col_eq(handle_, other.handle_, &out));
        return out;
    }
    bool operator!=(const Column &other) const { return !(*this == other); }

    LmColumn *get() const noexcept { return handle_; }
    LmColumn *release() noexcept {
        LmColumn *h = handle_;
        handle_ = nullptr;
        return h;
    }

  private:
    static Column from_values(LmVtype vtype, const std::vector<double> &data) {
        LmColumn *out = nullptr;
        detail::check(lm_col_new(vtype, data.data(), data.size(), &out));
        return Column(out);
    }

    LmColumn *handle_;
};

} // namespace lm

#endif // LUNA_MODEL_COLUMN_HPP
