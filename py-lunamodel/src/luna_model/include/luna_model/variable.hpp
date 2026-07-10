#ifndef LUNA_MODEL_VARIABLE_HPP
#define LUNA_MODEL_VARIABLE_HPP

#include <string>

#include <luna_model.h>

#include "luna_model/bounds.hpp"
#include "luna_model/detail/capi.hpp"

namespace lm {

class Variable {
  public:
    explicit Variable(LmVar *handle) noexcept : handle_(handle) {}

    ~Variable() {
        if (handle_ != nullptr) {
            lm_var_free(handle_);
        }
    }

    /* Returns the variable name (Rust `var.name()`). */
    std::string name() const {
        LmBuffer buf = {nullptr, 0, 0};
        detail::check(lm_var_name(handle_, &buf));
        std::string out(reinterpret_cast<const char *>(buf.data), buf.len);
        lm_buffer_free(buf);
        return out;
    }

    /* Returns the variable vtype (Rust `var.vtype()`). */
    LmVtype vtype() const {
        LmVtype out;
        detail::check(lm_var_vtype(handle_, &out));
        return out;
    }

    /* Returns the variable bounds (Rust `var.bounds()`). */
    Bounds bounds() const {
        LmBounds *out = nullptr;
        detail::check(lm_var_bounds(handle_, &out));
        return Bounds(out);
    }

    /* Returns the variable's inverted companion id if one exists (Rust `var.inverted()`). */
    std::optional<std::uint32_t> inverted() const {
        std::uint32_t out = 0;
        LmStatus s = lm_var_inverted(handle_, &out);
        if (s == LmStatus_Absent) {
            return std::nullopt;
        }
        detail::check(s);
        return out;
    }

  private:
    LmVar *handle_;
};

} // namespace lm

#endif // LUNA_MODEL_VARIABLE_HPP
