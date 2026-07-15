#ifndef LUNA_MODEL_RESULT_VIEW_HPP
#define LUNA_MODEL_RESULT_VIEW_HPP

#include <map>
#include <optional>
#include <string>

#include <luna_model.h>

#include "luna_model/detail/capi.hpp"
#include "luna_model/sample_view.hpp"

namespace lm {

class ResultView {
  public:
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

    std::size_t counts() const {
        std::size_t out = 0;
        detail::check(lm_result_view_counts(handle_, &out));
        return out;
    }

    std::optional<double> obj_value() const { return optional_value(lm_result_view_obj_value); }
    std::optional<double> raw_energy() const { return optional_value(lm_result_view_raw_energy); }

    std::optional<bool> feasible() const {
        bool out = false;
        const LmStatus status = lm_result_view_feasible(handle_, &out);
        if (status == LmStatus_Absent) {
            return std::nullopt;
        }
        detail::check(status);
        return out;
    }

    std::optional<std::map<std::string, bool>> constraints() const {
        return optional_flags(lm_result_view_constraints);
    }
    std::optional<std::map<std::string, bool>> variable_bounds() const {
        return optional_flags(lm_result_view_variable_bounds);
    }

    SampleView sample() const {
        LmSampleView *out = nullptr;
        detail::check(lm_result_view_sample(handle_, &out));
        return SampleView(out);
    }

    LmResultView *get() const noexcept { return handle_; }
    LmResultView *release() noexcept {
        LmResultView *handle = handle_;
        handle_ = nullptr;
        return handle;
    }

  private:
    using DoubleGetter = LmStatus (*)(const LmResultView *, double *);
    using FlagsGetter = LmStatus (*)(const LmResultView *, LmBoolMapBuffer *);

    std::optional<double> optional_value(DoubleGetter getter) const {
        double out = 0.0;
        const LmStatus status = getter(handle_, &out);
        if (status == LmStatus_Absent) {
            return std::nullopt;
        }
        detail::check(status);
        return out;
    }

    std::optional<std::map<std::string, bool>> optional_flags(FlagsGetter getter) const {
        LmBoolMapBuffer buffer = {nullptr, 0, 0};
        const LmStatus status = getter(handle_, &buffer);
        if (status == LmStatus_Absent) {
            return std::nullopt;
        }
        detail::check(status);
        std::map<std::string, bool> out;
        for (std::size_t index = 0; index < buffer.len; ++index) {
            const LmBoolMapEntry &entry = buffer.data[index];
            if (entry.len != 0) {
                out.emplace(entry.key, entry.data[0]);
            }
        }
        lm_bool_map_buffer_free(buffer);
        return out;
    }

    LmResultView *handle_;
};
} // namespace lm

#endif // LUNA_MODEL_RESULT_VIEW_HPP
