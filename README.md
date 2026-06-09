<h1 align="center">
  <img src="./assets/luna_model_logo.svg" alt="LunaModel" width="420">
</h1>

<p align="center">
  Fast, symbolic modeling for optimization — a high-performance Rust core with a Python-first API.
  <br />
  Build, translate, and transform optimization models.
</p>

<p align="center">
  <a href="https://pypi.org/project/luna-model/"><img src="https://img.shields.io/pypi/v/luna-model.svg?color=2563eb&label=PyPI" alt="PyPI version"></a>
  <a href="https://pypi.org/project/luna-model/"><img src="https://img.shields.io/badge/python-3.11%20%7C%203.12%20%7C%203.13%20%7C%203.14-2563eb" alt="Supported Python versions"></a>
  <a href="./LICENSE"><img src="https://img.shields.io/badge/license-Apache%202.0-2563eb.svg" alt="License: Apache 2.0"></a>
</p>

<p align="center">
  <a href="#about">About</a>
  ·
  <a href="#installation">Installation</a>
  ·
  <a href="#quick-example">Example</a>
  ·
  <a href="https://docs.aqarios.com/luna-model/intro">Documentation</a>
  ·
  <a href="./CONTRIBUTING.md">Contributing</a>
</p>

## About

LunaModel is a high-performance symbolic modeling library for describing, translating, and
transforming optimization problems. It provides Rust crates for model representation,
translation, serialization, and transformation, with a Python-first API exposed through the
[`luna-model`](https://pypi.org/project/luna-model/) package.

Use LunaModel when you need to build optimization models programmatically, convert between
common model formats, or prepare models for solver- or platform-specific workflows. You can
use it standalone, or through [luna-quantum](https://pypi.org/project/luna-quantum/) to solve
your problems on the [Luna Platform](https://aqarios.com/platform).

- **Symbolic modeling** — define algebraic expressions of arbitrary degree, constraints, and
  optimization models (in the spirit of dimod, Gurobi, or CPLEX).
- **Translators** — convert to and from LunaModel for many common formats, including LP, MPS,
  QUBO, BQM, CQM, and several solution result formats.
- **Transformations** — a compilation/transpilation stack to map a model into a target
  representation, e.g. CQM → BQM or integer → binary.
- **Serialization** — built-in, portable serialization for models and solutions.
- **Python-first** — an ergonomic Python API backed by a fast Rust core.
- **Rust crates** — use the workspace crates directly when embedding LunaModel in Rust.

## Installation

Install the package from PyPI with [uv](https://docs.astral.sh/uv/):

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
```

## Quick Example

The **Knapsack Problem**: given $n$ items, each with a weight $w_i$ and value $v_i$, and a
capacity $W$, select items to maximize total value without exceeding the capacity.

```math
\begin{align*}
&\text{maximize} \sum_{i=1}^{n} v_i x_i \\
&\text{subject to} \sum_{i=1}^{n} w_i x_i \leq W \quad \text{and} \quad x_i \in \{ 0, 1 \}
\end{align*}
```

```python
from luna_model import Model, Sense, Vtype
from luna_model.utils import quicksum

weights = [1.5, 10.0, 5.2, 3.5, 8.32]
values = [10.0, 22.0, 3.2, 1.99, 6.25]
capacity = 25

model = Model(sense=Sense.MAX, name="Knapsack")
items = [model.add_variable(f"x_{i}", vtype=Vtype.BINARY) for i in range(len(weights))]

model.objective = quicksum(values[i] * items[i] for i in range(len(items)))
model.constraints += quicksum(weights[i] * items[i] for i in range(len(items))) <= capacity

print(model)
```

Variables are `BINARY` by default and can also be `SPIN`, `INTEGER`, or `REAL`. See the
[documentation](https://docs.aqarios.com/luna-model/intro) for bounded variants, integer models, and more.

## Components

| Component                    | Description                                                                                                |
| ---------------------------- | ---------------------------------------------------------------------------------------------------------- |
| **LunaModel**                | A symbolic modeling library for arbitrary optimization models.                                             |
| **LunaModel.translator**     | A translation library that supports many common model formats.                                             |
| **LunaModel.transformation** | A compilation and transpilation stack to transform a model (source) into a target representation (target). |
| **LunaModel.utils**          | Utility functions for expression and model creation.                                                       |
| **LunaModel.errors**         | All error types that can be raised within LunaModel.                                                        |

## Development

This repository is a Rust workspace with the Python package in `py-lunamodel`.

**Prerequisites:** Rust and Cargo, [uv](https://docs.astral.sh/uv/), and Python 3.11 or newer.

```bash
git clone https://github.com/aqarios/luna-model.git
cd luna-model

# set up the Python development environment
cd py-lunamodel
uv sync
```

Useful local checks:

```bash
# Python tests, linting, and formatting (from py-lunamodel)
uv run pytest
uv run ruff check .
uv run ruff format .

# Rust checks (from the repository root)
cargo test --workspace
cargo fmt --all

# build the Python wheel (from py-lunamodel)
uv build
```

To build the Rust API documentation locally:

```bash
cargo doc --workspace --no-deps
```

## Documentation

The full documentation is published at [docs.aqarios.com](https://docs.aqarios.com/luna-model/intro).

## Communication

- Start in [GitHub Discussions](https://github.com/aqarios/luna-model/discussions) for bug
  reports, feature ideas, questions, and usage help. Maintainers triage discussions and turn
  confirmed, actionable items into issues — see [CONTRIBUTING.md](./CONTRIBUTING.md).
- Open a pull request when a change is ready for review. For anything beyond a small, obvious
  fix, make sure there is an accepted issue or an agreed direction in a discussion first.

## Contributing

Bug fixes, documentation improvements, examples, and focused feature work are welcome. See
[CONTRIBUTING.md](./CONTRIBUTING.md) for branch naming, local checks, pull request
expectations, and release notes.

## License

LunaModel is licensed under the Apache License 2.0. See [LICENSE](./LICENSE).
