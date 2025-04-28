from pathlib import Path
from typing import overload
from aqmodels import Model

class LpTranslator:
    @overload
    @staticmethod
    def to_aq(file: Path) -> Model: ...
    @overload
    @staticmethod
    def to_aq(file: str) -> Model: ...
    @overload
    @staticmethod
    def from_aq(model: Model) -> str: ...
    @overload
    @staticmethod
    def from_aq(model: Model, file: Path) -> None: ...
