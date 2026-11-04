#ifndef LUNA_MODEL_ENV_HPP
#define LUNA_MODEL_ENV_HPP

#include <cstdint>
#include <string>
#include <utility>
#include <vector>

#include <luna_model.h>

#include "luna_model/detail/capi.hpp"
#include "luna_model/error.hpp"
#include "luna_model/lazy_bounds.hpp"
#include "luna_model/variable.hpp"
#include "luna_model/vref.hpp"

namespace lm {

/* Owning handle to a variable environment (frees on destruction). An
 * environment stores the variables of a problem and maps names to indices.
 *
 * The include graph is one-directional (`env -> vref -> variable -> bounds`).
 * `vref.hpp` only forward-declares `Env`; the `Vref(id, env)` constructor is
 * defined at the bottom of this file, where both types are complete. */
class Env {
  public:
    // --- lifecycle ---

    /* Creates a new, empty environment. */
    Env() : handle_(lm_env_default()) {
        if (handle_ == nullptr) {
            throw Error(LmStatus_Panic, "failed to allocate a LunaModel environment");
        }
    }

    /* Adopts ownership of a raw C handle (for C interop). */
    explicit Env(LmEnv *handle) noexcept : handle_(handle) {}

    ~Env() {
        if (handle_ != nullptr) {
            lm_env_free(handle_);
        }
    }

    Env(Env &&other) noexcept : handle_(other.handle_) { other.handle_ = nullptr; }
    Env &operator=(Env &&other) noexcept {
        if (this != &other) {
            if (handle_ != nullptr) {
                lm_env_free(handle_);
            }
            handle_ = other.handle_;
            other.handle_ = nullptr;
        }
        return *this;
    }
    Env(const Env &) = delete;
    Env &operator=(const Env &) = delete;

    /* Clones into a new handle sharing the same underlying environment — a
     * cheap reference-count bump. */
    Env clone() const {
        LmEnv *out = nullptr;
        detail::check(lm_env_clone(handle_, &out));
        return Env(out);
    }

