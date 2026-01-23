from __future__ import annotations
from abc import abstractmethod
from luna_model._lm import AnalysisCache as PyAc, AnalysisPass as PyAp

from luna_model.model.model import Model
from luna_model.transformation.base import BasePass


class AnalysisCache:
    _ac: PyAc

    @classmethod
    def _from_pyc(cls, py_c: PyAc) -> AnalysisCache:
        c = cls.__new__(cls)
        c._ac = py_c
        return c


class AnalysisPass(BasePass):
    _ap: PyAp

    @property
    @abstractmethod
    def name(self) -> str:
        """Get the name of this pass."""
        return self._ap.name

    @property
    def requires(self) -> list[str]:
        """Get a list of required passes that need to be run before this pass."""
        return self._ap.requires

    @abstractmethod
    def run(self, model: Model, cache: AnalysisCache) -> ...:
        """Run/Execute this analysis pass."""
        return self._ap.run(model._m, cache._ac)


class MetaAnalysisPass: ...
