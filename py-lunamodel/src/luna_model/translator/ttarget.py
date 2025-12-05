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
