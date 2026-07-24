# cxx-lunamodel

Header-only C++ wrapper over the LunaModel C API (`liblunamodel`): RAII handles and
exceptions over the `lm_*` C functions.

## Usage

Build the C library and headers first (from the repo root):

    just c        # produces dist/c/lib/liblunamodel.* and dist/c/include

Then add two include paths and link the library:

- `-Icxx-lunamodel/src` (this library)
- `-Idist/c/include` (the C API headers)
- `-Ldist/c/lib -llunamodel`

```cpp
#include <luna_model.hpp>

int main() {
    lm::Model model;
    std::vector<std::uint8_t> bytes = model.serialize();
    lm::Model restored = lm::Model::deserialize(bytes);
}
```

See `samples/` for a buildable example and `test/` for the CTest suite:

    just cpp-samples
    just cpp-test

## Editor setup (clangd / Neovim)

Run `just cpp-test` once to generate `test/build/compile_commands.json`; the
`.clangd` file points clangd at it so `<luna_model.h>` and the wrapper headers
resolve while editing.
