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
from __future__ import annotations

from typing import TYPE_CHECKING

from luna_model._lm import PyConstraint
from luna_model._utils import wrap_expr
from luna_model.constraint.cmp import Comparator

if TYPE_CHECKING:
    from luna_model.expression.expr import Expression
    from luna_model.variable.var import Variable


class Constraint:
    """Constraint relating expressions with comparison operators and right-hand sides.

    A constraint specifies a relationship between a left-hand side expression
    and a right-hand side value using a comparison operator (``==``, ``<=``, ``>=``).
    Constraints are typically created using comparison operators on expressions
    or variables.

    Parameters
    ----------
    lhs : Variable or Expression
        The left-hand side of the constraint.
    rhs : float or Expression or Variable
        The right-hand side of the constraint.
    comparator : Comparator
        The comparison operator (``EQ``, ``LE``, or ``GE``).
    name : str, optional
        An optional name for the constraint for easier identification.

    Attributes
    ----------
    name : str
        The name of the constraint.
    lhs : Expression
        The left-hand side expression (constant is moved to rhs).
    rhs : float
        The right-hand side value (expressions and variables are moved to lhs).
    comparator : Comparator
        The comparison operator.

    Examples
    --------
    Create constraints using comparison operators (_auto-assigned name_):

    >>> from luna_model import Variable, Environment
    >>> with Environment():
    ...     x = Variable("x")
    ...     y = Variable("y")
    >>> c1 = x + y <= 10
    >>> c2 = 2 * x - y == 5
    >>> c3 = x >= 0

    Create named constraint:

    >>> from luna_model.constraint import Constraint, Comparator
    >>> with Environment():
    ...     x = Variable("x")
    ...     y = Variable("y")
    >>> constraint = Constraint(x + y, 10, Comparator.LE, name="capacity")

    Notes
    -----
    The right-hand side is always normalized to a constant. If an expression
    or variable is provided as the right-hand side, it is moved to the left-hand side.
    If the left-hand side contains a non-zero constant value it is moved to
    the right-hand side.
    """

    _c: PyConstraint

    def __init__(
        self,
        lhs: Variable | Expression,
        rhs: float | Expression | Variable,
        comparator: Comparator,
        name: str | None = None,
    ) -> None:
        self._c = PyConstraint(lhs, rhs, comparator._val, name)

    @classmethod
    def _from_pyc(cls, py_c: PyConstraint) -> Constraint:
        """Construct Constraint from FFI PyConstraint object."""
        c = cls.__new__(cls)
        c._c = py_c
        return c

    @property
    def name(self) -> str:
        """Get the constraint's name.

        Returns
        -------
        str
            The constraint name.
        """
        return self._c.name

    @name.setter
    def name(self, name: str) -> None:
        """Set the constraint's name.

        Parameters
        ----------
        name : str
            The new name for the constraint.
        """
        self._c.name = name

    @property
    def lhs(self) -> Expression:
        """Get the left-hand side expression.

        Returns
        -------
        Expression
            The left-hand side expression.
        """
        return wrap_expr(self._c.lhs)

    @property
    def rhs(self) -> float:
        """Get the right-hand side value.

        Returns
        -------
        float
            The right-hand side constant value.
        """
        return self._c.rhs

    @property
    def comparator(self) -> Comparator:
        """Get the comparison operator.

        Returns
        -------
        Comparator
            The comparison operator (``EQ``, ``LE``, or ``GE``).
        """
        return Comparator._from_pycmp(self._c.comparator)

    def equal_contents(self, other: Constraint) -> bool:
        """Check if two constraints have equal content.

        Parameters
        ----------
        other : Constraint
            The constraint to compare with.

        Returns
        -------
        bool
            True if constraints have the same ``rhs``, and ``comparator`` and
            the ``lhs`` has the same contents.
        """
        return self._c.equal_contents(other)

    def __eq__(self, other: Constraint) -> bool:  # type: ignore[override]
        """Check if two constraints are exactly equal.

        Parameters
        ----------
        other : Constraint
            The constraint to compare with.

        Returns
        -------
        bool
            True if constraints are structurally identical.
        """
        return self._c.__eq__(other)

    def __str__(self) -> str:
        """Return human-readable string representation.

        Returns
        -------
        str
            String representation of the constraint.
        """
        return self._c.__str__()

    def __repr__(self) -> str:
        """Return detailed string representation.

        Returns
        -------
        str
            Detailed representation of the constraint.
        """
        return self._c.__repr__()
