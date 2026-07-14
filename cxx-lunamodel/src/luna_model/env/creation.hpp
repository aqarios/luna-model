#ifndef LUNA_MODEL_ENV_CREATION_HPP
#define LUNA_MODEL_ENV_CREATION_HPP

#include <cstdint>
#include <string>
#include <utility>
#include <vector>

#include <luna_model.h>

#include "luna_model/detail/capi.hpp"
#include "luna_model/env/env.hpp"

// Construction, deep-copy, and (de)serialization, plus the one Env<->Vref bridge definition.

namespace lm {

inline Env Env::create(const std::vector<std::pair<std::uint32_t, const Variable *>> &variables,
                       const std::vector<std::pair<std::string, std::uint32_t>> &lookup,
                       std::uint32_t next_idx) {
    std::vector<const LmVar *> var_ptrs;
    std::vector<std::uint32_t> indices;
    var_ptrs.reserve(variables.size());
    indices.reserve(variables.size());
    for (const auto &[idx, var] : variables) {
        indices.push_back(idx);
        var_ptrs.push_back(var->get()); // borrowed; lm_env_new clones internally
    }

    std::vector<const char *> names;
    std::vector<std::uint32_t> lookup_idx;
    names.reserve(lookup.size());
    lookup_idx.reserve(lookup.size());
    for (const auto &[name, idx] : lookup) {
        names.push_back(name.c_str());
        lookup_idx.push_back(idx);
    }

    LmEnv *out = nullptr;
    detail::check(lm_env_new(var_ptrs.data(), indices.data(), var_ptrs.size(), names.data(),
                             lookup_idx.data(), names.size(), next_idx, &out));
    return Env(out);
}

inline Env Env::deep_clone() const {
    LmEnv *out = nullptr;
    detail::check(lm_env_deep_clone(handle_, &out));
    return Env(out);
}

inline std::vector<std::uint8_t> Env::serialize() const {
    LmBuffer buf = {nullptr, 0, 0};
    detail::check(lm_env_serialize(handle_, &buf));
    std::vector<std::uint8_t> bytes(buf.data, buf.data + buf.len);
    lm_buffer_free(buf);
    return bytes;
}

inline Env Env::deserialize(const std::uint8_t *data, std::size_t len) {
    LmEnv *out = nullptr;
    detail::check(lm_env_deserialize(data, len, &out));
    return Env(out);
}

// Env<->Vref bridge: declared in vref.hpp, defined here because it needs the
// complete `Env`. This is the single point where the one-directional
// `env -> vref` dependency is "closed"; keeping it in env/ (not vref.hpp) is
// what lets vref.hpp stay free of any env include.
inline Vref::Vref(std::uint32_t id, const Env &env) {
    LmVref *out = nullptr;
    detail::check(lm_vref_new(id, env.get(), &out));
    handle_ = out;
}

} // namespace lm

#endif // LUNA_MODEL_ENV_CREATION_HPP
