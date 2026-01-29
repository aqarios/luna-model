from __future__ import annotations

from luna_model._lm import PyIR
from luna_model.model.model import Model

from .cache import AnalysisCache
from .log import LogElement


class IR:
    _ir: PyIR
    """The intermediate representation (IR) of a model after transformation.

    The IR contains the resulting model after transformation (`ir.model`) as well
    as the analysis cache (`ir.cache`) and an execution log (`ir.execution_log`).
    """

    @classmethod
    def _from_pyir(cls, pyir: PyIR) -> IR:
        ir = cls.__new__(cls)
        ir._ir = pyir
        return ir

    @property
    def model(self) -> Model:
        """Get the model stored in the IR."""
        return Model._from_pym(self._ir.model)

    @property
    def cache(self) -> AnalysisCache:
        """Get the analysis cache stored the IR."""
        return AnalysisCache._from_pyac(self._ir.cache)

    @property
    def execution_log(self) -> list[LogElement]:
        """Get the analysis cache stored the IR."""
        return [LogElement._from_pyle(e) for e in self._ir.execution_log]