    /* Builds an environment from existing variables and a name->index lookup.
     * Variable handles are borrowed (cloned internally), not consumed;
     * `next_idx` is the next free variable id. */
    static Env create(const std::vector<std::pair<std::uint32_t, const Variable *>> &variables,
                      const std::vector<std::pair<std::string, std::uint32_t>> &lookup,
                      std::uint32_t next_idx) {
        std::vector<const LmVar *> var_ptrs;
        std::vector<std::uint32_t> indices;
        var_ptrs.reserve(variables.size());
        indices.reserve(variables.size());
        for (const auto &[idx, var] : variables) {
            indices.push_back(idx);
            var_ptrs.push_back(var->get());
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

    // --- const accessors ---

    /* Number of variables in the environment. */
    std::size_t len() const {
        std::uintptr_t out = 0;
        detail::check(lm_env_len(handle_, &out));
        return out;
    }

    /* Whether the environment has no variables. */
    bool is_empty() const {
        bool out = false;
        detail::check(lm_env_is_empty(handle_, &out));
        return out;
    }

    /* The next free variable index. */
    std::uint32_t next_idx() const {
        std::uint32_t out = 0;
        detail::check(lm_env_next_idx(handle_, &out));
        return out;
    }

    /* Stable 128-bit environment identity, as { hi, lo }. */
    LmEnvId id() const {
        LmEnvId out{};
        detail::check(lm_env_id(handle_, &out));
        return out;
    }

    /* Whether a variable named `name` exists. */
    bool contains(const std::string &name) const {
        bool out = false;
        detail::check(lm_env_contains(handle_, name.c_str(), &out));
        return out;
    }

    /* The type of the variable named `name`. */
    LmVtype vtype_of(const std::string &name) const {
        LmVtype out{};
        detail::check(lm_env_vtype_of(handle_, name.c_str(), &out));
        return out;
    }

    /* Structural content equality. */
    bool equal_contents(const Env &other) const {
        bool out = false;
        detail::check(lm_env_equal_contents(handle_, other.handle_, &out));
        return out;
    }

    /* Reorders `names` into the environment's canonical order. */
    std::vector<std::string> sort(const std::vector<std::string> &names) const {
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

    /* All variable references. */
    std::vector<Vref> vars() const {
        LmVrefBuffer buf = {nullptr, 0, 0};
        detail::check(lm_env_vars(handle_, &buf));
        std::vector<Vref> out;
        out.reserve(buf.len);
        for (std::size_t i = 0; i < buf.len; ++i) {
            out.emplace_back(buf.data[i]); // Vref adopts the handle,
            buf.data[i] = nullptr;         // so the buffer must not free it.
        }
        lm_vref_buffer_free(buf);
        return out;
    }

    /* A reference to the variable named `name`. */
    Vref lookup(const std::string &name) const {
        LmVref *out = nullptr;
        detail::check(lm_env_lookup(handle_, name.c_str(), &out));
        return Vref(out);
    }

    /* A reference to the variable with index `vid`. Note: the zero-argument
     * `get()` below returns the raw C handle instead. */
    Vref get(std::uint32_t vid) const {
        LmVref *out = nullptr;
        detail::check(lm_env_get(handle_, vid, &out));
        return Vref(out);
    }

    /* The variable with index `vid`, materialized as an owned `Variable`. */
    Variable get_variable(std::uint32_t vid) const { return get(vid).to_variable(); }

    // --- mutation ---

    /* Inserts a variable and returns a reference to it. `bounds == nullptr`
     * uses the default bounds for `vtype`. */
    Vref insert(const std::string &name, LmVtype vtype, const LazyBounds *bounds = nullptr) {
        LmVref *out = nullptr;
        detail::check(
            lm_env_insert(handle_, name.c_str(), vtype, bounds ? bounds->get() : nullptr, &out));
        return Vref(out);
    }

    /* Inserts a variable, retrying with an encoding fallback on name clashes. */
    Vref insert_with_fallback(const std::string &name, LmVtype vtype,
                              const std::vector<std::uint64_t> &enc,
                              const LazyBounds *bounds = nullptr) {
        LmVref *out = nullptr;
        detail::check(lm_env_insert_with_fallback(handle_, name.c_str(), vtype,
                                                  bounds ? bounds->get() : nullptr, enc.data(),
                                                  enc.size(), &out));
        return Vref(out);
    }

    /* Inserts the inverted-binary companion of `vref`. */
    Vref insert_inverted(const Vref &vref) {
        LmVref *out = nullptr;
        detail::check(lm_env_insert_inverted(handle_, vref.get(), &out));
        return Vref(out);
    }

    /* Removes the variable referenced by `vref`; throws `lm::Error` if it is not in the environment. */
    void remove(const Vref &vref) { detail::check(lm_env_remove(handle_, vref.get())); }

    /* Sets or clears the inverted companion index of the variable at `index`.
     * Pass `present == false` to clear. */
    void set_inverted(std::uint32_t index, bool present, std::uint32_t inverted) {
        detail::check(lm_env_set_inverted(handle_, index, present, inverted));
    }

    // --- copying / serialization ---

    /* Deep structural copy — a genuinely independent environment, unlike the
     * refcount-bump `clone()`. */
    Env deep_clone() const {
        LmEnv *out = nullptr;
        detail::check(lm_env_deep_clone(handle_, &out));
        return Env(out);
    }

    /* Serializes the environment to bytes. */
    std::vector<std::uint8_t> serialize() const {
        LmBuffer buf = {nullptr, 0, 0};
        detail::check(lm_env_serialize(handle_, &buf));
        std::vector<std::uint8_t> bytes(buf.data, buf.data + buf.len);
        lm_buffer_free(buf);
        return bytes;
    }

    /* Deserializes an environment previously produced by `serialize`. */
    static Env deserialize(const std::uint8_t *data, std::size_t len) {
        LmEnv *out = nullptr;
        detail::check(lm_env_deserialize(data, len, &out));
        return Env(out);
    }
    static Env deserialize(const std::vector<std::uint8_t> &bytes) {
        return deserialize(bytes.data(), bytes.size());
    }

    // --- raw C interop ---

    LmEnv *get() const noexcept { return handle_; }
    LmEnv *release() noexcept {
        LmEnv *h = handle_;
        handle_ = nullptr;
        return h;
    }

  private:
    LmEnv *handle_;
};

// Env<->Vref bridge: declared in vref.hpp, defined here where `Env` is complete.
inline Vref::Vref(std::uint32_t id, const Env &env) {
    LmVref *out = nullptr;
    detail::check(lm_vref_new(id, env.get(), &out));
    handle_ = out;
}

} // namespace lm

#endif // LUNA_MODEL_ENV_HPP
