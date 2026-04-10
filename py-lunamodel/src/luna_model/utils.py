# Copyright 2026 Aqarios GmbH
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

from collections.abc import Iterable

from luna_model._lm import quicksum as q
from luna_model.expression.expr import Expression
from luna_model.variable.var import Variable


def quicksum(iterable: Iterable, start: Expression | Variable | None = None) -> Expression:
    """Efficiently sum an iterable of expressions, variables, and floats.

    This function provides an optimized way to sum multiple expressions or
    variables, which is more efficient than using repeated addition.

    Parameters
    ----------
    iterable : Iterable
        An iterable containing Expression, Variable, and/or float objects.
    start : Expression or Variable, optional
        Optional starting value for the sum.

    Returns
    -------
    Expression
        An expression representing the sum.

    Examples
    --------
    Sum a list of variables:

    >>> from luna_model import Environment, Variable
    >>> from luna_model.utils import quicksum
    >>> with Environment():
    ...     vars = [Variable(f"x{i}") for i in range(10)]
    >>> expr = quicksum(vars)
    >>> print(expr)
    x0 + x1 + x2 + x3 + x4 + x5 + x6 + x7 + x8 + x9

    Sum with coefficients:

    >>> coeffs = [1, 2, 3, 4, 5]
    >>> terms = [c * v for c, v in zip(coeffs, vars[:5])]
    >>> expr = quicksum(terms)
    >>> print(expr)
    x0 + 2 x1 + 3 x2 + 4 x3 + 5 x4

    Notes
    -----
    This is significantly faster than using ``sum()`` or repeated ``+`` operations
    for large numbers of terms.
    """
    return Expression._from_pyexpr(q(iterable, start))
