from pathlib import Path
from aqmodels._model import Model

class LpTranslator:
    @staticmethod
    def to_model(filepath: Path) -> Model: ...
    @staticmethod
    def to_file(model: Model) -> str: ...
