#ifndef LUNA_MODEL_SPECS_HPP
#define LUNA_MODEL_SPECS_HPP

#include <optional>
#include <string>
#include <vector>

#include <luna_model.h>

#include "luna_model/detail/capi.hpp"
#include "luna_model/types.h"

namespace lm {

class Specs {
  public:
    Specs(std::optional<LmSense> sense, std::optional<std::vector<LmVtype>> vtypes,
          std::optional<std::vector<LmCtype>> ctypes, std::optional<std::size_t> max_degree,
          std::optional<std::size_t> max_constraint_degree,
          std::optional<std::size_t> max_num_variables) {
        const LmVtype empty_vtype = LmVtype_Binary;
        const LmCtype empty_ctype = LmCtype_Unconstrained;
        const LmVtype *vtype_data =
            vtypes ? (vtypes->empty() ? &empty_vtype : vtypes->data()) : nullptr;
        const LmCtype *ctype_data =
            ctypes ? (ctypes->empty() ? &empty_ctype : ctypes->data()) : nullptr;
        LmSpecs *out = nullptr;
        detail::check(lm_specs_new(sense ? &*sense : nullptr, vtype_data,
                                   vtypes ? vtypes->size() : 0, ctype_data,
                                   ctypes ? ctypes->size() : 0, max_degree ? &*max_degree : nullptr,
                                   max_constraint_degree ? &*max_constraint_degree : nullptr,
                                   max_num_variables ? &*max_num_variables : nullptr, &out));
        handle_ = out;
    }

    /* Adopts ownership of a raw C handle (for C interop). */
    explicit Specs(LmSpecs *handle) noexcept : handle_(handle) {}

    ~Specs() {
        if (handle_ != nullptr) {
            lm_specs_free(handle_);
        }
    }

    Specs(Specs &&other) noexcept : handle_(other.handle_) { other.handle_ = nullptr; }
    Specs &operator=(Specs &&other) noexcept {
        if (this != &other) {
            if (handle_ != nullptr) {
                lm_specs_free(handle_);
            }
            handle_ = other.handle_;
            other.handle_ = nullptr;
        }
        return *this;
    }
    Specs(const Specs &) = delete;
    Specs &operator=(const Specs &) = delete;

    Specs clone() const {
        LmSpecs *out = nullptr;
        detail::check(lm_specs_clone(handle_, &out));
        return Specs(out);
    }

    bool satisfies(const Specs &other) const {
        bool out = false;
        detail::check(lm_specs_satisfies(handle_, other.handle_, &out));
        return out;
    }

    std::string diff(const Specs &other) const {
        LmBuffer buffer = {nullptr, 0, 0};
        detail::check(lm_specs_diff(handle_, other.handle_, &buffer));
        std::string out =
            buffer.len == 0 ? std::string()
                            : std::string(reinterpret_cast<const char *>(buffer.data), buffer.len);
        lm_buffer_free(buffer);
        return out;
    }

    std::optional<LmSense> sense() const {
        LmSense out;
        const LmStatus status = lm_specs_sense(handle_, &out);
        if (status == LmStatus_Absent) {
            return std::nullopt;
        }
        detail::check(status);
        return out;
    }

    std::optional<std::vector<LmVtype>> vtypes() const {
        LmVtypeBuffer buffer = {nullptr, 0, 0};
        const LmStatus status = lm_specs_vtypes(handle_, &buffer);
        if (status == LmStatus_Absent) {
            return std::nullopt;
        }
        detail::check(status);
        std::vector<LmVtype> out(buffer.data, buffer.data + buffer.len);
        lm_vtype_buffer_free(buffer);
        return out;
    }

    std::optional<std::vector<LmCtype>> ctypes() const {
        LmCtypeBuffer buffer = {nullptr, 0, 0};
        const LmStatus status = lm_specs_ctypes(handle_, &buffer);
        if (status == LmStatus_Absent) {
            return std::nullopt;
        }
        detail::check(status);
        std::vector<LmCtype> out(buffer.data, buffer.data + buffer.len);
        lm_ctype_buffer_free(buffer);
        return out;
    }

    std::optional<std::size_t> max_degree() const { return optional_size(lm_specs_max_degree); }
    std::optional<std::size_t> max_constraint_degree() const {
        return optional_size(lm_specs_max_constraint_degree);
    }
    std::optional<std::size_t> max_num_variables() const {
        return optional_size(lm_specs_max_num_variables);
    }

    LmSpecs *get() const noexcept { return handle_; }
    LmSpecs *release() noexcept {
        LmSpecs *handle = handle_;
        handle_ = nullptr;
        return handle;
    }

  private:
    using SizeGetter = LmStatus (*)(const LmSpecs *, std::size_t *);

    std::optional<std::size_t> optional_size(SizeGetter getter) const {
        std::size_t out = 0;
        const LmStatus status = getter(handle_, &out);
        if (status == LmStatus_Absent) {
            return std::nullopt;
        }
        detail::check(status);
        return out;
    }

    LmSpecs *handle_;
};

} // namespace lm
  //
#endif // LUNA_MODEL_SPECS_HPP
