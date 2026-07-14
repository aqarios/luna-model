#ifndef LUNA_MODEL_ENV_ENV_HPP
#define LUNA_MODEL_ENV_ENV_HPP

#include <cstdint>
#include <string>
#include <utility>
#include <vector>

#include <luna_model.h>

#include "luna_model/detail/capi.hpp"
#include "luna_model/error.hpp"
#include "luna_model/lazy_bounds.hpp" // `LazyBounds` param of insert()
#include "luna_model/variable.hpp"    // `Variable` param of create()
#include "luna_model/vref.hpp"        // `Vref` return/param type

namespace lm {

/* Owning handle to a variable environment (frees on destruction). An
 * environment stores the variables of a problem and maps names to indices.
 *
 * The implementation is split across this directory so each file stays small
 * and focused:
 *
 *   env/env.hpp          this file — the class declaration + lifecycle
 *   env/access.hpp       const read accessors
 *   env/modification.hpp mutators (insert / remove / set_inverted)
 *   env/creation.hpp     construction, deep-clone, (de)serialization
 *
 * The include graph is one-directional (`env -> vref -> variable -> bounds`),
 * so every file in this directory sees a complete `Vref`. There are no reverse
 * includes and no ordering tricks: including <luna_model/env.hpp> yields a
 * fully usable `Env`.
 */
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

    // --- construction (env/creation.hpp) ---

    /* Builds an environment from existing variables and a name->index lookup.
     * Variable handles are borrowed (cloned internally), not consumed;
     * `next_idx` is the next free variable id. */
    static Env create(const std::vector<std::pair<std::uint32_t, const Variable *>> &variables,
                      const std::vector<std::pair<std::string, std::uint32_t>> &lookup,
                      std::uint32_t next_idx);
    /* Deserializes an environment previously produced by `serialize`. */
    static Env deserialize(const std::uint8_t *data, std::size_t len);

    // --- const accessors (env/access.hpp) ---

    /* Number of variables in the environment. */
    std::size_t len() const;
    /* Whether the environment has no variables. */
    bool is_empty() const;
    /* The next free variable index. */
    std::uint32_t next_idx() const;
    /* Stable 128-bit environment identity, as { hi, lo }. */
    LmEnvId id() const;
    /* Whether a variable named `name` exists. */
    bool contains(const std::string &name) const;
    /* The type of the variable named `name`. */
    LmVtype vtype_of(const std::string &name) const;
    /* Structural content equality. */
    bool equal_contents(const Env &other) const;
    /* Reorders `names` into the environment's canonical order. */
    std::vector<std::string> sort(const std::vector<std::string> &names) const;
    /* All variable references. */
    std::vector<Vref> vars() const;
    /* A reference to the variable named `name`. */
    Vref lookup(const std::string &name) const;
    /* A reference to the variable with index `vid`. Note: the zero-argument
     * `get()` below returns the raw C handle instead. */
    Vref get(std::uint32_t vid) const;

    // --- mutation (env/modification.hpp) ---

    /* Inserts a variable and returns a reference to it. `bounds == nullptr`
     * uses the default bounds for `vtype`. */
    Vref insert(const std::string &name, LmVtype vtype, const LazyBounds *bounds = nullptr);
    /* Inserts a variable, retrying with an encoding fallback on name clashes. */
    Vref insert_with_fallback(const std::string &name, LmVtype vtype,
                              const std::vector<std::uint64_t> &enc,
                              const LazyBounds *bounds = nullptr);
    /* Inserts the inverted-binary companion of `vref`. */
    Vref insert_inverted(const Vref &vref);
    /* Removes the variable referenced by `vref`. */
    void remove(const Vref &vref);
    /* Sets or clears the inverted companion index of the variable at `index`.
     * Pass `present == false` to clear. */
    void set_inverted(std::uint32_t index, bool present, std::uint32_t inverted);

    // --- misc (env/creation.hpp) ---

    /* Deep structural copy — a genuinely independent environment, unlike the
     * refcount-bump `clone()`. */
    Env deep_clone() const;
    /* Serializes the environment to bytes. */
    std::vector<std::uint8_t> serialize() const;

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

} // namespace lm

#endif // LUNA_MODEL_ENV_ENV_HPP
