from enum import Enum

from luna_model._lm import PyTranslationTarget


class TranslationTarget(Enum):
    QUBO = PyTranslationTarget.Qubo
    LP = PyTranslationTarget.Lp
    BQM = PyTranslationTarget.Bqm
    CQM = PyTranslationTarget.Cqm

    @property
    def name(self) -> str:
        match self:
            case TranslationTarget.QUBO:
                return "Qubo"
            case TranslationTarget.LP:
                return "Lp"
            case TranslationTarget.BQM:
                return "Bqm"
            case TranslationTarget.CQM:
                return "Cqm"

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
