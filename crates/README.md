# Crate structure

LunaModel is organized as a Rust workspace split into focused crates. Each crate owns one
logical part of the project, so that extensions — such as custom transformation passes — can
depend only on the components they actually use, keeping build times and binary size down.

The crates are combined and exposed through a single `lunamodel` crate. By default that crate
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
| `lunamodel-io` | Readers, writers, and `Display` implementations. |
| `lunamodel-translate` | Translators to and from formats such as LP and MPS. |
| `lunamodel-transform` | Builtin model transformations. |
| `lunamodel-transpiler` | The transformation and transpilation stack. |
| `lunamodel-transpiler-macros` | Procedural macros for the transpilation stack. |
| `lunamodel-python` | The Python interface. |
| `lunamodel-unwind` | Converts Rust panics into LunaModel errors across the FFI boundary. |
| `lunamodel-utils` | Shared utility helpers. |

The Python integration is provided through [`pyo3-lunamodel`](../pyo3-lunamodel/), which
implements the FFI between these crates and the LunaModel Python library.
