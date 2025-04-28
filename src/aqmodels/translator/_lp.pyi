from pathlib import Path
from typing import overload
from aqmodels import Model

class LpTranslator:
    @overload
    @staticmethod
    def to_model(file: Path) -> Model: ...
    @overload
    @staticmethod
    def to_model(file: str) -> Model: ...
    @overload
    @staticmethod
    def from_model(model: Model) -> str: ...
    @overload
    @staticmethod
    def from_model(model: Model, file: Path) -> None: ...
