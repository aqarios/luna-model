#ifndef LUNA_MODEL_VARIABLE_HPP
#define LUNA_MODEL_VARIABLE_HPP

#include <optional>
#include <string>

#include <luna_model.h>

#include "luna_model/bounds.hpp"
#include "luna_model/detail/capi.hpp"
#include "luna_model/lazy_bounds.hpp"

namespace lm {

/* Owning handle to a LunaModel variable definition (frees on destruction). */
class Variable {
  public:
    /* Creates a variable with the given name and type. `bounds == nullptr`
     * uses the default bounds for `vtype`. */
    Variable(const std::string &name, LmVtype vtype, const LazyBounds *bounds = nullptr) {
        LmVar *out = nullptr;
        detail::check(
            lm_var_new(name.c_str(), vtype, bounds ? bounds->get() : nullptr, &out));
        handle_ = out;
    }

    /* Adopts ownership of a raw C handle (for C interop). */
    explicit Variable(LmVar *handle) noexcept : handle_(handle) {}

    ~Variable() {
        if (handle_ != nullptr) {
            lm_var_free(handle_);
        }
    }

    Variable(Variable &&other) noexcept : handle_(other.handle_) { other.handle_ = nullptr; }
    Variable &operator=(Variable &&other) noexcept {
        if (this != &other) {
            if (handle_ != nullptr) {
                lm_var_free(handle_);
            }
            handle_ = other.handle_;
            other.handle_ = nullptr;
        }
        return *this;
    }
    Variable(const Variable &) = delete;
    Variable &operator=(const Variable &) = delete;

    /* Returns the variable name. */
    std::string name() const {
        LmBuffer buf = {nullptr, 0, 0};
        detail::check(lm_var_name(handle_, &buf));
        std::string out(reinterpret_cast<const char *>(buf.data), buf.len);
        lm_buffer_free(buf);
        return out;
    }

    /* Returns the variable type. */
    LmVtype vtype() const {
        LmVtype out;
        detail::check(lm_var_vtype(handle_, &out));
        return out;
    }

    /* Returns the variable bounds. */
    Bounds bounds() const {
        LmBounds *out = nullptr;
        detail::check(lm_var_bounds(handle_, &out));
        return Bounds(out);
    }

    /* Returns the variable's inverted companion id if one exists. */
    std::optional<std::uint32_t> inverted() const {
        std::uint32_t out = 0;
        LmStatus s = lm_var_inverted(handle_, &out);
        if (s == LmStatus_Absent) {
            return std::nullopt;
        }
        detail::check(s);
        return out;
    }

    /* Clones into an independent variable. */
    Variable clone() const {
        LmVar *out = nullptr;
        detail::check(lm_var_clone(handle_, &out));
        return Variable(out);
    }

    LmVar *get() const noexcept { return handle_; }
    LmVar *release() noexcept {
        LmVar *h = handle_;
        handle_ = nullptr;
        return h;
    }

  private:
    LmVar *handle_;
};

} // namespace lm

#endif // LUNA_MODEL_VARIABLE_HPP
