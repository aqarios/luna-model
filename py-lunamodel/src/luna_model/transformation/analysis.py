from __future__ import annotations

from abc import abstractmethod
from typing import Generic, TypeVar

from luna_model._lm import PyAnalysisCache, PyAnalysisPass, PyModel
from luna_model.model.model import Model

from .base import BasePass
from .cache import AnalysisCache

T = TypeVar("T")


class AnalysisPass(PyAnalysisPass, BasePass, Generic[T]):
    """AnalysisPass."""

    _base: AnalysisPass

    def __init__(self, base: AnalysisPass | None = None) -> None:
        self._base = base if base else PyAnalysisPass()

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
    def run(self, model: Model, cache: AnalysisCache) -> T:
        """Run/Execute this analysis pass."""
        ...

    def _run(self, model: PyModel, cache: PyAnalysisCache) -> T:
        return self.run(Model._from_pym(model), AnalysisCache._from_pyac(cache))


class ConcreteAnalysisPass(AnalysisPass, Generic[T]):
    """A concrete analysis pass."""

    @property
    def name(self) -> str:
        """Get the name of this pass."""
        return self._base.name

    def run(self, model: Model, cache: AnalysisCache) -> T:
        """Run/Execute this analysis pass."""
        return self._base.run(model._m, cache._ac)
