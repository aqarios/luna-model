#ifndef LUNA_MODEL_ENV_HPP
#define LUNA_MODEL_ENV_HPP

#include <luna_model.h>

#include "luna_model/detail/capi.hpp"
#include "luna_model/error.hpp"

namespace lm {

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
