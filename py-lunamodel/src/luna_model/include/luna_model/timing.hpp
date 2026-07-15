#ifndef LUNA_MODEL_TIMING_HPP
#define LUNA_MODEL_TIMING_HPP

#include <cstdint>
#include <optional>

#include <luna_model.h>

#include "luna_model/detail/capi.hpp"

namespace lm {

class Timer;
class Solution;

/* Owning handle to a LunaModel timing (frees on destruction). */
class Timing {
  public:
    /* Creates a timing record from explicit timestamps
     * (Rust `Timing::new(start, end, qpu)`).
     *
     * `start_unix_ns` / `end_unix_ns` are nanoseconds since the UNIX epoch;
     * `qpu` is the optional QPU/device execution time.
     */
    Timing(std::uint64_t start_unix_ns, std::uint64_t end_unix_ns,
           std::optional<double> qpu = std::nullopt) {
        LmTiming *out = nullptr;
        detail::check(lm_timing_new(start_unix_ns, end_unix_ns, qpu ? &*qpu : nullptr, &out));
        handle_ = out;
    }

    ~Timing() {
        if (handle_ != nullptr) {
            lm_timing_free(handle_);
        }
    }

    Timing(Timing &&other) noexcept : handle_(other.handle_) { other.handle_ = nullptr; }
    Timing &operator=(Timing &&other) noexcept {
        if (this != &other) {
            if (handle_ != nullptr) {
                lm_timing_free(handle_);
            }
            handle_ = other.handle_;
            other.handle_ = nullptr;
        }
        return *this;
    }
    Timing(const Timing &) = delete;
    Timing &operator=(const Timing &) = delete;

    std::uint64_t total_ns() const {
        std::uint64_t out = 0;
        detail::check(lm_timing_total_ns(handle_, &out));
        return out;
    }

    std::uint64_t start_ns() const {
        std::uint64_t out = 0;
        detail::check(lm_timing_start_ns(handle_, &out));
        return out;
    }

    std::uint64_t end_ns() const {
        std::uint64_t out = 0;
        detail::check(lm_timing_end_ns(handle_, &out));
        return out;
    }

    std::optional<double> qpu() const {
        double out = 0.0;
        const LmStatus status = lm_timing_qpu(handle_, &out);
        if (status == LmStatus_Absent) {
            return std::nullopt;
        }
        detail::check(status);
        return out;
    }

    Timing clone() const {
        LmTiming *out = nullptr;
        detail::check(lm_timing_clone(handle_, &out));
        return Timing(out);
    }

    bool operator==(const Timing &other) const {
        bool out = false;
        detail::check(lm_timing_eq(handle_, other.handle_, &out));
        return out;
    }
    bool operator!=(const Timing &other) const { return !(*this == other); }

    LmTiming *get() const noexcept { return handle_; }
    LmTiming *release() noexcept {
        LmTiming *handle = handle_;
        handle_ = nullptr;
        return handle;
    }

  protected:
    explicit Timing(LmTiming *handle) noexcept : handle_(handle) {}

  private:
    friend class Timer;
    friend class Solution;

    LmTiming *handle_;
};

} // namespace lm

#endif // LUNA_MODEL_TIMING_HPP
