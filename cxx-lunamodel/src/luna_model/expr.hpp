#ifndef LUNA_MODEL_EXPRESSION_HPP
#define LUNA_MODEL_EXPRESSION_HPP

#include <luna_model.h>

namespace lm {

class Vref; // defined in vref.hpp; ops are defined in ops.hpp

/* Owning handle to a LunaModel expression (frees on destruction).
 *
 * Mirrors the Rust `Expression`.
 */
class Expression {
  public:
    /* Adopts ownership of a raw C handle (for C interop). */
    explicit Expression(LmExpr *handle) noexcept : handle_(handle) {}

    ~Expression() {
        if (handle_ != nullptr) {
            lm_expr_free(handle_);
        }
    }

    Expression(Expression &&other) noexcept : handle_(other.handle_) {
        other.handle_ = nullptr;
    }
    Expression &operator=(Expression &&other) noexcept {
        if (this != &other) {
            if (handle_ != nullptr) {
                lm_expr_free(handle_);
            }
            handle_ = other.handle_;
            other.handle_ = nullptr;
        }
        return *this;
    }
    Expression(const Expression &) = delete;
    Expression &operator=(const Expression &) = delete;

    /* expr += vref, in place (Rust `expr += &vref`). Defined in ops.hpp. */
    Expression &operator+=(const Vref &rhs);

    LmExpr *get() const noexcept { return handle_; }
    LmExpr *release() noexcept {
        LmExpr *h = handle_;
        handle_ = nullptr;
        return h;
    }

  private:
    LmExpr *handle_;
};

} // namespace lm

#endif // LUNA_MODEL_EXPRESSION_HPP
