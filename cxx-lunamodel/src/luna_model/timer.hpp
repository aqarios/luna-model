#ifndef LUNA_MODEL_TIMER_HPP
#define LUNA_MODEL_TIMER_HPP

#include <string>

#include <luna_model.h>

#include "luna_model/detail/capi.hpp"
#include "luna_model/subtimer.hpp"
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

    Timer(Timer &&other) noexcept : handle_(other.handle_) { other.handle_ = nullptr; }
    Timer &operator=(Timer &&other) noexcept {
        if (this != &other) {
            if (handle_ != nullptr) {
                lm_timer_free(handle_);
            }
            handle_ = other.handle_;
            other.handle_ = nullptr;
        }
        return *this;
    }
    Timer(const Timer &) = delete;
    Timer &operator=(const Timer &) = delete;

    /* Starts a new wall-clock timer. */
    static Timer start() { return Timer(lm_timer_start()); }

    /* Starts a fine-grained sub-timer under `key`, to be stopped via `SubTimer::stop`. */
    SubTimer record(const std::string &key) const {
        LmSubTimer *out = nullptr;
        detail::check(lm_timer_record(handle_, key.c_str(), &out));
        return SubTimer(out);
    }

    /* Stops the timer, producing a timing record. The timer remains valid and
     * reusable after this call. */
    Timing stop() const {
        LmTiming *out = nullptr;
        detail::check(lm_timer_stop(handle_, &out));
        return Timing(out);
    }

  private:
    explicit Timer(LmTimer *handle) noexcept : handle_(handle) {}
    LmTimer *handle_;
};

} // namespace lm

#endif // LUNA_MODEL_TIMER_HPP
