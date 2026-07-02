#ifndef LUNA_MODEL_LAZY_BOUNDS_HPP
#define LUNA_MODEL_LAZY_BOUNDS_HPP

#include "luna_model/types.h"
#include <luna_model.h>

namespace lm {

class LazyBounds {
  public:
    /* Adopts ownership of a raw C handle (for C interop). */
    explicit LazyBounds(LmLazyBounds *handle) noexcept : handle_(handle) {}

    ~LazyBounds() {
        if (handle_ != nullptr) {
            lm_lazy_bounds_free(handle_);
        }
    }

  private:
    LmLazyBounds *handle_;
};

} // namespace lm
  //
#endif // LUNA_MODEL_LAZY_BOUNDS_HPP
