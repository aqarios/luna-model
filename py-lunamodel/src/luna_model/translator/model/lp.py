from pathlib import Path

from luna_model._lm import PyLpTranslator
from luna_model.model.model import Model


class LpTranslator:
    @staticmethod
    def to_lm(file: str | Path) -> Model:
        return Model._from_pym(PyLpTranslator.to_lm(file))

    @staticmethod
    def from_lm(model: Model, filepath: Path | None = None) -> str | None:
        return PyLpTranslator.from_lm(model._m, filepath)
