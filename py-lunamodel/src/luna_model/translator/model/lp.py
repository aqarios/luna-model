from pathlib import Path

from luna_model._lm import PyLpTranslator
from luna_model.model.model import Model


class LpTranslator:
    @staticmethod
    def to_aq(file: str | Path) -> Model:
        return Model._from_pym(PyLpTranslator.to_aq(file))

    @staticmethod
    def from_aq(model: Model, filepath: Path | None = None) -> str | None:
        return PyLpTranslator.from_aq(model._m, filepath)
