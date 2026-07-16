#ifndef LUNA_MODEL_CONSTRAINT_HPP
#define LUNA_MODEL_CONSTRAINT_HPP

#include <optional>
#include <string>
#include <vector>

#include <luna_model.h>

#include "luna_model/detail/capi.hpp"
#include "luna_model/env.hpp"
#include "luna_model/expr.hpp"
#include "luna_model/vref.hpp"

namespace lm {

/* Owning handle to a constraint: a comparison between an expression and a
 * right-hand-side value (frees on destruction). */
class Constraint {
  public:
    /* Creates a constraint `lhs <cmp> rhs`. `name == std::nullopt` assigns an
     * automatic name. */
    Constraint(const Expression &lhs, double rhs, LmComparator cmp,
               std::optional<std::string> name = std::nullopt) {
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

    /* The constraint name. */
    std::string name() const {
        LmBuffer buf = {nullptr, 0, 0};
        detail::check(lm_constr_name(handle_, &buf));
        std::string out(reinterpret_cast<const char *>(buf.data), buf.len);
        lm_buffer_free(buf);
        return out;
    }

    /* Whether the constraint's name was assigned automatically. */
    bool auto_name() const {
        bool out = false;
        detail::check(lm_constr_auto_name(handle_, &out));
        return out;
    }

    /* The left-hand-side expression. */
    Expression lhs() const {
        LmExpr *out = nullptr;
        detail::check(lm_constr_lhs(handle_, &out));
        return Expression(out);
    }

    /* The right-hand-side value. */
    double rhs() const {
        double out = 0;
        detail::check(lm_constr_rhs(handle_, &out));
        return out;
    }

    /* The comparator relating the two sides. */
    LmComparator comparator() const {
        LmComparator out = {};
        detail::check(lm_constr_comparator(handle_, &out));
        return out;
    }

    /* Evaluates the constraint against an index-aligned dense lookup. */
    bool evaluate_sample_quick(const std::vector<double> &lookup,
                               std::optional<double> tol = std::nullopt) const {
        bool out = false;
        detail::check(lm_constr_evaluate_sample_quick(handle_, lookup.data(), lookup.size(),
                                                      tol ? &*tol : nullptr, &out));
        return out;
    }

    /* Sets the constraint name. */
    void set_name(const std::string &name) {
        detail::check(lm_constr_set_name(handle_, name.c_str()));
    }

    /* Clones into a new handle sharing the environment. */
    Constraint clone() const {
        LmConstraint *out = nullptr;
        detail::check(lm_constr_clone(handle_, &out));
        return Constraint(out);
    }

    /* Deep-clones into an independent constraint rebound to `env`. */
    Constraint deep_clone(const Env &env) const {
        LmConstraint *out = nullptr;
        detail::check(lm_constr_deep_clone(handle_, env.get(), &out));
        return Constraint(out);
    }

    /* Substitutes `replacement` for `target` throughout the constraint. */
    void substitute(const Vref &target, const Expression &replacement) {
        detail::check(lm_constr_substitute(handle_, target.get(), replacement.get()));
    }

    /* Content equality, ignoring environment identity. */
    bool equal_contents(const Constraint &other) const {
        bool out = false;
        detail::check(lm_constr_equal_contents(handle_, other.get(), &out));
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
        LmConstraint *h = handle_;
        handle_ = nullptr;
        return h;
    }

  private:
    LmConstraint *handle_;
};

} // namespace lm

#endif // LUNA_MODEL_CONSTRAINT_HPP
