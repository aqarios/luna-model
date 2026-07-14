# cxx-lunamodel — C++ API status

The C++ API is a standalone, first-class wrapper over the LunaModel C ABI
(`crates/cext/include/luna_model/funcs.h`). It is header-only and lives under
`src/luna_model/`, with the umbrella header `<luna_model.hpp>` as the entry point.

## Status: complete

Every LunaModel C ABI function is wrapped (or its functionality is otherwise
exposed). Types and their surfaces:

- `Env` — split across `env/` (declaration, access, modification, creation).
- `Vref`, `Variable`, `Bound`, `Bounds`, `LazyBounds`.
- `Expression` (+ `Linear`, `Quadratic`, `HigherOrder`) and the `ops.hpp`
  operator overloads.
- `Constraint`, `ConstraintCollection`.
- `Model`, `Specs`.
- `Solution`, `Column`, `ResultView`, `SampleView`, `Timing`, `Timer`.
- Free functions `version()`, `abi_version()`.

Handle wrappers are move-only (copy deleted), free their handle in the
destructor, and expose `get()` / `release()` for C interop. Fallible C calls go
through `detail::check(...)`; owned buffers are copied into `std::vector` /
`std::string` / `std::map` and released with the matching `lm_*_buffer_free`.

### C ABI entries intentionally not wrapped 1:1

These are covered functionally without a dedicated wrapper call:

- `lm_bound_is_bounded` — `Bound::is_bounded()` reads the POD tag directly.
- `lm_cc_default` — the `ConstraintCollection()` default constructor uses the
  equivalent `lm_cc_new`.
- `lm_u32_buffer_free` — no wrapped accessor returns a standalone `LmU32Buffer`
  (nested index buffers are freed by their owning item-buffer free).

## Tests

`test/test_full_api.cpp` exercises the model / constraint-collection / specs /
lazy-bounds / variable / sample-view surface and the added solution, expression,
and term-store operations. The full suite (`test/test_*.cpp`) passes against the
real `liblunamodel`:

```
make cpp-test      # builds dist/c, configures CMake, builds the suite, runs ctest
```

## Possible follow-ups (ergonomic, beyond ABI coverage)

Convenience layers that exist in other bindings but are not required for full
ABI coverage: iterator/`iter_mut` editors for the term stores and constraint
collection, predicate-based `Solution::filter`, richer `SampleView` accessors
(`to_vec`, `as_map`, `try_get`), and an owned mutable `Environment` proxy.

---

## Appendix — layout

Each type is one small header (or, for `Env`, a small directory). The include
graph is strictly one-directional, e.g.:

```
model -> solution -> {result_view, sample_view, column, timing}
model -> constraint_collection -> constraint -> expr -> {env, linear, quadratic, higher_order}
env   -> vref -> variable -> bounds -> bound
```

To break the one mutual `Env`<->`Vref` relationship, `vref.hpp` forward-declares
`Env` and only declares the id-based constructor; its definition lives in
`env/creation.hpp`, where `Env` is complete. There are no reverse includes and
no include-ordering tricks, so including any type's header (or the umbrella
`<luna_model.hpp>`) yields a fully usable type.
