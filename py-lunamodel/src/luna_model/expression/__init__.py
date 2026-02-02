"""Mathematical expressions for optimization models.

This module provides classes for representing and manipulating mathematical
expressions composed of variables, constants, and arithmetic operations.
Expressions are used to define objective functions and constraints.

Key classes:
    - Expression: Main class for mathematical expressions
    - Linear, Quadratic, HigherOrder: Term types in expressions
    - ExprIter: Iterator for traversing expression terms
"""

from luna_model.expression.expr import Expression
from luna_model.expression.iter import (
    Constant,
    ExprIter,
    HigherOrder,
    Linear,
    Quadratic,
)

__all__ = [
    "Constant",
    "ExprIter",
    "Expression",
    "HigherOrder",
    "Linear",
    "Quadratic",
]
