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

from typing import TYPE_CHECKING, NamedTuple

from luna_model._lm import PyConstraint
from luna_model._utils import wrap_expr, wrap_var
from luna_model.constraint.cmp import Comparator

if TYPE_CHECKING:
    from luna_model.expression.expr import Expression
    from luna_model.variable.var import Variable


class Indicator(NamedTuple):
    """A constraint that only has to hold for one value of a binary variable.

    Reads as ``variable == value`` implies ``constraint``. Returned by
    :meth:`Constraint.indicator`.

    Attributes
    ----------
    variable : Variable
        The binary variable the constraint depends on, never an inverted one.
    value : int
        The value of ``variable``, 0 or 1, for which ``constraint`` has to hold.
    constraint : Constraint
        The constraint that has to hold while ``variable`` equals ``value``.
    """

    variable: Variable
    value: int
    constraint: Constraint


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
        if py_c is NotImplemented:
            # Comparison bindings return NotImplemented for unsupported operands;
            # pass it on so Python raises TypeError instead of wrapping it.
            return NotImplemented
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

    def when(self, var: Variable, value: int = 1) -> Constraint:
        """State this constraint only for one value of a binary variable.

        ``c.when(z)`` has to hold if ``z`` is 1 and ``c.when(z, value=0)`` if
        ``z`` is 0; for the other value of ``z`` it is satisfied whatever the
        other variables are. The result is an ordinary constraint: for ``z`` being
        1, ``lhs <= rhs`` becomes ``z * (lhs - rhs) <= 0``. It keeps the name of
        this constraint. :meth:`indicator` reads it back.

        Parameters
        ----------
        var : Variable
            The binary variable to depend on. An inverted variable (``z.inv()``)
            stands for the other value of ``z``, so ``c.when(z.inv())`` is
            ``c.when(z, value=0)``.
        value : int, default=1
            The value of ``var``, 0 or 1, for which the constraint has to hold.

        Returns
        -------
        Constraint
            The constraint that only has to hold while ``var`` equals ``value``.

        Raises
        ------
        ValueError
            If ``value`` is neither 0 nor 1.
        UnsupportedOperationError
            If ``var`` is not a binary variable.
        DifferentEnvsError
            If ``var`` is from a different environment than this constraint.

        Notes
        -----
        If the constraint itself contains ``var``, the product simplifies
        (``z * z`` is ``z``), and :meth:`indicator` returns this constraint with
        ``var`` fixed to ``value`` instead of the constraint as written.

        Examples
        --------
        >>> from luna_model import Variable, Environment, Vtype
        >>> with Environment():
        ...     z = Variable("z")
        ...     x = Variable("x", vtype=Vtype.INTEGER)
        >>> conditional = (x <= 3).when(z)
        >>> conditional.rhs
        0.0
        >>> conditional.lhs.degree()
        2
        """
        if value not in (0, 1):
            msg = f"a binary variable is 0 or 1, not {value!r}"
            raise ValueError(msg)
        return self._from_pyc(self._c.when(var, bool(value)))

    def indicator(self) -> Indicator | None:
        """Read this constraint as an indicator constraint, if it has that form.

        A constraint has that form if its right-hand side is 0 and a binary
        variable ``z`` is a factor of every term of its left-hand side:
        ``z * (lhs - rhs) <= 0`` is read as ``z == 1`` implies ``lhs <= rhs``.
        An inverted binary as the factor is read as ``z == 0``. Constraints
        written with :meth:`when` have this form, and so has any other constraint
        of that shape.

        Returns
        -------
        Indicator or None
            The variable, the value it holds for and the constraint that holds
            then, or ``None`` if the constraint doesn't have that form. Of
            several binary variables in every term, the first created is taken.

        Examples
        --------
        >>> from luna_model import Variable, Environment, Vtype
        >>> with Environment():
        ...     z = Variable("z")
        ...     x = Variable("x", vtype=Vtype.INTEGER)
        >>> indicator = (x <= 3).when(z).indicator()
        >>> indicator.variable.name, indicator.value
        ('z', 1)
        >>> indicator.constraint.rhs
        3.0
        >>> (x <= 3).indicator() is None
        True
        """
        indicator = self._c.indicator()
        if indicator is None:
            return None
        var, value, constraint = indicator
        return Indicator(wrap_var(var), int(value), self._from_pyc(constraint))

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
