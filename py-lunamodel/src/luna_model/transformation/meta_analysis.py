from __future__ import annotations
from abc import abstractmethod
from typing import Any

from luna_model._lm import PyMetaAnalysisPass, PyAnalysisCache

from .base import BasePass
from .cache import AnalysisCache


class MetaAnalysisPass(PyMetaAnalysisPass, BasePass):
    _base: MetaAnalysisPass

    def __init__(self, base: MetaAnalysisPass = PyMetaAnalysisPass()) -> None:
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
    def run(self, passes: list[BasePass], cache: AnalysisCache) -> Any:
        """Run/Execute this analysis pass."""
        ...

    def _run(self, passes: list[BasePass], cache: PyAnalysisCache) -> Any:
        return self.run(passes, AnalysisCache._from_pyac(cache))


class ConcreteMetaAnalysisPass(MetaAnalysisPass):
    @property
    def name(self) -> str:
        """Get the name of this pass."""
        return self._base.name

    def run(self, passes: list[BasePass], cache: AnalysisCache) -> Any:
        """Run/Execute this analysis pass."""
        return self._base.run(passes, cache._ac)
