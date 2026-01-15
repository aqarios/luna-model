from dimod import ConstrainedQuadraticModel, lp as dimod_lp

from luna_model._lm import PyLpTranslator
from luna_model.model.model import Model


class CqmTranslator:
    @staticmethod
    def to_lm(cqm: ConstrainedQuadraticModel, *, name: str | None = None) -> Model:
        if not isinstance(cqm, ConstrainedQuadraticModel):
            raise TypeError(f"Expected cqm to be of type CQM, received: {type(cqm)}")
        cqm_lp = dimod_lp.dumps(cqm)
        model = PyLpTranslator.to_lm(cqm_lp)
        if name is not None:
            model.name = name
        return model

    @staticmethod
    def from_lm(model: Model) -> ConstrainedQuadraticModel:
        return dimod_lp.loads(PyLpTranslator.from_lm(model))
