#ifndef LUNA_MODEL_TIMER_HPP
#define LUNA_MODEL_TIMER_HPP

#include <luna_model.h>

#include "luna_model/detail/capi.hpp"
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

    /* Starts a new wall-clock timer (Rust `Timer::start()`). */
    static Timer start() { return Timer(lm_timer_start()); }

    /* Stops the timer, producing a timing record (Rust `timer.stop()`). */
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
