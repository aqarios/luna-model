# type: ignore[reportPossiblyUnboundVariable]
from luna_model._lm import PyLpTranslator
from luna_model.model.model import Model

_DIMOD_AVAILABLE: bool = False
try:
    from dimod import ConstrainedQuadraticModel
    from dimod import lp as dimod_lp

    _DIMOD_AVAILABLE = True
except ImportError:
    _DIMOD_AVAILABLE = False


class CqmTranslator:
    """CqmTranslator."""

    @staticmethod
    def to_lm(cqm: "ConstrainedQuadraticModel", *, name: str | None = None) -> Model:
        """Translate to model from cqm."""
        if not _DIMOD_AVAILABLE:
            msg = "dimod is required for the CqmTranslator. You can install it using the 'dimod' extra."
            raise RuntimeError(msg)
        if not isinstance(cqm, ConstrainedQuadraticModel):
            msg = f"Expected cqm to be of type CQM, received: {type(cqm)}"
            raise TypeError(msg)
        cqm_lp = dimod_lp.dumps(cqm)
        model = Model._from_pym(PyLpTranslator.to_lm(cqm_lp))
        if name is not None:
            model.name = name
        return model

    @staticmethod
    def from_lm(model: Model) -> "ConstrainedQuadraticModel":
        """Translate to cqm from model."""
        if not _DIMOD_AVAILABLE:
            msg = "dimod is required for the CqmTranslator. You can install it using the 'dimod' extra."
            raise RuntimeError(msg)
        return dimod_lp.loads(PyLpTranslator.from_lm(model._m))
