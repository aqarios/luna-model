"""Bounds for constraining variable values in optimization models.

This module provides the Bounds class for specifying lower and upper bounds
on optimization variables, along with an Unbounded type for representing
variables without constraints.
"""

from __future__ import annotations

from typing import TypeAlias

from luna_model._lm import PyBounds, PyUnbounded
from luna_model.variable.vtype import Vtype

Unbounded: TypeAlias = PyUnbounded


class Bounds:
    """Bounds for constraining variable values.

    Bounds specify the lower and upper limits on the values a variable can take
    during optimization. Bounds can be finite values or unbounded (infinite).

    Parameters
    ----------
    lower : float | type[Unbounded] | None, optional
        The lower bound. Use ``None`` or ``Unbounded`` for negative infinity.
    upper : float | type[Unbounded] | None, optional
        The upper bound. Use ``None`` or ``Unbounded`` for positive infinity.

    Attributes
    ----------
    lower : float | type[Unbounded] | None
        The lower bound value.
    upper : float | type[Unbounded] | None
        The upper bound value.

    Examples
    --------
    Create bounded integer variable:

    >>> from luna_model import Variable, Vtype, Bounds
    >>> bounds = Bounds(lower=0, upper=10)
    >>> x = Variable("x", vtype=Vtype.INTEGER, bounds=bounds)

    Create bounds from tuple:

    >>> from luna_model import Variable, Vtype
    >>> y = Variable("y", vtype=Vtype.REAL, bounds=(0.0, 1.0))

    Use predefined bounds for standard types:

    >>> from luna_model.variable.bounds import Bounds
    >>> binary_bounds = Bounds.binary()  # [0, 1]
    >>> spin_bounds = Bounds.spin()  # [-1, 1]

    Notes
    -----
    Default bounds depend on variable type:

    - ``BINARY``: [0, 1]
    - ``SPIN``: [-1, 1]
    - ``INTEGER``: [-2^63, 2^63-1]
    - ``REAL``: [-inf, inf]

    See Also
    --------
    Variable : Decision variable class that uses bounds.
    Vtype : Variable type enumeration.
    """

    _b: PyBounds

    def __init__(
        self,
        lower: float | type[Unbounded] | None = None,
        upper: float | type[Unbounded] | None = None,
    ) -> None:
        self._b = PyBounds(lower, upper)

    @classmethod
    def _from_pyb(cls, py_b: PyBounds) -> Bounds:
        """Construct Bounds from internal PyBounds object.

        Parameters
        ----------
        py_b : PyBounds
            Internal bounds representation.

        Returns
        -------
        Bounds
            New Bounds instance wrapping the PyBounds object.
        """
        b = cls.__new__(cls)
        b._b = py_b
        return b

    @property
    def lower(self) -> float | type[Unbounded] | None:
        """Get the lower bound.

        Returns
        -------
        float | type[Unbounded] | None
            The lower bound value, or None/Unbounded for negative infinity.
        """
        return self._b.lower

    @property
    def upper(self) -> float | type[Unbounded] | None:
        """Get the upper bound.

        Returns
        -------
        float | type[Unbounded] | None
            The upper bound value, or None/Unbounded for positive infinity.
        """
        return self._b.upper

    @classmethod
    def default(cls, vtype: Vtype) -> Bounds:
        """Get the default bounds for a variable type.

        Parameters
        ----------
        vtype : Vtype
            The variable type.

        Returns
        -------
        Bounds
            Default bounds for the specified type:
            - BINARY/INVERTED_BINARY: [0, 1]
            - SPIN: [-1, 1]
            - INTEGER: [-2^63, 2^63-1]
            - REAL: [-inf, inf]
        """
        match vtype:
            case Vtype.BINARY | Vtype.INVERTED_BINARY:
                return cls.binary()
            case Vtype.SPIN:
                return cls.spin()
            case Vtype.INTEGER:
                return cls.integer()
            case Vtype.REAL:
                return cls.real()

    @classmethod
    def binary(cls) -> Bounds:
        """Get the bounds for binary variables [0, 1].

        Returns
        -------
        Bounds
            Bounds with lower=0, upper=1.
        """
        return cls._from_pyb(PyBounds.binary())

    @classmethod
    def spin(cls) -> Bounds:
        """Get the bounds for spin variables [-1, 1].

        Returns
        -------
        Bounds
            Bounds with lower=-1, upper=1.
        """
        return cls._from_pyb(PyBounds.spin())

    @classmethod
    def integer(cls) -> Bounds:
        """Get the bounds for integer variables [-2^63, 2^63-1].

        Returns
        -------
        Bounds
            Bounds with typical integer range limits.
        """
        return cls._from_pyb(PyBounds.integer())

    @classmethod
    def real(cls) -> Bounds:
        """Get the bounds for real variables [-inf, inf].

        Returns
        -------
        Bounds
            Unbounded bounds for real-valued variables.
        """
        return cls._from_pyb(PyBounds.real())

    def __str__(self) -> str:
        """Return human-readable string representation.

        Returns
        -------
        str
            String representation of the bounds.
        """
        return self._b.__str__()

    def __repr__(self) -> str:
        """Return detailed string representation.

        Returns
        -------
        str
            Detailed representation of the bounds.
        """
        return self._b.__repr__()

    def __eq__(self, other: Bounds) -> bool:  # type: ignore[override]
        """Check equality with another Bounds object.

        Parameters
        ----------
        other : Bounds
            The bounds to compare with.

        Returns
        -------
        bool
            True if bounds are equal, False otherwise.
        """
        return self._b.__eq__(other._b)

    def __hash__(self) -> int:
        """Compute hash for use in sets and dictionaries.

        Returns
        -------
        int
            Hash value of the bounds.
        """
        return self._b.__hash__()
