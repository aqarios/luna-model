from typing import Any

from aqmodels._core import Sense

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

__all__ = ["ChangeSensePass", "MaxBiasAnalysis", "PassManager"]
