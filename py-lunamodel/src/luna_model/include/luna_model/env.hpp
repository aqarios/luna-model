#ifndef LUNA_MODEL_ENV_HPP
#define LUNA_MODEL_ENV_HPP

// Umbrella header for the `Env` type. The implementation is split across the
// env/ directory so each file stays small and focused.
//
//   env/env.hpp          class declaration + lifecycle
//   env/access.hpp       const read accessors
//   env/modification.hpp mutators (insert / remove / set_inverted)
//   env/creation.hpp     construction, deep-clone, (de)serialization
//
// The include graph is one-directional (env -> vref -> variable -> bounds), so
// there are no reverse includes or ordering tricks. Including this single
// header yields a fully usable `Env` (and `Vref`).

#include "luna_model/env/access.hpp"       // IWYU pragma: export
#include "luna_model/env/creation.hpp"     // IWYU pragma: export
#include "luna_model/env/env.hpp"          // IWYU pragma: export
#include "luna_model/env/modification.hpp" // IWYU pragma: export

#endif // LUNA_MODEL_ENV_HPP
