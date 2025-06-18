from abc import abstractmethod
from enum import Enum
from typing import Any

from aqmodels._core import Model, Sense, Solution

class AnalysisCache: ...

class TransformationType(Enum):
    DidTransform = ...
    NoTranform = ...

class PassManager:
    """TODO."""

    def __init__(self, passes: None | list[Any]) -> None: ...
    def run(self, model: Model) -> tuple[Model, AnalysisCache]: ...

class BasePass:
    @property
    def name(self) -> str: ...
    @property
    def requires(self) -> list[str]: ...

class ChangeSensePass(BasePass):
    """TODO."""

    def __init__(self, sense: Sense = ...) -> None: ...
    @property
    def sense(self) -> Sense: ...

class MaxBiasAnalysis(BasePass):
    """TODO."""

    def __init__(self) -> None: ...

class TransformationPass:
    @property
    @abstractmethod
    def name(self) -> str: ...
    @property
    def requires(self) -> list[str]: ...
    @property
    def invalidates(self) -> list[str]: ...
    @abstractmethod
    def run(
        self, model: Model, cache: AnalysisCache
    ) -> tuple[Model, TransformationType]: ...
    @abstractmethod
    def backwards(self, solution: Solution, cache: AnalysisCache) -> Solution: ...
