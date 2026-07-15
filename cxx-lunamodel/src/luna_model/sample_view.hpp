#ifndef LUNA_MODEL_SAMPLE_VIEW_HPP
#define LUNA_MODEL_SAMPLE_VIEW_HPP

#include <cstddef>
#include <string>
#include <vector>

#include <luna_model.h>

#include "luna_model/detail/capi.hpp"

namespace lm {

class SampleView {
  public:
    explicit SampleView(LmSampleView *handle) noexcept : handle_(handle) {}

    ~SampleView() {
        if (handle_ != nullptr) {
            lm_sample_view_free(handle_);
        }
    }

    SampleView(SampleView &&other) noexcept : handle_(other.handle_) { other.handle_ = nullptr; }
    SampleView &operator=(SampleView &&other) noexcept {
        if (this != &other) {
            if (handle_ != nullptr) {
                lm_sample_view_free(handle_);
            }
            handle_ = other.handle_;
            other.handle_ = nullptr;
        }
        return *this;
    }
    SampleView(const SampleView &) = delete;
    SampleView &operator=(const SampleView &) = delete;

    std::size_t index() const {
        std::size_t out = 0;
        detail::check(lm_sample_view_index(handle_, &out));
        return out;
    }

    std::vector<std::string> variable_names() const {
        LmStrBuffer buffer = {nullptr, 0, 0};
        detail::check(lm_sample_view_variable_names(handle_, &buffer));
        std::vector<std::string> out;
        out.reserve(buffer.len);
        for (std::size_t index = 0; index < buffer.len; ++index) {
            out.emplace_back(buffer.data[index]);
        }
        lm_str_buffer_free(buffer);
        return out;
    }

    LmAssignment assignment(const std::string &variable) const {
        LmAssignment out;
        detail::check(lm_sample_view_assignment(handle_, variable.c_str(), &out));
        return out;
    }

    LmAssignment assignment(std::size_t variable) const {
        LmAssignment out;
        detail::check(lm_sample_view_assignment_idx(handle_, variable, &out));
        return out;
    }

    LmSampleView *get() const noexcept { return handle_; }
    LmSampleView *release() noexcept {
        LmSampleView *handle = handle_;
        handle_ = nullptr;
        return handle;
    }

  private:
    LmSampleView *handle_;
};

} // namespace lm

#endif // LUNA_MODEL_SAMPLE_VIEW_HPP
