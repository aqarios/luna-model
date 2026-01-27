from __future__ import annotations
from abc import abstractmethod
from typing import Any, Literal, Protocol, overload
from luna_model._lm import PyAnalysisCache, PyAnalysisPass, PyModel

from luna_model.model.model import Model

from .base import BasePass


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


class AnalysisPass(PyAnalysisPass, BasePass):
    _base: AnalysisPass

    def __init__(self, base: AnalysisPass = PyAnalysisPass()) -> None:
        self._base = base

    @property
    @abstractmethod
    def name(self) -> str:
        """Get the name of this pass."""
        ...

    @property
    def requires(self) -> list[str]:
        """Get a list of required passes that need to be run before this pass."""
        return self._base.requires

    @abstractmethod
    def run(self, model: Model, cache: AnalysisCache) -> Any:
        """Run/Execute this analysis pass."""
        ...

    def _run(self, model: PyModel, cache: PyAnalysisCache) -> Any:
        return self.run(Model._from_pym(model), AnalysisCache._from_pyac(cache))


class ConcreteAnalysisPass(AnalysisPass):
    @property
    def name(self) -> str:
        """Get the name of this pass."""
        return self._base.name

    def run(self, model: Model, cache: AnalysisCache) -> Any:
        """Run/Execute this analysis pass."""
        return self._base.run(model._m, cache._ac)
