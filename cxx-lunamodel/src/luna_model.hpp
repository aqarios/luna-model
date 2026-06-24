/*
 * LunaModel C++ API — header-only RAII wrapper over the LunaModel C API.
 *
 * Standalone: link liblunamodel; add cxx-lunamodel/src and dist/c/include to your
 * include path.
 *     #include <luna_model.hpp>
 *     lm::Model model;
 *     std::vector<std::uint8_t> bytes = model.serialize();
 *
 * Python extension: define LUNA_MODEL_PYTHON_EXTENSION before including, then call
 * import_luna_model() once during module init (lm_* then dispatch through the
 * wheel's _C_API capsule, same call syntax).
 */
#ifndef LUNA_MODEL_HPP
#define LUNA_MODEL_HPP

#include "luna_model/error.hpp"
#include "luna_model/model.hpp"
#include "luna_model/solution.hpp"
#include "luna_model/version.hpp"

#endif // LUNA_MODEL_HPP
