from __future__ import annotations
from luna_model._lm import PyIR

from luna_model.model.model import Model

from luna_model.transformation.analysis import AnalysisCache
from luna_model.transformation.log import LogElement


class IR:
    """The intermediate representation (IR) of a model after transformation.

    The IR contains the resulting model after transformation (`ir.model`) as well
    as the analysis cache (`ir.cache`) and an execution log (`ir.execution_log`).
    """

    _ir: PyIR

    @classmethod
    def _from_pyir(cls, py_ir: PyIR) -> IR:
        ir = cls.__new__(cls)
        ir._ir = py_ir
        return ir

    @property
    def model(self) -> Model:
        """Get the model stored in the IR."""
        return Model._from_pym(self._ir.model)

    @property
    def cache(self) -> AnalysisCache:
        """Get the analysis cache stored the IR."""
        return AnalysisCache._from_pyc(self._ir.cache)

    @property
    def execution_log(self) -> list[LogElement]:
        """Get the analysis cache stored the IR."""
        return [LogElement._from_pyle(e) for e in self._ir.execution_log]
