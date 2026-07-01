#ifndef LUNA_MODEL_TIMER_HPP
#define LUNA_MODEL_TIMER_HPP

#include <luna_model.h>

#include "luna_model/timing.hpp"

namespace lm {

/* Owning handle to a LunaModel timer (frees on destruction). */
class Timer {
  public:
    ~Timer() {
        if (handle_ != nullptr) {
            lm_timer_free(handle_);
        }
    }

    static Timer start() {
        LmTimer *out = nullptr;
        return Timer(out);
    }

    Timing stop() const {
        LmTiming *out = nullptr;
        lm_timer_stop(handle_, &out);
        return Timing(out);
    }

  private:
    explicit Timer(LmTimer *handle) noexcept : handle_(handle) {}
    LmTimer *handle_;
};

} // namespace lm

#endif // LUNA_MODEL_TIMER_HPP
