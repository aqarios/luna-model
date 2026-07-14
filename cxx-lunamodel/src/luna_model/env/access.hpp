#ifndef LUNA_MODEL_ENV_ACCESS_HPP
#define LUNA_MODEL_ENV_ACCESS_HPP

#include <string>
#include <vector>

#include <luna_model.h>

#include "luna_model/detail/capi.hpp"
#include "luna_model/env/env.hpp"

// Const read accessors.

namespace lm {

inline std::size_t Env::len() const {
    std::uintptr_t out = 0;
    detail::check(lm_env_len(handle_, &out));
    return out;
}

inline bool Env::is_empty() const {
    bool out = false;
    detail::check(lm_env_is_empty(handle_, &out));
    return out;
}

inline std::uint32_t Env::next_idx() const {
    std::uint32_t out = 0;
    detail::check(lm_env_next_idx(handle_, &out));
    return out;
}

inline LmEnvId Env::id() const {
    LmEnvId out{};
    detail::check(lm_env_id(handle_, &out));
    return out;
}

inline bool Env::contains(const std::string &name) const {
    bool out = false;
    detail::check(lm_env_contains(handle_, name.c_str(), &out));
    return out;
}

inline LmVtype Env::vtype_of(const std::string &name) const {
    LmVtype out{};
    detail::check(lm_env_vtype_of(handle_, name.c_str(), &out));
    return out;
}

inline bool Env::equal_contents(const Env &other) const {
    bool out = false;
    detail::check(lm_env_equal_contents(handle_, other.handle_, &out));
    return out;
}

inline std::vector<std::string> Env::sort(const std::vector<std::string> &names) const {
    std::vector<const char *> ptrs;
    ptrs.reserve(names.size());
    for (const auto &n : names) {
        ptrs.push_back(n.c_str());
    }

    LmStrBuffer buf = {nullptr, 0, 0};
    detail::check(lm_env_sort(handle_, ptrs.data(), ptrs.size(), &buf));
    std::vector<std::string> out;
    out.reserve(buf.len);
    for (std::size_t i = 0; i < buf.len; ++i) {
        out.emplace_back(buf.data[i]);
    }
    lm_str_buffer_free(buf);
    return out;
}

inline std::vector<Vref> Env::vars() const {
    LmVrefBuffer buf = {nullptr, 0, 0};
    detail::check(lm_env_vars(handle_, &buf));
    std::vector<Vref> out;
    out.reserve(buf.len);
    for (std::size_t i = 0; i < buf.len; ++i) {
        out.emplace_back(buf.data[i]); // Vref adopts the handle,
        buf.data[i] = nullptr;         // so the buffer must not free it.
    }
    lm_vref_buffer_free(buf); // frees only the outer array (skips nulls)
    return out;
}

inline Vref Env::lookup(const std::string &name) const {
    LmVref *out = nullptr;
    detail::check(lm_env_lookup(handle_, name.c_str(), &out));
    return Vref(out);
}

inline Vref Env::get(std::uint32_t vid) const {
    LmVref *out = nullptr;
    detail::check(lm_env_get(handle_, vid, &out));
    return Vref(out);
}

} // namespace lm

#endif // LUNA_MODEL_ENV_ACCESS_HPP
