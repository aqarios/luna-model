#ifndef LUNA_MODEL_ENV_MODIFICATION_HPP
#define LUNA_MODEL_ENV_MODIFICATION_HPP

#include <cstdint>
#include <string>
#include <vector>

#include <luna_model.h>

#include "luna_model/detail/capi.hpp"
#include "luna_model/env/env.hpp"

// Mutators.

namespace lm {

inline Vref Env::insert(const std::string &name, LmVtype vtype, const LazyBounds *bounds) {
    LmVref *out = nullptr;
    detail::check(
        lm_env_insert(handle_, name.c_str(), vtype, bounds ? bounds->get() : nullptr, &out));
    return Vref(out);
}

inline Vref Env::insert_with_fallback(const std::string &name, LmVtype vtype,
                                      const std::vector<std::uint64_t> &enc,
                                      const LazyBounds *bounds) {
    LmVref *out = nullptr;
    detail::check(lm_env_insert_with_fallback(handle_, name.c_str(), vtype,
                                              bounds ? bounds->get() : nullptr, enc.data(),
                                              enc.size(), &out));
    return Vref(out);
}

inline Vref Env::insert_inverted(const Vref &vref) {
    LmVref *out = nullptr;
    detail::check(lm_env_insert_inverted(handle_, vref.get(), &out));
    return Vref(out);
}

inline void Env::remove(const Vref &vref) { detail::check(lm_env_remove(handle_, vref.get())); }

inline void Env::set_inverted(std::uint32_t index, bool present, std::uint32_t inverted) {
    detail::check(lm_env_set_inverted(handle_, index, present, inverted));
}

} // namespace lm

#endif // LUNA_MODEL_ENV_MODIFICATION_HPP
