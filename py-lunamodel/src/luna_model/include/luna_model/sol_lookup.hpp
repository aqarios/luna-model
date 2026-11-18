#ifndef LUNA_MODEL_SOL_LOOKUP_HPP
#define LUNA_MODEL_SOL_LOOKUP_HPP

#include <cstddef>
#include <cstdint>
#include <vector>

#include <luna_model.h>

#include "luna_model/detail/capi.hpp"
#include "luna_model/env.hpp"
#include "luna_model/sample_view.hpp"
#include "luna_model/solution.hpp"

namespace lm {

/* Owning handle to a bulk per-sample solution lookup table (frees on destruction).
 *
 * Mirrors the native `luna-model` `ops::SolutionLookup`: build once from an
 * `Env` and a `Solution`, then reuse across many samples via `update()` and
 * `write_sol()` — avoiding one ABI round-trip per value in hot per-sample
 * loops such as samplers. */
class SolutionLookup {
  public:
    /* Builds a lookup table over `env`'s variables for `sol`. */
    SolutionLookup(const Env &env, const Solution &sol) {
        detail::check(lm_sol_lookup_new(env.get(), sol.get(), &handle_));
    }

    /* Adopts ownership of a raw C handle (for C interop). */
    explicit SolutionLookup(LmSolutionLookup *handle) noexcept : handle_(handle) {}

    ~SolutionLookup() {
        if (handle_ != nullptr) {
            lm_sol_lookup_free(handle_);
        }
    }

    SolutionLookup(SolutionLookup &&other) noexcept : handle_(other.handle_) {
        other.handle_ = nullptr;
    }
    SolutionLookup &operator=(SolutionLookup &&other) noexcept {
        if (this != &other) {
            if (handle_ != nullptr) {
                lm_sol_lookup_free(handle_);
            }
            handle_ = other.handle_;
            other.handle_ = nullptr;
        }
        return *this;
    }
    SolutionLookup(const SolutionLookup &) = delete;
    SolutionLookup &operator=(const SolutionLookup &) = delete;

    /* Loads one sample row's values into the lookup table, in place. */
    void update(const SampleView &sample) { detail::check(lm_sol_lookup_update(handle_, sample.get())); }

    /* The current lookup values, indexed by variable id. */
    std::vector<double> values() const {
        const double *data = nullptr;
        std::size_t len = 0;
        detail::check(lm_sol_lookup_values(handle_, &data, &len));
        return std::vector<double>(data, data + len);
    }

    /* Overwrites row `idx` of `sol` with `values` (indexed by variable id).
     *
     * `values` is typically a caller-owned copy of a prior `values()` call,
     * mutated locally (e.g. by a sampler's hot per-flip loop) without any
     * further ABI round-trips, then written back once per sample here. */
    void write_sol(std::uint32_t idx, const std::vector<double> &values, Solution &sol) {
        detail::check(
            lm_sol_lookup_write_sol(handle_, idx, values.data(), values.size(), sol.get()));
    }

    LmSolutionLookup *get() const noexcept { return handle_; }
    LmSolutionLookup *release() noexcept {
        LmSolutionLookup *h = handle_;
        handle_ = nullptr;
        return h;
    }

  private:
    LmSolutionLookup *handle_ = nullptr;
};

} // namespace lm

#endif // LUNA_MODEL_SOL_LOOKUP_HPP
