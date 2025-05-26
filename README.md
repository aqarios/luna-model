# 🧠 AqModels

> A high-performance symbolic modeling library for optimization, powered by Rust and exposed in Python.

`aqmodels` provides a fast, expressive, and extensible system for defining algebraic expressions, constraints, and symbolic models — with a focus on mathematical clarity and performance.

Built on a modern Rust core and exposed via `PyO3`, it bridges the performance of native code with the usability of Python.

---

## 🚀 Features

- ✅ Symbolic algebra with support for arbitrary polynomial degree
- ✅ Variable typing and bounding
- ✅ Constraint-based modeling
- ✅ Serialization & model portability
- ✅ Fast Rust backend with native bindings via [maturin](https://github.com/PyO3/maturin)
- ✅ Python-first development experience via [uv](https://astral.sh/blog/uv/)

---

## 📦 Installation

```bash
pip install git+https://github.com/aqarios/aq-models-rs.git@release
# or with ssh
pip install git+ssh://git@github.com/aqarios/aq-models-rs.git@release
```

Or clone and build locally (requires Rust):

```bash
git clone https://github.com/aqarios/aq-models-rs.git
cd aq-models-rs
uv sync
uv build
```

---

## ✨ Example

```python
from aqmodels import Variable, Model

model = Model("my_model")

with model.environment:
    x = Variable("x")
    y = Variable("y")

model.objective = x * y + 3 * x - 1
model.constraints += x >= 0
model.constraints += y <= 5

blob = model.encode()
new_model = Model.decode(blob)
```

---

## 📘 API Documentation

### 🧱 Core Classes

- **Vtype**: Variable type enum (Binary, Spin, Integer, Real)
- **Bounds**: Variable bounds
- **Variable**: Symbolic variable
- **Timing**, **Timer**: Timing utilities
- **Solution**: Solution container
- **SamplesIterator**, **SampleIterator**, **Samples**, **Sample**: Iterators and sample containers
- **ResultIterator**, **Result**, **ResultView**: Result containers
- **Sense**: Model sense (Minimize/Maximize)
- **Model**: Symbolic model container
- **Expression**: Symbolic algebraic expression
- **Environment**: Context manager for variable creation
- **Comparator**: Constraint comparator enum
- **Constraint**, **Constraints**: Constraint and constraint set

### 📐 Translators

All translators are available via `aqmodels.translator`:

- **ZibTranslator**: Convert between Zib (SCIP) and Solution
- **QuboTranslator**: Dense QUBO matrix ↔ symbolic Model
- **QctrlTranslator**: Qctrl dict ↔ Solution
- **NumpyTranslator**: Numpy arrays ↔ Solution
- **LpTranslator**: LP file/string ↔ Model
- **IbmTranslator**: IBM Qiskit result ↔ Solution
- **DwaveTranslator**: D-Wave SampleSet ↔ Solution
- **CqmTranslator**: dimod.ConstrainedQuadraticModel ↔ Model
- **BqmTranslator**: dimod.BinaryQuadraticModel ↔ Model
- **AwsTranslator**: AWS result dict ↔ Solution

#### Example: QuboTranslator

```python
from aqmodels import QuboTranslator
import numpy as np

qubo = np.array([[1.0, -1.0], [-1.0, 2.0]])
model = QuboTranslator.to_aq(qubo)
q_dense = QuboTranslator.from_aq(model)
```

#### Example: BqmTranslator

```python
from aqmodels.translator import BqmTranslator
import dimod

bqm = dimod.generators.gnm_random_bqm(5, 10, "BINARY")
model = BqmTranslator.to_aq(bqm)
bqm_back = BqmTranslator.from_aq(model)
```

---

### 🚨 Errors

All exceptions are exposed via `aqmodels.errors`:

- `VariableExistsError`, `NoActiveEnvironmentFoundError`
- `VariablesFromDifferentEnvsError`, `VariableOutOfRangeError`
- `ModelNotQuadraticError`, `ModelNotUnconstrainedError`
- `ModelSenseNotMinimizeError`, `ModelVtypeError`
- `DecodeError`, `MultipleActiveEnvironmentsError`
- ...and more

Use idiomatic try/except for symbolic safety:

```python
from aqmodels import Model, Variable
from aqmodels.errors import VariableExistsError

try:
    model = Model()
    with model.environment:
        x = Variable("x")
        x2 = Variable("x")  # raises VariableExistsError
except VariableExistsError as e:
    print("You already created this variable.")
```

---

## ⚙️ Internals

This library is powered by a high-performance Rust backend using:

- [PyO3](https://github.com/PyO3/pyo3) — For building native Python bindings
- [maturin](https://github.com/PyO3/maturin) — For packaging as a Python module
- [uv](https://astral.sh/blog/uv/) — For project and dependency management

The entire symbolic system is implemented in Rust for safety and speed, while Python exposes a clean, linter-friendly, IDE-friendly API via wrapper classes and `.pyi` stub generation.

---

## 🦀 Using the Rust Library Directly

AqModels is fundamentally a Rust library. All core modeling, algebra, and solution logic is implemented in Rust. The Python interface is a thin layer over this robust backend.

### Structure

- `src/core/`: Symbolic modeling, algebra, constraint, and solution logic.
- `src/serialization/`: Efficient serialization and deserialization.
- `src/translator/`: Translators for interoperability with other modeling libraries and formats.
- `src/errors.rs`: All error types and error handling.
- `src/prelude.rs`: Commonly used types and traits for ergonomic imports.

### Usage

To use AqModels as a Rust crate, add it to your `Cargo.toml` (when published):

```toml
[dependencies]
aqmodels = "0.1"
```

Or use a local path:

```toml
[dependencies]
aqmodels = { path = "../aq-models-rs" }
```

#### Example: Creating and Evaluating a Model in Rust

> [!CAUTION]
> The following code is provided as an illustrative example and may require adjustments to compile and run successfully in your environment.

```rust
// filepath: README.md (Rust section)
use aqmodels::core::{Model, Vtype, Sense};
use aqmodels::core::constraints::{Constraint, Comparator};
use aqmodels::core::solution::{Sample, ResultIterator, ConcreteBias, ConcreteAssignmentTypes};

fn main() {
    // Create a new model
    let mut model = Model::new(Some("my_model".to_string()));

    // Add variables to the environment
    let mut env = model.environment.borrow_mut();
    let x = env.add_variable("x".to_string(), Some(&Vtype::Real), None);
    let y = env.add_variable("y".to_string(), Some(&Vtype::Real), None);
    drop(env); // Release the mutable borrow

    // Directly build the objective using variable operations
    // x * y + 3 * x - 1
    let mut objective = &x * &y; // x * y
    objective += &x * 3.0;       // + 3 * x
    objective += -1.0;           // - 1

    *model.objective.borrow_mut() = objective;
    model.set_sense(Sense::Min);

    // Add constraints: x >= 0, y <= 5
    let constraint1 = Constraint::new((&x).into(), Comparator::Ge, 0.0);
    let constraint2 = Constraint::new((&y).into(), Comparator::Le, 5.0);
    model.constraints.borrow_mut().push(constraint1);
    model.constraints.borrow_mut().push(constraint2);

    // Example: Evaluate the objective for a sample assignment
    let sample_vec = vec![1.0, 2.0]; // x=1.0, y=2.0
    let sample = Sample::<ConcreteBias, ConcreteAssignmentTypes>::from_vec(sample_vec);

    let result = model.evaluate_sample(&sample);
    println!("Objective value: {:?}", result.obj_value());

    // Working with solutions and results
    let solution = result.solution();
    let mut results = ResultIterator::new(solution);
    while let Some(res_view) = results.next() {
        println!(
            "Sample: {:?}, Objective: {:?}, Feasible: {:?}",
            res_view.get_sample(),
            res_view.obj_value(),
            res_view.feasible()
        );
    }
}
```

#### Features

- **No Python Required:** Use the full modeling and algebraic API in Rust.
- **Performance:** All operations are zero-cost abstractions and use Rust's safety guarantees.
- **Interoperability:** Use the same serialization and translation features as in Python.

#### Documentation

- Rustdoc is available for all public modules and types.
- See [`src/core/`](src/core/) and [`src/prelude.rs`](src/prelude.rs) for main entry points.

---

## 👷 Development

```bash
uv dev
```

- Make changes in Rust (`src/lib.rs`, etc.)
- Python interface lives in `src/aqmodels/`
- Auto-generate `.pyi` and `__init__.py` files during:

```bash
uv sync
```

---

## 📄 License

_**TBD**_

---

## 🧠 Contributing

We welcome contributions, fixes, and features!  
For roadmap ideas, check out [issues](https://github.com/your-org/aqmodels/issues).

---

## 🌌 Related Projects

- [`rustworkx`](https://github.com/Qiskit/rustworkx) — Rust-powered graph library for Python
- [`maturin`](https://github.com/PyO3/maturin) — Painless Rust + Python packaging

---

> © Aqarios GmbH 2024-present. Built with ❤️ and Rust.