#ifndef LUNA_MODEL_SAMPLE_VIEW_HPP
#define LUNA_MODEL_SAMPLE_VIEW_HPP

#include <cstddef>
#include <string>
#include <vector>

#include <luna_model.h>

#include "luna_model/detail/capi.hpp"

namespace lm {

/* Owning handle to a single sample row of a solution (frees on destruction).
 *
 * A sample view exposes one row's variable assignments by name or by column
 * index. */
class SampleView {
  public:
    /* Adopts ownership of a raw C handle (for C interop). */
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

    /* The row's position within the solution. */
    std::size_t index() const {
        std::size_t out = 0;
        detail::check(lm_sample_view_index(handle_, &out));
        return out;
    }

    /* The variable names in column order. */
    std::vector<std::string> variable_names() const {
        LmStrBuffer buf = {nullptr, 0, 0};
        detail::check(lm_sample_view_variable_names(handle_, &buf));
        std::vector<std::string> out;
        out.reserve(buf.len);
        for (std::size_t i = 0; i < buf.len; ++i) {
            out.emplace_back(buf.data[i]);
        }
        lm_str_buffer_free(buf);
        return out;
    }

    /* The typed assignment for the variable named `var`. */
    LmAssignment assignment(const std::string &var) const {
        LmAssignment out;
        detail::check(lm_sample_view_assignment(handle_, var.c_str(), &out));
        return out;
    }

    /* The typed assignment at column position `var`. */
    LmAssignment assignment_idx(std::size_t var) const {
        LmAssignment out;
        detail::check(lm_sample_view_assignment_idx(handle_, var, &out));
        return out;
    }

    LmSampleView *get() const noexcept { return handle_; }
    LmSampleView *release() noexcept {
        LmSampleView *h = handle_;
        handle_ = nullptr;
        return h;
    }

  private:
    LmSampleView *handle_;
};

} // namespace lm

#endif // LUNA_MODEL_SAMPLE_VIEW_HPP
