from __future__ import annotations
from typing import Any, Literal, Protocol, overload
from luna_model._lm import PyAnalysisCache


class MaxBias(Protocol):
    @property
    def val(self) -> float: ...


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
    def __getitem__(self, key: str) -> Any: ...
    def __getitem__(self, key: str) -> Any:
        return self._ac[key]
