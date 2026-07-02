#ifndef LUNA_MODEL_LINEAR_HPP
#define LUNA_MODEL_LINEAR_HPP

#include "luna_model/types.h"
#include <luna_model.h>

namespace lm {

class Linear {
  public:
    /* Adopts ownership of a raw C handle (for C interop). */
    explicit Linear(LmLinear *handle) noexcept : handle_(handle) {}

    ~Linear() {
        if (handle_ != nullptr) {
            lm_linear_free(handle_);
        }
    }

  private:
    LmLinear *handle_;
};

} // namespace lm
  //
#endif // LUNA_MODEL_LINEAR_HPP
