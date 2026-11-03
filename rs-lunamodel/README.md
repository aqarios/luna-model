# rs-lunamodel

Rust SDK for LunaModel extensions. It mirrors the native `luna-model` Rust API (`Model`,
`Expression`, `Environment`, `ConstraintCollection`, `Solution`, ...) but routes every call
through the LunaModel C API, so an extension works on the objects of the `luna_model` it runs
in instead of linking its own copy of the core. Its only LunaModel dependencies are
`lunamodel-capi` (the ABI types) and `lunamodel-types`.

## Features

- `py`: pyo3 types for Python extensions (`PyModel`, `PyExpression`, `PySolution`, ... in
  `rs_lunamodel::py::prelude`) and macros that expose plugin passes to Python.

## Python extensions

```toml
[dependencies]
pyo3 = { version = "0.29", features = ["extension-module"] }
rs-lunamodel = { git = "https://github.com/aqarios/luna-model", features = ["py"] }
```

The `Py*` types convert to and from `luna_model` objects. They are handles to the same object,
so a change made in Rust is visible in Python:

```rust
use pyo3::prelude::*;
use rs_lunamodel::py::prelude::*;

#[pyfunction]
fn rename(mut model: PyModel, name: String) -> PyResult<()> {
    model.set_name(name).map_err(into_pyerr)
}

#[pymodule]
fn my_ext(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(rename, m)?)?;
    Ok(())
}
```

The conversions read the C API table from the `luna_model._lm._C_API` capsule on first use.
Code that calls into the API before any conversion calls `rs_lunamodel::py::ensure_c_api(py)`
first.

## Plugins

`rs_lunamodel::plugin` mirrors the native pass traits: `TransformationPass` (with `Reversible`
and an `Artifact`), `AnalysisPass`, `MetaAnalysisPass`, `CompositePass`, and `ControlFlowPass`.
`py_transform_plugin!`, `py_analysis_plugin!`, `py_meta_analysis_plugin!`,
`py_composite_plugin!`, and `py_control_flow_plugin!` turn a pass into a Python class, and
`register_plugins!` in the `#[pymodule]` function registers the reversal handlers of the
reversible passes. From Python:

```python
import luna_model
import my_ext
from luna_model.transformation import PassManager

luna_model.register(my_ext)
out = PassManager([my_ext.MyAnalysis(), my_ext.MyTransform()]).run(model)
```

Artifacts and analysis results are protobuf messages (`prost`). The schemas of the built-in
analysis results ship in the wheel (`luna_model.capi.get_proto()`). A complete extension with
every pass kind is in [`tests/plugin_ext/src/lib.rs`](tests/plugin_ext/src/lib.rs).

## Rust programs

A Rust program that links the host directly installs its table once:

```rust
unsafe { rs_lunamodel::init(&lunamodel_capi_host::capi_table::LUNA_MODEL_C_API) }?;
```

## Tests

`cargo test` runs the parity tests against `lunamodel-capi-host`. The Python-driven extension
tests are described in [`tests/README.md`](tests/README.md) and run with `just rs-ext-test`
and `just rs-plugin-test` from the repository root.
