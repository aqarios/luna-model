from __future__ import annotations
from typing import Any, Literal, Protocol, overload
from luna_model._lm import PyAnalysisCache
from luna_model.variable.vtype import Vtype


class MaxBias(Protocol):
    @property
    def val(self) -> float: ...


class BinarySpinInfo(Protocol):
    @property
    def old_vtype(self) -> Vtype:
        """Get the source vtype."""
        ...

    @property
    def new_vtype(self) -> Vtype:
        """Get the target vtype."""
        ...

    @property
    def map(self) -> dict[str, str]:
        """Get the variable name mapping."""
        ...


class IfElseInfo(Protocol):
    @property
    def fulfilled_condition(self) -> bool:
        """If the if-else condition is fulfilled."""
        ...


class AnalysisCache:
    _ac: PyAnalysisCache

    def __init__(self) -> None:
        self._ac = PyAnalysisCache()

    @classmethod
    def _from_pyac(cls, py_ac: PyAnalysisCache) -> AnalysisCache:
        ac = cls.__new__(cls)
        ac._ac = py_ac
        return ac

    @overload
    def __getitem__(self, key: Literal["max-bias"]) -> MaxBias: ...
    @overload
    def __getitem__(self, key: Literal["binary-spin"]) -> BinarySpinInfo: ...
    @overload
    def __getitem__(self, key: str) -> Any | IfElseInfo: ...
    def __getitem__(self, key: str) -> Any:
        return self._ac[key]
