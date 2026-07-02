#ifndef LUNA_MODEL_QUADRATIC_HPP
#define LUNA_MODEL_QUADRATIC_HPP

#include "luna_model/types.h"
#include <luna_model.h>

namespace lm {

class Quadratic {
  public:
    /* Adopts ownership of a raw C handle (for C interop). */
    explicit Quadratic(LmQuadratic *handle) noexcept : handle_(handle) {}

    ~Quadratic() {
        if (handle_ != nullptr) {
            lm_quadratic_free(handle_);
        }
    }

  private:
    LmQuadratic *handle_;
};

} // namespace lm
  //
#endif // LUNA_MODEL_QUADRATIC_HPP

