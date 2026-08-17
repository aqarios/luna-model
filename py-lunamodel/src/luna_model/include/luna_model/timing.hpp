#ifndef LUNA_MODEL_TIMING_HPP
#define LUNA_MODEL_TIMING_HPP

#include <cstdint>
#include <optional>
#include <vector>

#include <luna_model.h>

#include "luna_model/detail/capi.hpp"

namespace lm {

class Timer;
class Solution;

/* Owning handle to a LunaModel timing record (frees on destruction). */
class Timing {
  public:
    /* Creates a timing record from explicit timestamps.
     *
     * `start_unix_ns` / `end_unix_ns` are nanoseconds since the UNIX epoch;
     * `qpu` is the optional QPU/device execution time in seconds.
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

    /* Total wall-clock duration in nanoseconds. */
    std::uint64_t total_ns() const {
        std::uint64_t out = 0;
        detail::check(lm_timing_total_ns(handle_, &out));
        return out;
    }

    /* Start timestamp, nanoseconds since the UNIX epoch. */
    std::uint64_t start_ns() const {
        std::uint64_t out = 0;
        detail::check(lm_timing_start_ns(handle_, &out));
        return out;
    }

    /* End timestamp, nanoseconds since the UNIX epoch. */
    std::uint64_t end_ns() const {
        std::uint64_t out = 0;
        detail::check(lm_timing_end_ns(handle_, &out));
        return out;
    }

    /* QPU/device execution time in seconds. */
    double qpu() const {
        double out = 0.0;
        detail::check(lm_timing_qpu(handle_, &out));
        return out;
    }

    /* Serializes this timing record to bytes. */
    std::vector<std::uint8_t> serialize() const {
        LmBuffer buf = {nullptr, 0, 0};
        detail::check(lm_timing_serialize(handle_, &buf));
        std::vector<std::uint8_t> bytes(buf.data, buf.data + buf.len);
        lm_buffer_free(buf);
        return bytes;
    }

    /* Deserializes a timing record from bytes. */
    static Timing deserialize(const std::uint8_t *data, std::size_t len) {
        LmTiming *out = nullptr;
        detail::check(lm_timing_deserialize(data, len, &out));
        return Timing(out);
    }
    static Timing deserialize(const std::vector<std::uint8_t> &bytes) {
        return deserialize(bytes.data(), bytes.size());
    }

    LmTiming *get() const noexcept { return handle_; }

  protected:
    explicit Timing(LmTiming *handle) noexcept : handle_(handle) {}

  private:
    friend class Timer;
    friend class Solution;

    LmTiming *handle_;
};

} // namespace lm

#endif // LUNA_MODEL_TIMING_HPP
