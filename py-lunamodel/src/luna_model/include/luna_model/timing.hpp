#ifndef LUNA_MODEL_TIMING_HPP
#define LUNA_MODEL_TIMING_HPP

#include <cstdint>
#include <optional>
#include <string>
#include <vector>

#include <luna_model.h>

#include "luna_model/detail/capi.hpp"
#include "luna_model/funcs.h"

namespace lm {

class Timer;
class Solution;

/* Owning handle to a LunaModel timing record (frees on destruction). */
class Timing {
  public:
    /* Creates a timing record with the given total duration in seconds and no
     * recorded start/end time or sub-timings. */
    explicit Timing(double total = 0.0) {
        LmTiming *out = nullptr;
        detail::check(lm_timing_new(total, &out));
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

    /* Total wall-clock duration in seconds. */
    double total() const {
        double out = 0.0;
        detail::check(lm_timing_total_seconds(handle_, &out));
        return out;
    }

    /* Start timestamp, nanoseconds since the UNIX epoch, or `std::nullopt` if unrecorded. */
    std::optional<std::uint64_t> start_ns() const {
        std::uint64_t out = 0;
        LmStatus status = lm_timing_start_ns(handle_, &out);
        if (status == LmStatus_Absent) {
            return std::nullopt;
        }
        detail::check(status);
        return out;
    }

    /* End timestamp, nanoseconds since the UNIX epoch, or `std::nullopt` if unrecorded. */
    std::optional<std::uint64_t> end_ns() const {
        std::uint64_t out = 0;
        LmStatus status = lm_timing_end_ns(handle_, &out);
        if (status == LmStatus_Absent) {
            return std::nullopt;
        }
        detail::check(status);
        return out;
    }

    /* The values recorded under `key`, or an empty vector if absent. */
    std::vector<double> get(const std::string &key) const {
        LmF64Buffer buf = {nullptr, 0, 0};
        detail::check(lm_timing_get(handle_, key.c_str(), &buf));
        std::vector<double> values(buf.data, buf.data + buf.len);
        lm_f64_buffer_free(buf);
        return values;
    }

    /* The sum of the values recorded under `key`, or `std::nullopt` if absent. */
    std::optional<double> total_for(const std::string &key) const {
        double out = 0.0;
        LmStatus status = lm_timing_total_for(handle_, key.c_str(), &out);
        if (status == LmStatus_Absent) {
            return std::nullopt;
        }
        detail::check(status);
        return out;
    }

    /* Overwrites the values recorded under `key` with a single `value`. */
    void set(const std::string &key, double value) {
        detail::check(lm_timing_set(handle_, key.c_str(), value));
    }

    /* Appends `value` to the list recorded under `key`. */
    void add(const std::string &key, double value) {
        detail::check(lm_timing_add(handle_, key.c_str(), value));
    }

    /* Merges `other` into this timing. */
    void merge(Timing other) { detail::check(lm_timing_merge(handle_, other.handle_)); }

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
