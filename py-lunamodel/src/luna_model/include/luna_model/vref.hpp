#ifndef LUNA_MODEL_VREF_HPP
#define LUNA_MODEL_VREF_HPP

#include <cstddef>
#include <cstdint>
#include <optional>
#include <string>

#include <luna_model.h>

#include "luna_model/bounds.hpp"
#include "luna_model/detail/capi.hpp"
#include "luna_model/variable.hpp"

namespace lm {

// `Vref` deliberately does NOT depend on the full `Env` type: the include graph
// flows one way, `env -> vref`. `Env` is referenced only by the id-based
// constructor below, whose definition lives in env/creation.hpp (where `Env` is
// complete). This keeps vref.hpp free of any reverse include of env.hpp.
class Env;        // defined in env/env.hpp
class Expression; // defined in expr.hpp; ops are defined in ops.hpp

/* A reference to a variable within an environment (frees on destruction). */
class Vref {
  public:
    /* Creates a new reference to variable `id` within `env`. Defined out-of-line
     * in env/creation.hpp, where `Env` is complete. */
    Vref(std::uint32_t id, const Env &env);

    /* Adopts ownership of a raw C handle (for C interop). */
    explicit Vref(LmVref *handle) noexcept : handle_(handle) {}

    ~Vref() {
        if (handle_ != nullptr) {
            lm_vref_free(handle_);
        }
    }

    Vref(Vref &&other) noexcept : handle_(other.handle_) { other.handle_ = nullptr; }
    Vref &operator=(Vref &&other) noexcept {
        if (this != &other) {
            if (handle_ != nullptr) {
                lm_vref_free(handle_);
            }
            handle_ = other.handle_;
            other.handle_ = nullptr;
        }
        return *this;
    }
    Vref(const Vref &) = delete;
    Vref &operator=(const Vref &) = delete;

    /* The variable's index within its environment. */
    std::uint32_t id() const {
        std::uint32_t out = 0;
        detail::check(lm_vref_id(handle_, &out));
        return out;
    }

    /* Whether the referenced variable still exists in its environment. */
    bool check_living() const {
        bool out = false;
        detail::check(lm_vref_check_living(handle_, &out));
        return out;
    }

    /* The variable name. */
    std::string name() const {
        LmBuffer buf = {nullptr, 0, 0};
        detail::check(lm_vref_name(handle_, &buf));
        std::string out(reinterpret_cast<const char *>(buf.data), buf.len);
        lm_buffer_free(buf);
        return out;
    }

    /* The variable type. */
    LmVtype vtype() const {
        LmVtype out = {};
        detail::check(lm_vref_vtype(handle_, &out));
        return out;
    }

    /* The variable bounds. */
    Bounds bounds() const {
        LmBounds *out = nullptr;
        detail::check(lm_vref_bounds(handle_, &out));
        return Bounds(out);
    }

    /* The inverted companion of this reference, if one exists. */
    std::optional<Vref> inverted() const {
        LmVref *out = nullptr;
        detail::check(lm_vref_inverted(handle_, &out));
        if (out == nullptr) {
            return std::nullopt;
        }
        return Vref(out);
    }

    /* A stable hash derived from the variable name. */
    std::uint64_t hash() const {
        std::uint64_t out = 0;
        detail::check(lm_vref_hash(handle_, &out));
        return out;
    }

    /* Materializes the referenced environment entry as an owned variable. */
    Variable to_var() const {
        LmVar *out = nullptr;
        detail::check(lm_vref_to_var(handle_, &out));
        return Variable(out);
    }

    /* Compares variable index and the semantic contents of the environment. */
    bool operator==(const Vref &other) const {
        bool out = false;
        detail::check(lm_vref_eq(handle_, other.handle_, &out));
        return out;
    }
    bool operator!=(const Vref &other) const { return !(*this == other); }

    /* Raises the variable to the power `n`, promoting to an expression.
     * Defined in ops.hpp. */
    Expression pow(std::size_t n) const;

    /* Whether `value` satisfies the variable's bounds; `std::nullopt` tol selects
     * the default tolerance. */
    bool eval(double value, std::optional<double> tol = std::nullopt) const {
        bool out = false;
        detail::check(lm_vref_eval(handle_, value, tol ? &tol.value() : nullptr, &out));
        return out;
    }

    /* Clones into an independent reference to the same variable. */
    Vref clone() const {
        LmVref *out = nullptr;
        detail::check(lm_vref_clone(handle_, &out));
        return Vref(out);
    }

    LmVref *get() const noexcept { return handle_; }
    LmVref *release() noexcept {
        LmVref *h = handle_;
        handle_ = nullptr;
        return h;
    }

  private:
    LmVref *handle_;
};

} // namespace lm

template <> struct std::hash<lm::Vref> {
    std::size_t operator()(const lm::Vref &v) const { return v.hash(); }
};

#endif // LUNA_MODEL_VREF_HPP
