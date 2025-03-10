# Aq-Models (Rust)

The rust based `aq-models` implementation.

## TODOs

- [x] Equality of Expressions
- [x] Equality of Models (uses expressions eq + some more checks)
- [x] Model Constraints (lhs is an expression and rhs is a single value and an equality type: Leq, Eq, Geq)
- [ ] Model evaluation (takes solution object and computes value)
- [ ] Solution object gives variables values
- [x] Model serialization and deserialization (versioned)
- [ ] Variable fixing
- [ ] Variable substitution
- [ ] From any to all others (types)
- [ ] Transformations (common ones required for initial release, to/from qubo, to/from lp, to/from dimod)
- [x] String representation of expressions
- [x] String representation of models


## Benchmarks

See [here](./report.md)
