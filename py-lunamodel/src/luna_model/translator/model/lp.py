from pathlib import Path

from luna_model._lm import PyLpTranslator
from luna_model.model.model import Model


class LpTranslator:
    """Lp translator."""

    @staticmethod
    def to_lm(file: str | Path) -> Model:
        """To model from lp file or string."""
        return Model._from_pym(PyLpTranslator.to_lm(file))

    @staticmethod
    def from_lm(model: Model, filepath: Path | None = None) -> str | None:
        """To lp file or string from model."""
        return PyLpTranslator.from_lm(model._m, filepath)
