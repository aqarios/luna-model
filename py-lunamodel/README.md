# Symbolic modeling for optimization

[**About**](#about-LunaModel)
| [**Getting Started**](#getting-started)
| [**Resources**](#resources)

## Summary

LunaModel is a high-performance symbolic modeling library for describing, translating and transforming optimization problems.
It provides the following high-level features:
- System for defining symbolic algebraic expressions of arbitrary degree, constraints and optimization models (like dimod, gurobi or cplex)
- Translations from and to an LunaModel for many common optimization model formats (like LP)
- Transformations to map an LunaModel from a general model to a specific model, such as transforming a Constrained (Binary) Quadratic Model (CQM) to a (Unconstrained) Binary Quadratic Model (BQM), or from an Integer Model to a Binary Model.
- Builtin serialization for maximum portability
- Python-first development experience

You can use LunaModel as a standalone package or by using [luna-quantum](https://pypi.org/project/luna-quantum/) which gives you additional builtin functionality to solve your optimization problems using the [Luna Platform](https://aqarios.com/platform).

<!-- toc -->

- [About LunaModel](#about-luna_model)
- [Getting Started](#getting-started)
- [Resources](#resources)

<!-- tocstop -->

## About LunaModel

Most optimization tasks involve working with problems, which generally consist of an objective function,
wether this objective function should be minimized or maximized and optionally constraints to the problem itself.

LunaModel consists of the following components:

| Component                    | Description                                                               |
| ---------------------------- | ------------------------------------------------------------------------- |
| **LunaModel**                | A symbolic modeling library for arbitrary optimization models (problems). |
| **LunaModel.translator**     | A translation library that supports many common model formats.            |
| **LunaModel.transformation** | A compilation and transpilation stack to transform a model (source) into a target representation (target). _Not all targets are reachable from all sources, for more information on why see detailed in the resepective section._ |
| **LunaModel.utils**          | Utility functions for expression and model creation.                      |
| **LunaModel.errors**         | All error types that can be raised within LunaModel.                      |

LunaModel is usually used as either:
- A replacement for plain LP files, dimod or similar frameworks to define optimization models.
- As part of [luna-quantum](https://pypi.org/project/luna-quantum/) to solve arbitrary optimization problems.

### A Symbolic Modeling Library

With LunaModel you can define symbolic Expressions and Constraints (_which in consist of left-hand side (lhs), an Expression, a right-hand side (rhs) which is a constant numerical value and a Comparator_).
A Model defining arbitrary optimization problems consists of a single Expression as the objective function (_the function to be optimized_) and, optionally, one or more Constraints.
Expressions are created using mathematical operations on Variables. Variables represent an unknown in the Expression which is determined by an optimization. By default variables are Binary, can represent any of the following Variable types:

- **Binary**: the variable can be either $0$ or $1$.
- **Spin**: the variable can be either $-1$ or $+1$.
- **Integer**: the variable can be any integer number $\in [-2^{64}-1, 2^{64}-1]$ (_for a 64-Bit system_).
- **Real**: the variable can be any floating point number $\in [\approx -1.7976...E308, \approx +1.7976...E308]$ (_[-f64::MAX, f64::MAX]_).

_In general not all variable types are supported by all optimizers you can find. It can be the case that a defined model cannot be natively translated into the expected format of an optimizer. To resolve this you can use **LunaModel.transformation**._

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
model = Model(sense=Sense.MAX, name="Knapsack")
# Next, we need to create all variables. Note, there are alternative ways to create
# variables, you can find details in the LunaModel docs.
variables = [model.add_variable(f"x_{i+1}", vtype=Vtype.Binary) for i in range(n)]
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
  model = Model(sense=Sense.MAX, name="Bounded Knapsack")
  # Next, we need to create all variables. Note, there are alternative ways to create
  # variables, you can find details in the LunaModel docs.
  variables = [
      # We can have each item at least `0` times and at most `c` times.
      model.add_variable(f"x_{i+1}", vtype=Vtype.Integer, lower=0, upper=c)
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
  model = Model(sense=Sense.MAX, name="Bounded Knapsack")
  # Next, we need to create all variables. Note, there are alternative ways to create
  # variables, you can find details in the LunaModel docs.
  variables = [
      model.add_variable(f"x_{i+1}", vtype=Vtype.Integer)
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

## Getting Started

For more information and help in getting started with LunaModel see our [Getting Started Guide](https://docs.aqarios.com/luna-model/getting-started) and the [documentation](https://docs.aqarios.com/luna-model).

## Resources

- [LunaModel](https://pypi.org/project/luna-model/)
- [LunaModel Documentation](https://docs.aqarios.com/luna-model)

- [LunaQuantum](https://pypi.org/project/luna-quantum/)
- [LunaQuantum Documentation](https://docs.aqarios.com/luna-quantum)
