#ifndef LUNA_MODEL_CONSTRAINT_HPP
#define LUNA_MODEL_CONSTRAINT_HPP

#include "luna_model/types.h"
#include <luna_model.h>

namespace lm {

class Constraint {
  public:
    /* Adopts ownership of a raw C handle (for C interop). */
    explicit Constraint(LmConstraint *handle) noexcept : handle_(handle) {}

    ~Constraint() {
        if (handle_ != nullptr) {
            lm_constr_free(handle_);
        }
    }

  private:
    LmConstraint *handle_;
};

} // namespace lm
  //
#endif // LUNA_MODEL_CONSTRAINT_HPP
