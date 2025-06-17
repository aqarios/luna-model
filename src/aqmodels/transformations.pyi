from abc import abstractmethod
from typing import Any
from enum import Enum

from aqmodels._core import Model, Sense

class TransformationType(Enum):
    DidTransform = ...
    NoTranform = ...

class PassManager:
    """TODO."""

    def __init__(self, passes: None | list[Any]) -> None: ...

class BasePass:
    @property
    def name(self) -> str: ...
    @property
    def requires(self) -> list[str]: ...

class ChangeSensePass(BasePass):
    """TODO."""

    def __init__(self, sense: Sense = Sense.Min) -> None: ...
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
    @abstractmethod
    def requires(self) -> list[str]: ...
    @abstractmethod
    def run(self, model: Model, cache) -> tuple[Model, TransformationType]: ...

__all__ = ["ChangeSensePass", "MaxBiasAnalysis", "PassManager"]
