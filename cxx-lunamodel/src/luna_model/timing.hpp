#ifndef LUNA_MODEL_TIMING_HPP
#define LUNA_MODEL_TIMING_HPP

#include "luna_model/funcs.h"
#include <luna_model.h>

namespace lm {

class Timer;

/* Owning handle to a LunaModel timing (frees on destruction). */
class Timing {
  public:
    ~Timing() {
        if (handle_ != nullptr) {
            lm_timing_free(handle_);
        }
    }

  protected:
    explicit Timing(LmTiming *handle) noexcept : handle_(handle) {}

  private:
    friend class Timer;

    LmTiming *handle_;
};

} // namespace lm

#endif // LUNA_MODEL_TIMING_HPP
