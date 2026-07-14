#ifndef LUNA_MODEL_SPECS_HPP
#define LUNA_MODEL_SPECS_HPP

#include <cstddef>
#include <optional>
#include <vector>

#include <luna_model.h>

#include "luna_model/detail/capi.hpp"

namespace lm {

/* Owning handle to a model specification: the structural properties a model is
 * expected to satisfy (frees on destruction). */
class Specs {
  public:
    /* Builds a specification. Any argument left empty (`std::nullopt` / empty
     * vector) is treated as unspecified. */
    Specs(std::optional<LmSense> sense, const std::vector<LmVtype> &vtypes,
          const std::vector<LmCtype> &ctypes, std::optional<std::size_t> max_degree = std::nullopt,
          std::optional<std::size_t> max_constraint_degree = std::nullopt,
          std::optional<std::size_t> max_num_variables = std::nullopt) {
        LmSpecs *out = nullptr;
        detail::check(lm_specs_new(sense ? &*sense : nullptr, vtypes.data(), vtypes.size(),
                                   ctypes.data(), ctypes.size(),
                                   max_degree ? &*max_degree : nullptr,
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

    /* The optimization sense. */
    LmSense sense() const {
        LmSense out;
        detail::check(lm_specs_sense(handle_, &out));
        return out;
    }

    /* The maximum objective degree. */
    std::size_t max_degree() const {
        std::size_t out = 0;
        detail::check(lm_specs_max_degree(handle_, &out));
        return out;
    }

    /* The maximum constraint degree. */
    std::size_t max_constraint_degree() const {
        std::size_t out = 0;
        detail::check(lm_specs_max_constraint_degree(handle_, &out));
        return out;
    }

    /* The maximum number of variables. */
    std::size_t max_num_variables() const {
        std::size_t out = 0;
        detail::check(lm_specs_max_num_variables(handle_, &out));
        return out;
    }

    /* The allowed variable types. */
    std::vector<LmVtype> vtypes() const {
        LmVtypeBuffer buf = {nullptr, 0, 0};
        detail::check(lm_specs_vtypes(handle_, &buf));
        std::vector<LmVtype> out(buf.data, buf.data + buf.len);
        lm_vtype_buffer_free(buf);
        return out;
    }

    /* The allowed constraint types. */
    std::vector<LmCtype> ctypes() const {
        LmCtypeBuffer buf = {nullptr, 0, 0};
        detail::check(lm_specs_ctypes(handle_, &buf));
        std::vector<LmCtype> out(buf.data, buf.data + buf.len);
        lm_ctype_buffer_free(buf);
        return out;
    }

    /* Whether this specification satisfies (is at least as strict as) `other`. */
    bool satisfies(const Specs &other) const {
        bool out = false;
        detail::check(lm_specs_satisfies(handle_, other.handle_, &out));
        return out;
    }

    /* Clones into an independent handle. */
    Specs clone() const {
        LmSpecs *out = nullptr;
        detail::check(lm_specs_clone(handle_, &out));
        return Specs(out);
    }

    LmSpecs *get() const noexcept { return handle_; }
    LmSpecs *release() noexcept {
        LmSpecs *h = handle_;
        handle_ = nullptr;
        return h;
    }

  private:
    LmSpecs *handle_;
};

} // namespace lm

#endif // LUNA_MODEL_SPECS_HPP
