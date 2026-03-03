<div align="center">
  <img src="./assets/luna_model_logo.svg" alt="lunaᴹᴼᴰᴱᴸ" width="400">
</div>

# Symbolic modeling for optimization

[![ci](https://github.com/aqarios/luna-model/actions/workflows/ci.yml/badge.svg)](https://github.com/aqarios/luna-model/actions/workflows/ci.yml)
[![docs](https://github.com/aqarios/luna-model/actions/workflows/pages.yml/badge.svg)](https://github.com/aqarios/luna-model/actions/workflows/pages.yml)

[**About**](#about-LunaModel)
| [**Installation**](#installation)
| [**Getting Started**](#getting-started)
| [**Resources**](#resources)
| [**Releases and Contributing**](#releases-and-contributing)

## Summary

LunaModel is a high-performance symbolic modeling library for describing, translating and transforming optimization problems.
It provides the following high-level features:
- System for defining symbolic algebraic expressions of arbitrary degree, constraints and optimization models (like dimod, gurobi or cplex)
- Translations from and to an LunaModel for many common optimization model formats (like LP)
- Transformations to map an LunaModel from a general model to a specific model, such as transforming a Constrained (Binary) Quadratic Model (CQM) to a (Unconstrained) Binary Quadratic Model (BQM), or from an Integer Model to a Binary Model.
- Builtin serialization for maximum portability
- Python-first development experience

You can use LunaModel as a standalone package or by using [luna-quantum](https://pypi.org/project/luna-quantum/) which gives you additional builtin primitives to solve your optimization problems using the [Luna Platform](https://aqarios.com/platform).

<!-- toc -->

- [About LunaModel](#about-luna_model)
- [Installation](#installation)
  - [Binaries](#binaries)
  - [From Source](#from-source)
    - [Prerequisites](#prerequisites)
  - [Get the LunaModel Source](#get-the-luna_model-source)
  - [Install Dependencies](#install-dependencies)
  - [Install LunaModel](#install-luna_model)
    - [Adjust Build Options (Optional)](#adjust-build-options-optional)
  - [Building the Documentation (Python)](#building-the-documentation-python)
  - [Building the Documentation (Rust)](#building-the-documentation-rust)
- [Getting Started](#getting-started)
- [Resources](#resources)
- [Communication](#communication)
- [Releases and Contributing](#releases-and-contributing)

<!-- tocstop -->

## About LunaModel

Most optimization tasks involve working with problems, which generally consist of an objective function,
wether this objective function should be minimized or maximized and optionally constraints to the problem itself.
You can learn more about using LunaModel in [this tutorial](./examples/basics.md) where we take a deeper dive into
a complete optimization workflow implemented using LunaModel and [LunaSolve](https://pypi.org/project/luna-quantum/).

LunaModel consists of the following components:

| Component                                                 | Description                                                               |
| --------------------------------------------------------- | ------------------------------------------------------------------------- |
| [**LunaModel**](#a-symbolic-modeling-library)              | A symbolic modeling library for arbitrary optimization models (problems). |
| [**LunaModel.translator**](#the-translation-library)       | An automatic translation library that supports most common model format.  |
| [**LunaModel.transformations**](#the-transformation-stack) | A compilation and transpilation stack to transform a model (source) into a target representation (target). _Not all targets are reachable from all sources, for more information on why see detailed in the resepective section._ |
| [**LunaModel.utils**](./src/luna_model/translator.pyi)       | Utility functions for expression and model creation.                      |
| [**LunaModel.errors**](./src/luna_model/errors.pyi)          | All error types that can be raised within LunaModel.                       |

LunaModel is usually used as either:
- A replacement for plain LP files, dimod or similar frameworks to define optimization models.
- As part of [luna-quantum](https://pypi.org/project/luna-quantum/) to solve arbitrary optimization problems.

### A Symbolic Modeling Library

> [!IMPORTANT]
> The following assumes you are using from Python. If you are interested in using it from Rust look at the [**Documentation**](#building-the-documentation-rust).

With LunaModel you can define symbolic Expressions and Constraints (_which in consist of left-hand side (lhs), an Expression, a right-hand side (rhs) which is a constant numerical value and a Comparator_).
A Model defining arbitrary optimization problems consists of a single Expression as the objective function (_the function to be optimized_) and, optionally, one or more Constraints.
Expressions are created using mathematical operations on Variables. Variables represent an unknown in the Expression which is determined by an optimization. By default variables are Binary, can represent any of the following Variable types:

- **Binary**: the variable can be either $0$ or $1$.
- **Spin**: the variable can be either $-1$ or $+1$.
- **Integer**: the variable can be any integer number $\in [-2^{64}-1, 2^{64}-1]$ (_for a 64-Bit system_).
- **Real**: the variable can be any floating point number $\in [\approx -1.7976...E308, \approx +1.7976...E308]$ (_[-f64::MAX, f64::MAX]_).

_In general not all variable types are supported by all optimizers you can find. It can be the case that a defined model cannot be natively translated into the expected format of an optimizer. To resolve this you can use [**LunaModel.transformations**]()._

Let's have a look a the **Knapsack Problem** for defining an optimization problem using only Binary variables.
We have $n$ items $x_1, x_2, \dots, x_n$, each with a weight $w_i$ and a value $v_i$, and a maximum capacity of $W$.
The optimization problem is defined as:

```math
\begin{align*}
&\text{maximize} \sum_{i=1}^{n} v_i x_i \\
&\text{subject to} \sum_{i=1}^{n} w_i x_i \leq W \quad \text{and} \quad x_i \in \{ 0, 1 \}
\end{align*}
```

Using LunaModel and $n = 5$ and $W = 25$:

```python
from luna_model import Expression, Model, Sense, Vtype
# A faster alternative to creating Expressions using loops in Python.
from luna_model.utils import quicksum
# Initialize the known values:
n: int = 5  # number of items.
W: int = 25 # maximum capacity.
weights: list[float] = [ 1.5, 10.0, 5.2,  3.5, 8.32] # weight of each item.
values:  list[float] = [10.0, 22.0, 3.2, 1.99, 6.25] # value of each item.
# First, we create the Model with it's sense set to Maximize the objective function.
# You can also give your model a name, optionally but recommended.
model = Model(sense=Sense.Max, name="Knapsack")
# Next, we need to create all variables. Note, there are alternative ways to create
# variables, you can find details in the LunaModel docs.
variables = [model.add_variable(f"x_{i+1}", vtype=Vtype.BINARY) for i in range(n)]
# Now we can define the objective function:
model.objective = quicksum(values[i] * variables[i] for i in range(n))
# And for the constraints:
# Ensure the maximum capacity of `W`:
model.constraints += quicksum(weights[i] * variables[i] for i in range(n)) <= W
# The second constraint that all `x_i` are in [0, 1] is natively encoded by using
# Binary variables.
```

As an extension, the **Bounded Knapsack Problem (BKP)** with a maximum number of each item $c = 4$ can be defined like this:

```math
\begin{align*}
&\text{maximize} \sum_{i=1}^{n} v_i x_i \\
&\text{subject to} \sum_{i=1}^{n} w_i x_i \leq W \quad \text{and} \quad x_i \in \{ 0, 1, 2, \dots, c \}
\end{align*}
```

Now we have two equivalent approaches to implement this using LunaModel:
_Note that we have to use Integer variables now._

- Using Bounds on the variables:
  ```python
  from luna_model import Expression, Model, Sense, Vtype, Bounds
  # A faster alternative to creating Expressions using loops in Python.
  from luna_model.utils import quicksum
  # Initialize the known values:
  c: int = 4  # maximum number of each item.
  n: int = 5  # number of items.
  W: int = 25 # maximum capacity.
  weights: list[float] = [ 1.5, 10.0, 5.2,  3.5, 8.32] # weight of each item.
  values:  list[float] = [10.0, 22.0, 3.2, 1.99, 6.25] # value of each item.
  # First, we create the Model with it's sense set to Maximize the objective function.
  # You can also give your model a name, optionally but recommended.
  model = Model(sense=Sense.Max, name="Bounded Knapsack")
  # Next, we need to create all variables. Note, there are alternative ways to create
  # variables, you can find details in the LunaModel docs.
  variables = [
      # We can have each item at least `0` times and at most `c` times.
      model.add_variable(f"x_{i+1}", vtype=Vtype.INTEGER, lower=0, upper=c)
      for i in range(n)
  ]
  # Now we can define the objective function:
  model.objective = quicksum(values[i] * variables[i] for i in range(n))
  # And for the constraints:
  # Ensure the maximum capacity of `W`:
  model.constraints += quicksum(weights[i] * variables[i] for i in range(n)) <= W
  # The second constraint that all `x_i` are in [0, 1, 2, ..., c] is natively encoded
  # by using Bounds on the Integer variables.
  ```
- Using a Constraint for each variable:
  ```python
  from luna_model import Expression, Model, Sense, Vtype, Bounds
  # A faster alternative to creating Expressions using loops in Python.
  from luna_model.utils import quicksum
  # Initialize the known values:
  c: int = 4  # maximum number of each item.
  n: int = 5  # number of items.
  W: int = 25 # maximum capacity.
  weights: list[float] = [ 1.5, 10.0, 5.2,  3.5, 8.32] # weight of each item.
  values:  list[float] = [10.0, 22.0, 3.2, 1.99, 6.25] # value of each item.
  # First, we create the Model with it's sense set to Maximize the objective function.
  # You can also give your model a name, optionally but recommended.
  model = Model(sense=Sense.Max, name="Bounded Knapsack")
  # Next, we need to create all variables. Note, there are alternative ways to create
  # variables, you can find details in the LunaModel docs.
  variables = [
      model.add_variable(f"x_{i+1}", vtype=Vtype.INTEGER)
      for i in range(n)
  ]
  # Now we can define the objective function:
  model.objective = quicksum(values[i] * variables[i] for i in range(n))
  # And for the constraints:
  # Ensure the maximum capacity of `W`:
  model.constraints += quicksum(weights[i] * variables[i] for i in range(n)) <= W
  # The second constraint that all `x_i` are in [0, 1, 2, ..., c]:
  for i in range(n):
      model.constraints += variables[i] <= c
      model.constraints += variables[i] >= 0
  ```

LunaModel also provides a native Solution class that represents solutions to an LunaModel natively.

### The Translation Library

LunaModel has builtin translators to convert from most model representations to an LunaModel.
To use optimizers not natively supporting LunaModel you can use one of the provided translators to translate from luna_model to any of the supported formats. It also contains translators for converting from an optimizer native solution representation to a LunaModel Solution.

LunaModel ships with the following [**luna_model.translators**](./src/luna_model/translator.pyi) for Models:

- **QuboTranslator**: To translate from and to a [Quadratic Unconstrained Binary Optimization (QUBO)](https://en.wikipedia.org/wiki/Quadratic_unconstrained_binary_optimization) problem.
- **LpTranslator**: To translate from and to a [LP File](https://web.mit.edu/lpsolve/doc/CPLEX-format.htm).
- **BqmTranslator**: To translate from and to a [Dimod BQM](https://docs.dwavequantum.com/en/latest/ocean/api_ref_dimod/models.html#module-dimod.binary.binary_quadratic_model).
- **CqmTranslator**: To translate from and to a [Dimod CQM](https://docs.dwavequantum.com/en/latest/ocean/api_ref_dimod/models.html#module-dimod.constrained.constrained).

LunaModel ships with the following [**luna_model.translators**](./src/luna_model/translator.pyi) for Solutions:

- **ZibTranslator**: for converting a [SCIP](https://www.scipopt.org/) result.
- **QctrlTranslator**: for converting a [Q-Ctrl](https://q-ctrl.com/) result.
- **NumpyTranslator**: for converting a result formatted as a [Numpy](https://numpy.org/) array.
- **IbmTranslator**: for converting a [Qiskit](https://www.ibm.com/quantum/qiskit) result.
- **DwaveTranslator**: for converting a [DWave](https://www.dwavequantum.com/) result.
- **AwsTranslator**: for converting an [Amazon Braket](https://aws.amazon.com/braket/) result.

### The Transformation Stack

> [!WARNING]
> The transformation stack is still a work in progress and does not have a mostly finalized API yet.
> A description will follow once the API is stabilized.

## Installation

### Binaries

You have multiple options for obtaining pre-built binaries:
- Using LunaModel as a builtin of the [luna-quantum](https://pypi.org/project/luna-quantum) package.
- Obtianing a pre-built wheel from the [GitHub releases page](https://github.com/aqarios/luna-model/releases).
- Installing it from the Aqarios private artifact feed. **This is only available for Aqarios Employees.**
  - The package can be installed using `luna-model`.
  - The package distribution is named `luna-model`.
  - They are both imported as `luna_model`, e.g., `import luna-model` or `from luna_model import ...`

> [!TIP]
> If you are installing a package that uses the **public** luna-model, but want to use the **private** luna-model,
> you can simply install it as an additional dependency. You can check if the **priavate** LunaModel is installed using
> the `luna_model.__version__`, which should reoslve to a version string containing `pub`. If it does not work out-of-the-box,
> try adding:
> ```text
[tool.uv]
override-dependencies = [ "luna-model; sys_platform == 'never'"]
```
> or (if now available in uv ([See if the code of this PR is released](https://https://github.com/astral-sh/uv/pull/16528)))
```text
[tool.uv]
exclude-dependencies = [ "luna-model" ]
```
> Also remove the current `.venv` and run `uv clean` in case you still encounter issues.
> If none of the mentioned tips resolves you problem, open a [discussion here](https://github.com/aqarios/luna-model/discussions/categories/q-a).

### From Source

The following assumes you are on Linux or MacOS. Windows is supported using pre-built [**Binaries**](#binaries).
While you can install it from Source on Windows, we will not provide explicit installation instructions.

#### Prerequisites

If you are installing from source, you will need:

- The [**uv**](https://docs.astral.sh/uv/) Python package and project manager.
- [**Rust & Cargo**](https://rust-lang.org/learn/get-started/) for the Rust compiler and the Rust build tool and package manager.

##### Get the LunaModel Source

```bash
git clone https://github.com/aqarios/luna-model
cd luna-model
# If you are updating an existing checkout
git pull
```

##### Install Dependencies

###### For Development

```bash
uv sync
# Optionally activate your environment if you don't want to use `uv run ...`
source .venv/bin/activate
```

This installs LunaModel and all dev dependencies into the `.venv`.

###### For Usage Only

```bash
uv sync --no-dev
# Optionally activate your environment if you don't want to use `uv run ...`
source .venv/bin/activate
```

This installs only LunaModel and runtime dependencies into the `.venv`.

### Build and Install LunaModel

#### For Python

```bash
uv build
```

To build for a specific python version, e.g., Python 3.14:

```bash
uv build --python 3.14
```

This will put the LunaModel binary into `./target/wheels/`. To install it in another project
use your favorite Python package manager and install it using the wheels path, e.g., with
pip:

```bash
pip install <path>/<to>/target/wheels/LunaModel-<version>-cp314-cp314-<platform>.whl
```

#### For Rust

```bash
cargo build --release
```

To compile LunaModel with default features (for rust usage only).

#### Adjust Build Options (Optional)

You have the following options for compiling the LunaModel Rust source code. By default
no feature is enabled.

- **transformations**: To compile with transformations.
- **py**: To compile with Python bindings.
- **pyt**: Extension to **py** with transformations and their Python bindings. _Does not produce the same code as using the **py** and **transformations** feature_
- **lq**: To compile the **pyt** feature with the namespace changed to `luna_quantum` for all Python imports.

### Building the Documentation (Python)

Currently not supported, you can fine the python documentation for the latest release online [here](https://docs.aqarios.com) as part of the `luna_quantum` documentation.

> [!TIP]
> The online documentation might not be up-to-date or complete. To get an exhaustive documentation of the Python API see [Building the Documentation (Rust)](#building-the-documentation-rust) and look at the dcoumentation of the LunaModel Python Bindings (_`py_bindings`_).

### Building the Documentation (Rust)

Similar to building the library you can pass the `--features <feature>` argument to generate the docs for the objects included with `<feature>`.
To open the docs in a browser after building add the `--open` flag.

```bash
cargo doc --no-deps --document-private-items
```

To also generate the docs for the `transformations` run:

```bash
cargo doc --no-deps --document-private-items --features transformations
```

To also generate the docs for the `py_bindings` run:

```bash
cargo doc --no-deps --document-private-items --features py
```

To also generate the docs for the `py_bindings` (including transformations) run:

```bash
cargo doc --no-deps --document-private-items --features pyt
```

## Getting Started

Have a look at:
- The [Get Started](https://docs.aqarios.com/get-started/) for [luna-quantum](https://pypi.org/project/luna-quantum).
- The [Examples](./examples) contained in this repository.

## Resources

- [luna-quantum documentation](https://docs.aqarios.com)
- [LunaModel Discussions](https://github.com/aqarios/aq-models-rs/discussions)

## Communication

- GitHub Discussions: Talk about anything LunaModel related (thoughts, usage issues that are neither a Bug or a Use Case, cool examples, ...) [here](https://github.com/aqarios/aq-models-rs/discussions)
- GitHub Issues: Bug Reports, Proposals, Use Cases, etc.

## Releases and Contributing

Typically, LunaModel releases new features as soon as the core Maintainers decide that they are stable enough to be made available publicly.
Bug-fixes are released as soon as the fix is accepted by the core Maintainers. Please let us know if you encounter a bug by [filing a bug report](https://github.com/aqarios/aq-models-rs/issues/new?template=bug.yml).

We appreciate all contribtions. If you plan to contribute back bug-fixes, please do so without any further discussion, see the [Contribution page](./CONTRIBUTING.md) for details. Once done open a PR and assign one of the core Maintainers as a Reviewer.

If you plan to contribute back new features, or extensions, that are not yet mentioned in the Issues, please [file a proposal](https://github.com/aqarios/aq-models-rs/issues/new?template=proposal.yml) and discuss your idea with us once we accepted your proposal (proposal also has the "accepted" label) you can start your implementation and open a PR once deemed ready to be merged. Opening a PR not associated with an "accepted" proposal might end up being rejected because we might be taking LunaModel in a different direction than you might be aware of.

If you have a particular use case where it's either extremely hard or impossible to use LunaModel but you don't propose a solution, please [file a use case](https://github.com/aqarios/aq-models-rs/issues/new?template=usecase.yml) and discuss the use case with us.
It might be transformed into a proposal once enough information is gathered to have an actionable plan on resolving you issue.

Do not assign or mention a core Maintainer yourself. We regularly check the Issues page and work through proposals depending on the direction we are currently focusing on.

For further information about making a contribution and how releases are made, please see our [Contribution guidelines](./CONTRIBUTING.md).
