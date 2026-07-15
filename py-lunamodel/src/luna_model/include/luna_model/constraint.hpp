#ifndef LUNA_MODEL_CONSTRAINT_HPP
#define LUNA_MODEL_CONSTRAINT_HPP

#include <optional>
#include <string>
#include <vector>

#include <luna_model.h>

#include "luna_model/detail/capi.hpp"
#include "luna_model/env.hpp"
#include "luna_model/expr.hpp"
#include "luna_model/types.h"
#include "luna_model/vref.hpp"

namespace lm {

class Constraint {
  public:
    /* Creates a new Constraint. */
    explicit Constraint(Expression lhs, double rhs, LmComparator cmp,
                        std::optional<std::string> name) {
        LmConstraint *out = nullptr;
        detail::check(lm_constr_new(lhs.get(), rhs, cmp, name ? name->c_str() : nullptr, &out));
        handle_ = out;
    }

    /* Adopts ownership of a raw C handle (for C interop). */
    explicit Constraint(LmConstraint *handle) noexcept : handle_(handle) {}

    ~Constraint() {
        if (handle_ != nullptr) {
            lm_constr_free(handle_);
        }
    }

    Constraint(Constraint &&other) noexcept : handle_(other.handle_) { other.handle_ = nullptr; }
    Constraint &operator=(Constraint &&other) noexcept {
        if (this != &other) {
            if (handle_ != nullptr) {
                lm_constr_free(handle_);
            }
            handle_ = other.handle_;
            other.handle_ = nullptr;
        }
        return *this;
    }
    Constraint(const Constraint &) = delete;
    Constraint &operator=(const Constraint &) = delete;

    /* Returns the constraint name. */
    std::string name() const {
        LmBuffer buf = {nullptr, 0, 0};
        detail::check(lm_constr_name(handle_, &buf));
        std::string out(reinterpret_cast<const char *>(buf.data), buf.len);
        lm_buffer_free(buf);
        return out;
    }

    /* Returns the constraint's lhs. */
    Expression lhs() const {
        LmExpr *out = nullptr;
        detail::check(lm_constr_lhs(handle_, &out));
        return Expression(out);
    }

    /* Returns the constraint's rhs. */
    double rhs() const {
        double out = 0;
        detail::check(lm_constr_rhs(handle_, &out));
        return out;
    }

    /* Returns the constraint's comparator. */
    LmComparator comparator() const {
        LmComparator out = {};
        detail::check(lm_constr_comparator(handle_, &out));
        return out;
    }

    bool auto_name() const {
        bool out = false;
        detail::check(lm_constr_auto_name(handle_, &out));
        return out;
    }

    bool evaluate_sample_quick(const std::vector<double> &lookup,
                               std::optional<double> tol = std::nullopt) const {
        bool out = false;
        detail::check(lm_constr_evaluate_sample_quick(handle_, lookup.data(), lookup.size(),
                                                      tol ? &*tol : nullptr, &out));
        return out;
    }

    /* Set the constraint's name. */
    void set_name(std::string name) { detail::check(lm_constr_set_name(handle_, name.c_str())); }

    /* Clones into a new handle. */
    Constraint clone() const {
        LmConstraint *out = nullptr;
        detail::check(lm_constr_clone(handle_, &out));
        return Constraint(out);
    }

    /* Deep-clones into a fully independent model with its own environment. */
    Constraint deep_clone(const Env &env) const {
        LmConstraint *out = nullptr;
        detail::check(lm_constr_deep_clone(handle_, env.get(), &out));
        return Constraint(out);
    }

    void substitute(const Vref &target, const Expression &replacement) {
        detail::check(lm_constr_substitute(handle_, target.get(), replacement.get()));
    }

    /* Compares two constraints for equal contents (ignoring different environments). */
    bool equal_contents(const Constraint &constr) const {
        bool out = false;
        detail::check(lm_constr_equal_contents(handle_, constr.get(), &out));
        return out;
    }

    bool operator==(const Constraint &other) const {
        bool out = false;
        detail::check(lm_constr_eq(handle_, other.handle_, &out));
        return out;
    }
    bool operator!=(const Constraint &other) const { return !(*this == other); }

    LmConstraint *get() const noexcept { return handle_; }
    LmConstraint *release() noexcept {
        LmConstraint *handle = handle_;
        handle_ = nullptr;
        return handle;
    }

  private:
    LmConstraint *handle_;
};

} // namespace lm
  //
#endif // LUNA_MODEL_CONSTRAINT_HPP
