from __future__ import annotations

from luna_model._lm import PyBounds, PyUnbounded
from luna_model.variable.vtype import Vtype


class Unbounded(PyUnbounded): ...


class Bounds:
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
        return self._b.lower

    @property
    def upper(self) -> float | type[Unbounded] | None:
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
