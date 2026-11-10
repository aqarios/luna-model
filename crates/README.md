# Crate structure

LunaModel is organized as a Rust workspace split into focused crates. Each crate owns one
logical part of the project, so that extensions — such as custom transformation passes — can
depend only on the components they actually use, keeping build times and binary size down.

The crates are combined and exposed through a single `luna-model` crate (`crates/lunamodel`). By default that crate
includes only the base components (`types`, `core`, `error`, ...); the remaining components are
enabled through Cargo features.

All crates are named `lunamodel-*` but live in directories without the `lunamodel-` prefix to
keep the workspace layout concise.

> [!NOTE]
> This layout reflects the current architecture and may evolve as the project matures — crates
> may be added, combined, or removed.

## Crates

| Crate | Description |
| ----- | ----------- |
| `lunamodel-types` | Primitive types such as variable and constraint types. |
| `lunamodel-core` | Core structs for defining optimization models. |
| `lunamodel-error` | Shared error and result types. |
| `lunamodel-serializer` | Serialization for models and solutions. |
| `lunamodel-hashing` | Stable hashing of models. |
| `lunamodel-fmt` | Readers, writers, and `Display` implementations. |
| `lunamodel-translate` | Translators to and from formats such as LP and MPS. |
| `lunamodel-transform` | Builtin model transformations. |
| `lunamodel-transpiler` | The transformation and transpilation stack. |
| `lunamodel-transpiler-macros` | Procedural macros for the transpilation stack. |
| `lunamodel-python` | The Python interface. |
| `lunamodel-unwind` | Converts Rust panics into LunaModel errors across the FFI boundary. |
| `lunamodel-utils` | Shared utility helpers. |
| `lunamodel-capi` | ABI types of the C API: the `LunaModelCApi` dispatch table struct, handles, buffers, and status codes. Has no dependencies. |
| `lunamodel-capi-host` | The C API implementation (`liblunamodel`), its filled dispatch table, and the generated C headers. |
| `lunamodel-capi-handle` | Storage behind the C API's opaque handles. |
| `lunamodel-capi-pyffi` | PyCapsule transport for C API handles between `luna_model` and Python extensions. |
| `lunamodel-capi-plugin` | Host-side adapters that run C ABI plugin passes in the transpiler. |
| `lunamodel-capi-plugin-proto` | Protobuf messages of the built-in analysis results that plugins consume. |

The `luna_model._lm` Python module is built from [`py-lunamodel/pylm`](../py-lunamodel/pylm/),
which combines `lunamodel-python` with `lunamodel-capi-host`. Native extensions use the C API
through [`luna-model-ffi`](../rs-lunamodel/) (Rust) or [`cxx-lunamodel`](../cxx-lunamodel/) (C++).
