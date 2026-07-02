#ifndef LUNA_MODEL_HIGHER_ORDER_HPP
#define LUNA_MODEL_HIGHER_ORDER_HPP

#include "luna_model/types.h"
#include <luna_model.h>

namespace lm {

class HigherOrder {
  public:
    /* Adopts ownership of a raw C handle (for C interop). */
    explicit HigherOrder(LmHigherOrder *handle) noexcept : handle_(handle) {}

    ~HigherOrder() {
        if (handle_ != nullptr) {
            lm_higher_order_free(handle_);
        }
    }

  private:
    LmHigherOrder *handle_;
};

} // namespace lm
  //
#endif // LUNA_MODEL_HIGHER_ORDER_HPP

