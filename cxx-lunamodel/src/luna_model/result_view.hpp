#ifndef LUNA_MODEL_RESULT_VIEW_HPP
#define LUNA_MODEL_RESULT_VIEW_HPP

#include <cstddef>
#include <map>
#include <optional>
#include <string>
#include <vector>

#include <luna_model.h>

#include "luna_model/detail/capi.hpp"
#include "luna_model/sample_view.hpp"

namespace lm {

namespace detail {

/* Reads a bool-map buffer into a `std::map`, releasing the buffer. */
inline std::map<std::string, std::vector<bool>> read_bool_map(LmBoolMapBuffer buf) {
    std::map<std::string, std::vector<bool>> out;
    for (std::size_t i = 0; i < buf.len; ++i) {
        const LmBoolMapEntry &e = buf.data[i];
        out.emplace(std::string(e.key), std::vector<bool>(e.data, e.data + e.len));
    }
    lm_bool_map_buffer_free(buf);
    return out;
}

} // namespace detail

/* Owning handle to the result metadata of one solution sample row (frees on
 * destruction). Accessors are lazy: quantities that were not evaluated read
 * back as `std::nullopt` (scalars) or empty maps. */
class ResultView {
  public:
    /* Adopts ownership of a raw C handle (for C interop). */
    explicit ResultView(LmResultView *handle) noexcept : handle_(handle) {}

    ~ResultView() {
        if (handle_ != nullptr) {
            lm_result_view_free(handle_);
        }
    }

    ResultView(ResultView &&other) noexcept : handle_(other.handle_) { other.handle_ = nullptr; }
    ResultView &operator=(ResultView &&other) noexcept {
        if (this != &other) {
            if (handle_ != nullptr) {
                lm_result_view_free(handle_);
            }
            handle_ = other.handle_;
            other.handle_ = nullptr;
        }
        return *this;
    }
    ResultView(const ResultView &) = delete;
    ResultView &operator=(const ResultView &) = delete;

    /* Occurrence count of this sample row. */
    std::size_t counts() const {
        std::size_t out = 0;
        detail::check(lm_result_view_counts(handle_, &out));
        return out;
    }

    /* Model-evaluated objective value, or `std::nullopt` if unevaluated. */
    std::optional<double> obj_value() const {
        double out = 0.0;
        LmStatus st = lm_result_view_obj_value(handle_, &out);
        if (st == LmStatus_Absent) {
            return std::nullopt;
        }
        detail::check(st);
        return out;
    }

    /* Raw solver energy, or `std::nullopt` if unevaluated. */
    std::optional<double> raw_energy() const {
        double out = 0.0;
        LmStatus st = lm_result_view_raw_energy(handle_, &out);
        if (st == LmStatus_Absent) {
            return std::nullopt;
        }
        detail::check(st);
        return out;
    }

    /* Feasibility flag, or `std::nullopt` if unevaluated. */
    std::optional<bool> feasible() const {
        bool out = false;
        LmStatus st = lm_result_view_feasible(handle_, &out);
        if (st == LmStatus_Absent) {
            return std::nullopt;
        }
        detail::check(st);
        return out;
    }

    /* Per-constraint feasibility flags keyed by name. */
    std::map<std::string, std::vector<bool>> constraints() const {
        LmBoolMapBuffer buf = {nullptr, 0, 0};
        detail::check(lm_result_view_constraints(handle_, &buf));
        return detail::read_bool_map(buf);
    }

    /* Per-variable bound-check flags keyed by name. */
    std::map<std::string, std::vector<bool>> variable_bounds() const {
        LmBoolMapBuffer buf = {nullptr, 0, 0};
        detail::check(lm_result_view_variable_bounds(handle_, &buf));
        return detail::read_bool_map(buf);
    }

    /* This row as an owned sample view. */
    SampleView sample() const {
        LmSampleView *out = nullptr;
        detail::check(lm_result_view_sample(handle_, &out));
        return SampleView(out);
    }

    LmResultView *get() const noexcept { return handle_; }
    LmResultView *release() noexcept {
        LmResultView *h = handle_;
        handle_ = nullptr;
        return h;
    }

  private:
    LmResultView *handle_;
};

} // namespace lm

#endif // LUNA_MODEL_RESULT_VIEW_HPP
