#ifndef LUNA_MODEL_VERSION_HPP
#define LUNA_MODEL_VERSION_HPP

#include <cstdint>
#include <string>

#include <luna_model.h>

namespace lm {

/* Returns the LunaModel library version string. */
inline std::string version() { return std::string(lm_version()); }

/* Returns the C ABI version of the LunaModel core in use.
 *
 * `abi_version` is a plain field of the C API table, not a function pointer, so extension mode has
 * no `lm_abi_version` dispatch macro: read it off the imported table (falling back to the version
 * this extension was compiled against if the table has not been imported yet). */
inline std::uint32_t abi_version() {
#if defined(LUNA_MODEL_PYTHON_EXTENSION)
    const LunaModelCApi *api = lunamodel_api();
    return api != nullptr ? api->abi_version
                          : static_cast<std::uint32_t>(LUNA_MODEL_CAPI_ABI_VERSION);
#else
    return lm_abi_version();
#endif
}

} // namespace lm

#endif // LUNA_MODEL_VERSION_HPP
