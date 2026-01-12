from enum import Enum

from luna_model._lm import PyTranslationTarget


class TranslationTarget(Enum):
    QUBO = PyTranslationTarget.Qubo
    LP = PyTranslationTarget.Lp
    BQM = PyTranslationTarget.Bqm
    CQM = PyTranslationTarget.Cqm

    # todo: deprecate

    Qubo = PyTranslationTarget.Qubo
    Lp = PyTranslationTarget.Lp
    Bqm = PyTranslationTarget.Bqm
    Cqm = PyTranslationTarget.Cqm

    @property
    def name(self) -> str:
        match self:
            case TranslationTarget.QUBO | TranslationTarget.Qubo:
                return "Qubo"
            case TranslationTarget.LP | TranslationTarget.Lp:
                return "Lp"
            case TranslationTarget.BQM | TranslationTarget.Bqm:
                return "Bqm"
            case TranslationTarget.CQM | TranslationTarget.Cqm:
                return "Cqm"

    @property
    def _val(self) -> PyTranslationTarget:
        match self:
            case TranslationTarget.QUBO | TranslationTarget.Qubo:
                return PyTranslationTarget.Qubo
            case TranslationTarget.LP | TranslationTarget.Lp:
                return PyTranslationTarget.Lp
            case TranslationTarget.BQM | TranslationTarget.Bqm:
                return PyTranslationTarget.Bqm
            case TranslationTarget.CQM | TranslationTarget.Cqm:
                return PyTranslationTarget.Cqm
