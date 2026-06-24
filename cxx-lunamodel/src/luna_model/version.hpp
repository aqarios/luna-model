#ifndef LUNA_MODEL_VERSION_HPP
#define LUNA_MODEL_VERSION_HPP

#include <string>

#include <luna_model.h>

namespace lm {

/* Returns the LunaModel library version string. */
inline std::string version() { return std::string(lm_version()); }

} // namespace lm

#endif // LUNA_MODEL_VERSION_HPP
