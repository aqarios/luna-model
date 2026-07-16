# cxx-lunamodel — C++ API status

A standalone, first-class C++ wrapper over the LunaModel C ABI. Header-only under
`src/luna_model/`; the umbrella header `<luna_model.hpp>` is the entry point. The
API mirrors the shape and naming of the Rust `rs-lunamodel` binding so the two
feel the same, allowing for language differences.

## Status

**C ABI coverage:** every function is wrapped (or exposed via composition).

**rs-API parity layer** (client-side reconstructions on top of the ABI, matching
`rs-lunamodel`):

- `Vref`: `evaluate`, `to_variable`, `pow`, `clone` (names aligned with rs).
- `Env`: `get_variable`; single-file (see below).
- `Column`: construction, cloning, filtering, extraction, value access, and mutation.
- `Expression`: `linear_terms`/`quadratic_terms`/`higher_order_terms`,
  `linear(idx)`/`quadratic(u,v)`/`higher_order(vars)`, `raw_*_items`, `*_items`,
  `items`, `contains`, `from_vref`, `deep_clone_many`, `evaluate_sample`,
  `evaluate_sample_quick`, `evaluate_sampleset`.
- `Constraint`: `evaluate_sample`, `evaluate_sample_quick`.
- `ConstraintCollection`: `iter`, `add_many_constraints`, `evaluate_sample`,
  `evaluate_sample_quick`.
- `SampleView`: `to_vec`, `get`, `try_get`, `iter`, `iter_named`, `as_map`.
- `Solution`: `sample`, predicate-based `filter`.
- `Model`: `violated_constraints`.
- `Specs`: optional-returning accessors + `diff`.
- `ResultView`: lazy handle wrapper with `counts`/`obj_value`/`raw_energy`/
  `feasible`/`constraints`/`variable_bounds`/`sample` accessors.

Shared float-comparison/evaluation helpers live in `detail/eval.hpp` (ported
from `lunamodel_utils`: `DEFAULT_TOL`, `float_eq/le/ge`, `validate_tol`).

## Intentional differences from rs-lunamodel

Driven by language differences or the C ABI boundary:

- **No owned `Environment` + `VariableMut`/`commit` editor.** That borrow-checker
  -driven proxy is Rust-specific; `Env` is mutated in place in C++.
- **No `iter_mut`/`commit` editors** on the term stores / constraint collection.
  In C++ you mutate directly (`add_entry`, `set_bias`, `set_constraint`).
- **`clone()` is not named `try_clone`** — it throws on error, the C++ idiom.
- **Scalar factories** are `from_double` / `from_size_t` (not `from_f64`/`from_usize`).
- **`SampleViewIdx`** is expressed as overloads of `try_get(size_t|Vref|string)`.

## Layout

One header per type. `Env` is a single `env.hpp` like every other type. The
include graph is one-directional; `vref.hpp` forward-declares `Env` and the
`Vref(id, env)` constructor is defined in `env.hpp` where both types are
complete — the only cross-type "bridge".

## Tests

```
make cpp-test      # builds dist/c, configures CMake, builds the suite, runs ctest
```

`test/test_full_api.cpp` and `test/test_parity.cpp` exercise the wrapped ABI
surface and the rs-parity reconstructions at runtime. The full suite passes
against the real `liblunamodel`.
