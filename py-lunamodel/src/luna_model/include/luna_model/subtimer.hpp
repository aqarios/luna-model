#ifndef LUNA_MODEL_SUBTIMER_HPP
#define LUNA_MODEL_SUBTIMER_HPP

#include <luna_model.h>

#include "luna_model/detail/capi.hpp"

namespace lm {

class Timer;

/* Owning handle to a LunaModel fine-grained sub-timer.
 *
 * Created by `Timer::record`. Destroying it without calling `stop()` discards
 * the measurement unrecorded.
 */
class SubTimer {
  public:
    ~SubTimer() {
        if (handle_ != nullptr) {
            lm_subtimer_free(handle_);
        }
    }

    SubTimer(SubTimer &&other) noexcept : handle_(other.handle_) { other.handle_ = nullptr; }
    SubTimer &operator=(SubTimer &&other) noexcept {
        if (this != &other) {
            if (handle_ != nullptr) {
                lm_subtimer_free(handle_);
            }
            handle_ = other.handle_;
            other.handle_ = nullptr;
        }
        return *this;
    }
    SubTimer(const SubTimer &) = delete;
    SubTimer &operator=(const SubTimer &) = delete;

    /* Stops the sub-timer, recording its elapsed time (in seconds) on the
     * parent timer under the key it was started with, and returns that
     * elapsed time. Consumes this sub-timer. */
    double stop() {
        double out = 0.0;
        detail::check(lm_subtimer_stop(handle_, &out));
        handle_ = nullptr; // the ABI call already freed the handle
        return out;
    }

  private:
    friend class Timer;
    explicit SubTimer(LmSubTimer *handle) noexcept : handle_(handle) {}
    LmSubTimer *handle_;
};

} // namespace lm

#endif // LUNA_MODEL_SUBTIMER_HPP
