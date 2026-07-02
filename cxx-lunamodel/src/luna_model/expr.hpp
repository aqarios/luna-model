#ifndef LUNA_MODEL_EXPRESSION_HPP
#define LUNA_MODEL_EXPRESSION_HPP

#include "luna_model/types.h"
#include <luna_model.h>

namespace lm {

class Expression {
  public:
    /* Adopts ownership of a raw C handle (for C interop). */
    explicit Expression(LmExpr *handle) noexcept : handle_(handle) {}

    ~Expression() {
        if (handle_ != nullptr) {
            lm_expr_free(handle_);
        }
    }

  private:
    LmExpr *handle_;
};

} // namespace lm
  //
#endif // LUNA_MODEL_EXPRESSION_HPP
