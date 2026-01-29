from enum import Enum

from luna_model._lm import PyTranslationTarget


class TranslationTarget(Enum):
    """TranslationTarget."""

    QUBO = "Qubo"
    LP = "Lp"
    BQM = "Bqm"
    CQM = "Cqm"

    @property
    def _val(self) -> PyTranslationTarget:
        match self:
            case TranslationTarget.QUBO:
                return PyTranslationTarget.Qubo
            case TranslationTarget.LP:
                return PyTranslationTarget.Lp
            case TranslationTarget.BQM:
                return PyTranslationTarget.Bqm
            case TranslationTarget.CQM:
                return PyTranslationTarget.Cqm
