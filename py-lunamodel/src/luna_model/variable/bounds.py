from __future__ import annotations

from luna_model._lm import PyBounds, PyUnbounded
from luna_model.variable.vtype import Vtype


class Unbounded(PyUnbounded):
    """ """

    ...


class Bounds:
    """
    Represents bounds for a variable (only supported for real and integer variables).

    A `Bounds` object defines the valid interval for a variable. Bounds are inclusive,
    and can be partially specified by providing only a lower or upper limit. If neither
    is specified, the variable is considered unbounded.

    Parameters
    ----------
    lower : float, optional
        Lower bound of the variable. Defaults to negative infinity if not specified.
    upper : float, optional
        Upper bound of the variable. Defaults to positive infinity if not specified.

    Examples
    --------
    >>> from luna_model import Bounds
    >>> Bounds(-1.0, 1.0)
    Bounds { lower: -1, upper: 1 }

    >>> Bounds(lower=0.0)
    Bounds { lower: -1, upper: unlimited }

    >>> Bounds(upper=10.0)
    Bounds { lower: unlimited, upper: 1 }

    Notes
    -----
    - Bounds are only meaningful for variables of type `Vtype.Real` or `Vtype.Integer`.
    - If both bounds are omitted, i.e., `lower=None, upper=None`, the variable is created
      with the default bounds for this variable type.
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
        """Construct LunaModel Variable from FFI PyVariable object."""
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
        return self._b.lower

    @classmethod
    def default(cls, vtype: Vtype) -> Bounds:
        match vtype:
            case (
                Vtype.BINARY
                | Vtype.Binary
                | Vtype.INVERTED_BINARY
                | Vtype.InvertedBinary
            ):
                return cls.binary()
            case Vtype.SPIN | Vtype.Spin:
                return cls.spin()
            case Vtype.INTEGER | Vtype.Integer:
                return cls.integer()
            case Vtype.REAL | Vtype.Real:
                return cls.real()

    @classmethod
    def binary(cls) -> Bounds:
        return cls._from_pyb(PyBounds.binary())

    @classmethod
    def spin(cls) -> Bounds:
        return cls._from_pyb(PyBounds.spin())

    @classmethod
    def integer(cls) -> Bounds:
        return cls._from_pyb(PyBounds.integer())

    @classmethod
    def real(cls) -> Bounds:
        return cls._from_pyb(PyBounds.real())

    # todo: access to the default bounds given a vtype.
