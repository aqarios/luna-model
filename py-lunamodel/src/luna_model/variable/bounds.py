from __future__ import annotations

from typing import TypeAlias

from luna_model._lm import PyBounds, PyUnbounded
from luna_model.variable.vtype import Vtype

Unbounded: TypeAlias = PyUnbounded


class Bounds:
    """The bounds."""

    _b: PyBounds

    def __init__(
        self,
        lower: float | type[Unbounded] | None = None,
        upper: float | type[Unbounded] | None = None,
    ) -> None:
        self._b = PyBounds(lower, upper)

    @classmethod
    def _from_pyb(cls, py_b: PyBounds) -> Bounds:
        """Construct LunaModel Bounds from FFI PyBounds object."""
        b = cls.__new__(cls)
        b._b = py_b
        return b

    @property
    def lower(self) -> float | type[Unbounded] | None:
        """Get the lower bound."""
        return self._b.lower

    @property
    def upper(self) -> float | type[Unbounded] | None:
        """Get the upper bound."""
        return self._b.upper

    @classmethod
    def default(cls, vtype: Vtype) -> Bounds:
        """Get the default bounds for the vtype."""
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
        """Get the bounds for binary typed variables."""
        return cls._from_pyb(PyBounds.binary())

    @classmethod
    def spin(cls) -> Bounds:
        """Get the bounds for spin typed variables."""
        return cls._from_pyb(PyBounds.spin())

    @classmethod
    def integer(cls) -> Bounds:
        """Get the bounds for integer typed variables."""
        return cls._from_pyb(PyBounds.integer())

    @classmethod
    def real(cls) -> Bounds:
        """Get the bounds for real typed variables."""
        return cls._from_pyb(PyBounds.real())

    def __str__(self) -> str:
        """Compute string representation human readable."""
        return self._b.__str__()

    def __repr__(self) -> str:
        """Compute repr."""
        return self._b.__repr__()

    def __eq__(self, other: Bounds) -> bool:  # type: ignore[override]
        """Compute eq."""
        return self._b.__eq__(other._b)

    def __hash__(self) -> int:
        """Compute hash."""
        return self._b.__hash__()
