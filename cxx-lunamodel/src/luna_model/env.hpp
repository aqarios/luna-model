#ifndef LUNA_MODEL_ENV_HPP
#define LUNA_MODEL_ENV_HPP

#include <luna_model.h>

namespace lm {

/* Owning handle to a LunaModel env (frees on destruction).
 *
 * Mirrors the Rust `ArcEnv`.
 */
class Env {

  public:
    LmEnv *get() const { return handle_; }

  private:
    LmEnv *handle_;
};

} // namespace lm

#endif // LUNA_MODEL_ENV_HPP
