<h1 align="center">
  <img src="./assets/luna_model_logo.svg" alt="LunaModel" width="420">
</h1>

<p align="center">
  <strong>Fast, symbolic modeling for optimization.</strong>
  <br />
  Build, translate, and transform optimization models on a high-performance Rust core,
  from Python, C, C++, Rust, or JavaScript.
</p>

<p align="center">
  <a href="https://pypi.org/project/luna-model/"><img src="https://img.shields.io/pypi/v/luna-model.svg?color=2563eb&label=PyPI" alt="PyPI version"></a>
  <a href="https://www.npmjs.com/package/@aqarios/luna-model"><img src="https://img.shields.io/npm/v/%40aqarios%2Fluna-model.svg?color=2563eb&label=npm" alt="npm version"></a>
  <a href="https://pypi.org/project/luna-model/"><img src="https://img.shields.io/badge/python-3.11%20%E2%80%93%203.15-2563eb" alt="Supported Python versions"></a>
  <a href="https://github.com/aqarios/luna-model/actions/workflows/main-ci.yml"><img src="https://img.shields.io/github/actions/workflow/status/aqarios/luna-model/main-ci.yml?branch=main&label=CI" alt="CI status"></a>
  <a href="./LICENSE"><img src="https://img.shields.io/badge/license-Apache%202.0-2563eb.svg" alt="License: Apache 2.0"></a>
</p>

<p align="center">
  <a href="#installation">Installation</a>
  ·
  <a href="#quick-start">Quick start</a>
  ·
  <a href="#extensions">Extensions</a>
  ·
  <a href="https://docs.aqarios.com/luna-model/intro">Documentation</a>
  ·
  <a href="./CONTRIBUTING.md">Contributing</a>
</p>

---

