from __future__ import annotations

from luna_model._lm import PyBounds, PyUnbounded


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
        """Construct LunaModel Variable from FFI PyVariable object."""
        b = cls.__new__(cls)
        b._b = py_b
        return b

    @property
    def lower(self) -> float | type[Unbounded] | None:
        return self._b.lower

    @property
    def upper(self) -> float | type[Unbounded] | None:
        return self._b.lower
