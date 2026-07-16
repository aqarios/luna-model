#ifndef LUNA_MODEL_VERSION_HPP
#define LUNA_MODEL_VERSION_HPP

#include <cstdint>
#include <string>

#include <luna_model.h>

namespace lm {

/* Returns the LunaModel library version string. */
inline std::string version() { return std::string(lm_version()); }

/* Returns the C ABI version this library was built against. */
inline std::uint32_t abi_version() { return lm_abi_version(); }

} // namespace lm

#endif // LUNA_MODEL_VERSION_HPP
