#ifndef LUNA_MODEL_SAMPLE_VIEW_HPP
#define LUNA_MODEL_SAMPLE_VIEW_HPP

#include <cstddef>
#include <map>
#include <string>
#include <utility>
#include <vector>

#include <luna_model.h>

#include "luna_model/detail/capi.hpp"
#include "luna_model/vref.hpp"

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

    /* The typed assignment for the variable named `var`. */
    LmAssignment get(const std::string &var) const { return assignment(var); }

    /* The typed assignment selected by column index. */
    LmAssignment try_get(std::size_t var) const { return assignment_idx(var); }
    /* The typed assignment for the given variable reference. */
    LmAssignment try_get(const Vref &var) const { return assignment(var.name()); }
    /* The typed assignment for the variable named `var`. */
    LmAssignment try_get(const std::string &var) const { return assignment(var); }

    /* All typed assignments of the row in column order. */
    std::vector<LmAssignment> to_vec() const {
        std::size_t n = variable_names().size();
        std::vector<LmAssignment> out;
        out.reserve(n);
        for (std::size_t i = 0; i < n; ++i) {
            out.push_back(assignment_idx(i));
        }
        return out;
    }

    /* The raw numeric values of the row in column order. */
    std::vector<double> iter() const {
        std::vector<double> out;
        for (const LmAssignment &a : to_vec()) {
            out.push_back(a.value);
        }
        return out;
    }

    /* The `(name, value)` pairs of the row in column order. */
    std::vector<std::pair<std::string, double>> iter_named() const {
        std::vector<std::string> names = variable_names();
        std::vector<LmAssignment> values = to_vec();
        std::vector<std::pair<std::string, double>> out;
        out.reserve(names.size());
        for (std::size_t i = 0; i < names.size(); ++i) {
            out.emplace_back(std::move(names[i]), values[i].value);
        }
        return out;
    }

    /* The row as a `name -> value` map. */
    std::map<std::string, double> as_map() const {
        std::map<std::string, double> out;
        for (auto &[name, value] : iter_named()) {
            out.emplace(std::move(name), value);
        }
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
