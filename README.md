# 🧠 AqModels

> A high-performance symbolic modeling library for optimization, powered by Rust and exposed in Python.

`aqmodels` provides a fast, expressive, and extensible system for defining algebraic expressions, constraints, and symbolic models — with a focus on mathematical clarity and performance.

Built on a modern Rust core and exposed via `PyO3`, it bridges the performance of native code with the usability of Python.

---

## 🚀 Features

- ✅ Symbolic algebra with support for arbitrary polynomial degree
- ✅ Variable typing and bounding
- ✅ Constraint-based modeling
- ✅ Dense matrix ↔ symbolic conversion
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

<details>
<summary><strong>🧱 Core Classes</strong></summary>

### `Variable`
Represents a typed symbolic variable inside an environment.

```python
x = Variable("x")
```

- Supports `+`, `-`, `*` with scalars, variables, expressions
- Bounds, types, and environments
- 🔥 Raises `VariableExistsError`, `NoActiveEnvironmentFoundError`

### `Expression`
Symbolic polynomial expression: linear, quadratic, and higher-order.

```python
expr = 2 * x + x * y + 1
```

- Algebraic operations return new expressions
- `.get_linear(x)`, `.get_quadratic(x, y)` introspect structure
- Can generate constraints via `==`, `<=`, `>=`
- 🔥 Raises `VariablesFromDifferentEnvsError`, `VariableOutOfRangeError`

### `Constraint`, `Constraints`
Encapsulates symbolic constraints.

```python
model.constraints += x * x <= 3.0
```

- Manual construction also supported
- Symbolic only — no solving performed
- 🔥 `+=` can raise `RuntimeError` for invalid inputs

### `Environment`
Context manager for variable creation.

```python
with Environment():
    x = Variable("x")
```

- Automatically handles scoping and indexing
- 🔥 Only one active environment at a time

### `Model`
Full symbolic model container.

```python
model = Model()
model.objective = expr
model.constraints += expr >= 2.0
```

- Supports serialization via `.encode()` / `.decode()`
- Quadratic/unconstrained models can be exported as QUBOs
- 🔥 Can raise `ModelNotQuadraticError`, `ModelNotUnconstrainedError`

</details>

<details>
<summary><strong>📐 Translators</strong></summary>

### `MatrixTranslator`

```python
from aqmodels import MatrixTranslator

qubo = np.array([[1.0, -1.0], [-1.0, 2.0]])
model = MatrixTranslator.to_model(qubo)
q_dense = MatrixTranslator.to_dense(model)
```

- Bridges matrix-style QUBO ↔ symbolic `Model`
- 🔥 Fails on constrained or higher-order models
</details>

<details>
<summary><strong>🚨 Errors</strong></summary>

All exceptions are exposed via `aqmodels.errors`.

- `VariableExistsError`, `NoActiveEnvironmentFoundError`
- `VariablesFromDifferentEnvsError`, `VariableOutOfRangeError`
- `ModelNotQuadraticError`, `ModelNotUnconstrainedError`
- `DecodeError`, `MultipleActiveEnvironmentsError`, ...

Use idiomatic try/except for symbolic safety:

```python
try:
    model = Model()
    with model.environment:
        x = Variable("x")
except VariableExistsError as e:
    print("You already created this variable.")
```
</details>

---

## ⚙️ Internals

This library is powered by a high-performance Rust backend using:

- [PyO3](https://github.com/PyO3/pyo3) — For building native Python bindings
- [maturin](https://github.com/PyO3/maturin) — For packaging as a Python module
- [uv](https://astral.sh/blog/uv/) — For project and dependency management

The entire symbolic system is implemented in Rust for safety and speed, while Python exposes a clean, linter-friendly, IDE-friendly API via wrapper classes and `.pyi` stub generation.

📄 Internal structure documentation is coming soon!

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

> © Aqarios GmbH / Jonas Blenninger 2024-present. Built with ❤️ and Rust.
