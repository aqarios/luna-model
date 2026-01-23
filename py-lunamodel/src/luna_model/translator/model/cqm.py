from luna_model._lm import PyLpTranslator
from luna_model.model.model import Model

_DIMOD_AVAILABLE: bool = False
try:
    from dimod import ConstrainedQuadraticModel, lp as dimod_lp  # type: ignore[reportMissingImports]

    _DIMOD_AVAILABLE = True
except ImportError:
    _DIMOD_AVAILABLE = False


class CqmTranslator:
    @staticmethod
    def to_lm(cqm: ConstrainedQuadraticModel, *, name: str | None = None) -> Model:
        if not _DIMOD_AVAILABLE:
            raise RuntimeError(
                "dimod is required for the CqmTranslator. You can install it using the 'dimod' extra."
            )
        if not isinstance(cqm, ConstrainedQuadraticModel):  # type: ignore[reportPossiblyUnboundVariable]
            raise TypeError(f"Expected cqm to be of type CQM, received: {type(cqm)}")
        cqm_lp = dimod_lp.dumps(cqm)  # type: ignore[reportPossiblyUnboundVariable]
        model = Model._from_pym(PyLpTranslator.to_lm(cqm_lp))
        if name is not None:
            model.name = name
        return model

    @staticmethod
    def from_lm(model: Model) -> ConstrainedQuadraticModel:
        if not _DIMOD_AVAILABLE:
            raise RuntimeError(
                "dimod is required for the CqmTranslator. You can install it using the 'dimod' extra."
            )
        return dimod_lp.loads(PyLpTranslator.from_lm(model._m))  # type: ignore[reportPossiblyUnboundVariable]
