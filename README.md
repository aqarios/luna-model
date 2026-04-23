<div align="center">
  <img src="./assets/luna_model_logo.svg" alt="LunaModel" width="400">
</div>

# LunaModel

[![Main CI/CD](https://github.com/aqarios/luna-model/actions/workflows/main-ci.yml/badge.svg)](https://github.com/aqarios/luna-model/actions/workflows/main-ci.yml)
[![Docs](https://github.com/aqarios/luna-model/actions/workflows/pages.yml/badge.svg)](https://github.com/aqarios/luna-model/actions/workflows/pages.yml)

LunaModel is a symbolic modeling library for optimization problems. It provides Rust crates for model representation, translation, serialization, transformation, and Python bindings exposed through the `luna_model` package.

Use LunaModel when you need to build optimization models programmatically, convert between common model formats, or prepare models for solver- or platform-specific workflows.

## What Is Included

- Symbolic variables, expressions, constraints, models, and solutions.
- Python bindings with a Python-first modeling API.
- Translators for common model formats and solver ecosystems, including LP, MPS, QUBO, BQM, CQM, and several solution result formats.
- Transformation infrastructure for moving models between supported representations.
- Serialization support for portable model and solution data.
- Rust crates that can be used directly when embedding LunaModel in Rust projects.

## Installation

For Python users, install the public package from PyPI:

```bash
uv add luna-model
```

or with pip:

```bash
pip install luna-model
```

The package is imported as `luna_model`:

```python
from luna_model import Model, Sense, Vtype
from luna_model.utils import quicksum

weights = [1.5, 10.0, 5.2, 3.5, 8.32]
values = [10.0, 22.0, 3.2, 1.99, 6.25]
capacity = 25

model = Model(sense=Sense.MAX, name="Knapsack")
items = [
    model.add_variable(f"x_{idx}", vtype=Vtype.BINARY)
    for idx in range(len(weights))
]

model.objective = quicksum(values[idx] * items[idx] for idx in range(len(items)))
model.constraints += quicksum(weights[idx] * items[idx] for idx in range(len(items))) <= capacity
```

## Development Setup

This repository contains a Rust workspace and the Python package in `py-lunamodel`.

Prerequisites:

- Rust and Cargo
- [uv](https://docs.astral.sh/uv/)
- Python 3.11 or newer

Clone the repository with submodules:

```bash
git clone --recurse-submodules https://github.com/aqarios/luna-model.git
cd luna-model
```

Set up the Python development environment:

```bash
cd py-lunamodel
uv sync
```

Useful local checks:

```bash
# Python tests
uv run pytest

# Python linting and formatting
uv run ruff check .
uv run ruff format .

# Rust checks from the repository root
cargo test --workspace
cargo fmt --all
```

Build the Python wheel from `py-lunamodel`:

```bash
uv build
```

## Documentation

The public documentation is published at [docs.aqarios.com](https://docs.aqarios.com).

To build the combined Python and Rust documentation site locally from the repository root:

```bash
ci/tools/build-docs-site.sh
```

The generated site is written to `site/`. The entry page links to the Python documentation and the Rust workspace documentation.

To build only the Rust documentation:

```bash
cargo doc --workspace --no-deps --document-private-items
```

## Communication

- Use [GitHub Issues](https://github.com/aqarios/luna-model/issues) for bug reports, concrete feature proposals, and use cases that need project tracking.
- Use [GitHub Discussions](https://github.com/aqarios/luna-model/discussions) for questions, design discussion, examples, and usage help.
- Open a pull request when a change is ready for review. For larger features, start with an issue or discussion so the direction is clear before implementation work begins.

## Contributing

Bug fixes, documentation improvements, examples, and focused feature work are welcome. See [CONTRIBUTING.md](./CONTRIBUTING.md) for branch naming, local checks, pull request expectations, and release notes.

## License

LunaModel is licensed under the Apache License 2.0. See [LICENSE](./LICENSE).