LunaModel describes optimization problems as symbolic models, converts them between common
formats, and compiles them into the representation a solver or platform expects. Use it
standalone, or through [luna-quantum](https://pypi.org/project/luna-quantum/) to solve your
problems on the [Luna Platform](https://aqarios.com/platform).

- **Symbolic modeling**: algebraic expressions of arbitrary degree, constraints, and models
  over binary, spin, integer, and real variables, with NumPy-style vectorized construction.
- **Translators**: LP, MPS, QUBO, BQM, CQM, Qiskit, and several solution result formats.
- **Transformations**: a compilation stack of analysis and transformation passes, e.g.
  constrained → unconstrained or integer → binary, with solutions mapped back to the
  original model.
- **Serialization**: portable, versioned serialization for models and solutions.
- **C API**: a versioned C ABI with a header-only C++ wrapper and a Rust SDK, for native
  extensions and for using LunaModel outside Python.
- **Plugins**: passes written in Python, Rust, or C++ run together in one pass manager.

## Installation

| Language                | Package                                                                    | Scope                                                    |
| ----------------------- | -------------------------------------------------------------------------- | -------------------------------------------------------- |
| Python                  | [`luna-model`](https://pypi.org/project/luna-model/) on PyPI               | Modeling, translation, transformation, and serialization |
| JavaScript / TypeScript | [`@aqarios/luna-model`](https://www.npmjs.com/package/@aqarios/luna-model) | Reading and inspecting serialized solutions              |
| C / C++                 | [`cxx-lunamodel`](./cxx-lunamodel/) and `liblunamodel`                     | Python extensions and standalone native programs         |
| Rust                    | [`rs-lunamodel`](./rs-lunamodel/) and the [workspace crates](./crates/)    | Python extensions over the C API, or embedding the core  |

```bash
uv add luna-model          # or: pip install luna-model
npm install @aqarios/luna-model
```

The C/C++ headers ship in the Python wheel (`luna_model.capi.get_include()`). The Rust crates
are used as git dependencies of this repository; see [Extensions](#extensions).

## Quick start

### Build a model

The **Multi-Dimensional Knapsack Problem**: given $n$ items with values $v_i$ and weights
$w_{ij}$ in each dimension $j \in J$ with capacity $W_j$, select items to maximize the total
value without exceeding any capacity.

```math
\max \sum_{i=1}^{n} v_i x_i \quad \text{s.t.} \quad \sum_{i=1}^{n} w_{ij} x_i \leq W_j \;\; \forall j \in J, \qquad x_i \in \{0, 1\}
```

```python
import numpy as np

from luna_model import Model, Sense, Vtype
from luna_model.utils import quicksum

weights = np.array([[1.5, 1.0], [10.0, 3.0], [5.2, 9.0], [3.5, 12.0], [8.32, 4.0]])
values = np.array([10.0, 22.0, 3.2, 1.99, 6.25])
capacity = np.array([25, 19])

model = Model(sense=Sense.MAX, name="Knapsack")
items = model.add_variables(name="x", shape=values.shape, vtype=Vtype.BINARY)
model.set_objective(quicksum(values * items))
model.add_constraints(weights.T @ items <= capacity, name="capacity")

print(model)
```

```text
Model: Knapsack
Maximize
  10 * x0 + 22 * x1 + 3.2 * x2 + 1.99 * x3 + 6.25 * x4
Subject To
  capacity_0: 1.5 * x0 + 10 * x1 + 5.2 * x2 + 3.5 * x3 + 8.32 * x4 <= 25
  capacity_1: x0 + 3 * x1 + 9 * x2 + 12 * x3 + 4 * x4 <= 19
Binary
  x0 x1 x2 x3 x4
```

### Translate

```python
from luna_model.translator import LpTranslator

lp = LpTranslator.from_lm(model)    # LP-format string
restored = LpTranslator.to_lm(lp)   # back to a Model
```

### Transform

```python
from luna_model.transformation import PassManager, pipelines

out = PassManager([pipelines.ToUnconstrainedBinaryPipeline()]).run(model)
qubo = out.model                    # unconstrained, binary, minimization

# solve `qubo`, then map the solution back to the original model:
# solution = out.record.backward(qubo_solution)
```

See the [documentation](https://docs.aqarios.com/luna-model/intro) for bounded and integer
variables, solutions, all translators, and custom passes.

## Python modules

| Module                      | Description                                                                          |
| --------------------------- | ------------------------------------------------------------------------------------ |
| `luna_model`                | Models, variables, expressions, constraints, environments, and solutions.            |
| `luna_model.translator`     | Translators for LP, MPS, QUBO, BQM, CQM, Qiskit, and solution formats.               |
| `luna_model.transformation` | Pass manager, built-in passes and pipelines, and the base classes for custom passes. |
| `luna_model.utils`          | Helpers for building expressions and models, such as `quicksum`.                     |
| `luna_model.errors`         | All error types raised by LunaModel.                                                 |
| `luna_model.capi`           | Paths to the bundled C API headers and protobuf schemas for native extensions.       |

## Extensions

LunaModel exposes its core through a C API, so native code works with the same models,
expressions, and solutions as the Python package.

| Directory                                 | Description                                                                                      |
| ----------------------------------------- | ------------------------------------------------------------------------------------------------ |
| [`crates/capi-host`](./crates/capi-host/) | `liblunamodel`: the C API (`lm_*` functions over opaque handles) and its headers.                |
| [`cxx-lunamodel`](./cxx-lunamodel/)       | Header-only C++ wrapper: RAII handles, exceptions, and nanobind casters for Python extensions.   |
| [`rs-lunamodel`](./rs-lunamodel/)         | Rust SDK that mirrors the native Rust API over the C API, with pyo3 types for Python extensions. |
| [`proto`](./proto/)                       | Protobuf schemas of the built-in analysis results that plugins consume.                          |

Python extensions written in C, C++, or Rust build against an installed `luna-model`: the wheel
ships the C/C++ headers and the `.proto` schemas (`luna_model.capi.get_include()`,
`luna_model.capi.get_proto()`) and exports the C API to extensions at runtime, so an extension
does not link its own copy of the core. Standalone C and C++ programs link `liblunamodel`,
which `just c` builds into `dist/c`.

Transformation, analysis, meta-analysis, composite, and control-flow passes can be written in
Python, Rust, or C++ and combined in one `PassManager`. Register an extension's passes with
`luna_model.register(module)` before using them.

## Development

```bash
git clone https://github.com/aqarios/luna-model.git
cd luna-model
just doctor   # check the toolchain
just test     # run every test suite
```

[CONTRIBUTING.md](./CONTRIBUTING.md) lists the prerequisites, the checks for each part of the
repository, and the pull request process.

## Community

- [GitHub Discussions](https://github.com/aqarios/luna-model/discussions) is the place for bug
  reports, feature ideas, questions, and usage help. Maintainers triage discussions and turn
  confirmed, actionable items into issues.
- Bug fixes, documentation improvements, examples, and focused feature work are welcome. For
  anything beyond a small, obvious fix, agree on the direction in a discussion first; see
  [CONTRIBUTING.md](./CONTRIBUTING.md).

## License

LunaModel is licensed under the Apache License 2.0. See [LICENSE](./LICENSE).
