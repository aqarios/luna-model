# Crate structure

> [!IMPORTANT]
> The crate structure description and it's components are not final yet and might change going forward.
> This just reflects the current (_rough_) envisioned architecture of the lunamodel project.
> The main goal is to divide the project into it's logical components to enable reuse with minimal overhead
> in (_mainly_) private extensions such as special **transformation passes** that can be injected into the
> general LunaModel functionality without rebuilding the project.
> 
> Any extension can depend on the components it actually uses and requires, minimizing the extensions size and 
> build times. The main interaction with LunaModel is via Python, to enable compatibility with the LunaModel 
> python library the [`pyo3-lunamodel`](../pyo3-lunamodel/) is used, implementing the FFI for all `core` structs
> and `types` from and to python. Extensions to LunaModel can thus be written in pure Rust (_mostly_).

All crates are called `lunamodel-*` but stored in the directories without the `lunamodel-` prefix to 
make the directory structure less verbose. All crates are combined and accessible via a single crate 
`lunamodel`. By default this crate only includes the base components (`types`, `core`, `error`, ...).
Using features other required components can be enabled addining and exposing the feature-crates to 
`lunamodel`.

> [!IMPORTANT]
> This most likely changes in the future, it's a rough sketch of the architecture.
> Crates might be deleted or combined depending on implementation details.

- `lunamodel-error` defined the single error and result type used in all subcrates. Not sure if the enum variation is good or not. We'll see.
- `lunamodel-types` are all "primitive" types such as the type of a variable or of constraints.
- `lunamodel-core` contains all "first-class citizen" structs relevant for defining optimization problems as a model.
- `lunamodel-ops` mathematical operations on core structs. Not sure if this works as expected... Also might include more operations ?
- `lunamodel-serializer`
- `lunamodel-python` defines and implements the entire python interface. **Open Question** how to handle enums: (1) annotate where they are defined or (2) a wrapper enum.
    - `lunamodel-unwind` python specific proc macro to transform a panic into an Error type. Also changes the result type of pymethods to `PyResult` if they don't yet return a `PyResult` compatible result type.
    - `lunamodel-hashing` relevant for the python interface to hash a model. Uses a special serialization, maybe we need something more stable though that does not change when we alter the model but remains valid as long as nothing is removed.
- `lunamodel-transform` is the transformation/transpilation stack to transform a model (source) into a target representation (target). 
    - `lunamodel-tpass` proc macro to annotate a transformation pass.
- `lunamodel-translate` contains LunaModel to many other model representations such as LP or dimod etc.

Not sure if we actually need:

- `lunamodel-io` implements all reader and writers. Such as display of all structs. 

Will most likely **not** need:

- `lunamodel-utils` don't think we actually have overarching utilities.
