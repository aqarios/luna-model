#ifndef LUNA_MODEL_ENV_HPP
#define LUNA_MODEL_ENV_HPP

#include <cstddef>
#include <cstdint>
#include <string>
#include <vector>

#include <luna_model.h>

#include "luna_model/detail/capi.hpp"
#include "luna_model/error.hpp"

namespace lm {

class Vref; // defined in vref.hpp, where Env::insert is also defined
class LazyBounds;

/* Owning handle to a LunaModel env (frees on destruction).
 *
 * Mirrors the Rust `ArcEnv`.
 */
class Env {
  public:
    /* Creates a new, empty environment (Rust `ArcEnv::default()`). */
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

    /* Clones into a new handle sharing the same underlying environment
     * (Rust `Clone` on `ArcEnv` — a cheap reference-count bump). */
    Env clone() const {
        LmEnv *out = nullptr;
        detail::check(lm_env_clone(handle_, &out));
        return Env(out);
    }

    Env deep_clone() const {
        LmEnv *out = nullptr;
        detail::check(lm_env_deep_clone(handle_, &out));
        return Env(out);
    }

    bool operator==(const Env &other) const {
        bool out = false;
        detail::check(lm_env_eq(handle_, other.handle_, &out));
        return out;
    }
    bool operator!=(const Env &other) const { return !(*this == other); }

    bool equal_contents(const Env &other) const {
        bool out = false;
        detail::check(lm_env_equal_contents(handle_, other.handle_, &out));
        return out;
    }

    std::size_t len() const {
        std::size_t out = 0;
        detail::check(lm_env_len(handle_, &out));
        return out;
    }

    bool is_empty() const {
        bool out = false;
        detail::check(lm_env_is_empty(handle_, &out));
        return out;
    }
    bool empty() const { return is_empty(); }

    LmEnvId id() const {
        LmEnvId out = {0, 0};
        detail::check(lm_env_id(handle_, &out));
        return out;
    }

    std::uint32_t next_idx() const {
        std::uint32_t out = 0;
        detail::check(lm_env_next_idx(handle_, &out));
        return out;
    }

    bool contains(const std::string &name) const {
        bool out = false;
        detail::check(lm_env_contains(handle_, name.c_str(), &out));
        return out;
    }

    LmVtype vtype_of(const std::string &name) const {
        LmVtype out;
        detail::check(lm_env_vtype_of(handle_, name.c_str(), &out));
        return out;
    }

    std::vector<std::string> sort(const std::vector<std::string> &names) const {
        std::vector<const char *> pointers;
        pointers.reserve(names.size());
        for (const std::string &name : names) {
            pointers.push_back(name.c_str());
        }
        LmStrBuffer buffer = {nullptr, 0, 0};
        detail::check(lm_env_sort(handle_, pointers.data(), pointers.size(), &buffer));
        std::vector<std::string> out;
        out.reserve(buffer.len);
        for (std::size_t index = 0; index < buffer.len; ++index) {
            out.emplace_back(buffer.data[index]);
        }
        lm_str_buffer_free(buffer);
        return out;
    }

    std::vector<std::uint8_t> serialize() const {
        LmBuffer buffer = {nullptr, 0, 0};
        detail::check(lm_env_serialize(handle_, &buffer));
        std::vector<std::uint8_t> out(buffer.data, buffer.data + buffer.len);
        lm_buffer_free(buffer);
        return out;
    }

    static Env deserialize(const std::uint8_t *data, std::size_t len) {
        LmEnv *out = nullptr;
        detail::check(lm_env_deserialize(data, len, &out));
        return Env(out);
    }
    static Env deserialize(const std::vector<std::uint8_t> &data) {
        return deserialize(data.data(), data.size());
    }

    /* Inserts a new variable with the default bounds for `vtype` and returns
     * a reference tied to this environment (Rust `env.insert(name, vtype,
     * None)`). Defined in vref.hpp. */
    std::vector<Vref> vars() const;
    Vref lookup(const std::string &name) const;
    Vref get(std::uint32_t id) const;
    Vref insert(const std::string &name, LmVtype vtype, const LazyBounds *bounds = nullptr);
    Vref insert_with_fallback(const std::string &name, LmVtype vtype,
                              const LazyBounds *bounds = nullptr,
                              const std::vector<std::uint64_t> *encoding = nullptr);
    Vref insert_inverted(const Vref &base);
    void remove(const Vref &target);

    LmEnv *get() const noexcept { return handle_; }
    LmEnv *release() noexcept {
        LmEnv *h = handle_;
        handle_ = nullptr;
        return h;
    }

  private:
    LmEnv *handle_;
};

} // namespace lm

#endif // LUNA_MODEL_ENV_HPP
