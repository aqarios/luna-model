#ifndef LUNA_MODEL_SPECS_HPP
#define LUNA_MODEL_SPECS_HPP

#include "luna_model/types.h"
#include <luna_model.h>

namespace lm {

class Specs {
  public:
    /* Adopts ownership of a raw C handle (for C interop). */
    explicit Specs(LmSpecs *handle) noexcept : handle_(handle) {}

    ~Specs() {
        if (handle_ != nullptr) {
            lm_specs_free(handle_);
        }
    }

  private:
    LmSpecs *handle_;
};

} // namespace lm
  //
#endif // LUNA_MODEL_SPECS_HPP
