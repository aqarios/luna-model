from __future__ import annotations
from abc import abstractmethod
from typing import Any, overload

from luna_model.model.model import Model
from luna_model.solution.sol import Solution

from luna_model._lm import (
    PyTransformationPass,
    PyTransformationOutcome,
    PyModel,
    PySolution,
    PyAnalysisCache,
    PyActionType,
)

from .base import BasePass
from .analysis import AnalysisCache
from .action_type import ActionType


class TransformationOutcome:
    """Output object for transformation pass."""

    _to: PyTransformationOutcome

    @classmethod
    def _from_pyto(cls, pyto: PyTransformationOutcome) -> TransformationOutcome:
        to = cls.__new__(cls)
        to._to = pyto
        return to

    @overload
    def __init__(self, model: Model, action: ActionType) -> None: ...
    @overload
    def __init__(self, model: Model, action: ActionType, analysis: object) -> None: ...
    def __init__(
        self, model: Model, action: ActionType, analysis: object | None = None
    ) -> None:
        self._to = PyTransformationOutcome(model._m, action._val, analysis)

    @classmethod
    def nothing(cls, model: Model) -> TransformationOutcome:
        """Easy nothing action return."""
        return cls._from_pyto(PyTransformationOutcome.nothing(model))

    @property
    def model(self) -> Model:
        return Model._from_pym(self._to.model)

    @model.setter
    def model(self, model: Model) -> None:
        self._to.model = model._m

    @property
    def action(self) -> ActionType:
        return ActionType._from_pyat(self._to.action)

    @action.setter
    def action(self, action_type: ActionType) -> None:
        self._to.action = action_type._val

    @property
    def analysis(self) -> Any:
        return self._to.analysis

    @analysis.setter
    def analysis(self, value: Any) -> None:
        self._to.analysis = value


class TransformationPass(PyTransformationPass, BasePass):
    _base: TransformationPass

    def __init__(self, base: TransformationPass = PyTransformationPass()) -> None:
        self._base = base

    @property
    @abstractmethod
    def name(self) -> str:
        """Get the name of this pass."""
        return self._base.name

    @property
    def requires(self) -> list[str]:
        """Get a list of required passes that need to be run before this pass."""
        return self._base.requires

    @property
    def invalidates(self) -> list[str]:
        """Get a list of passes that are invalidated by this pass."""
        return self._base.invalidates

    @abstractmethod
    def run(
        self, model: Model, cache: AnalysisCache
    ) -> (
        TransformationOutcome
        | tuple[PyModel, PyActionType]
        | tuple[PyModel, PyActionType, Any]
    ):
        """Run/Execute this transformation pass."""
        ...

    @abstractmethod
    def backwards(self, solution: Solution, cache: AnalysisCache) -> Solution:
        """Convert a solution back to fit this pass' input.

        Convert a solution from a representation fitting this pass' output to
        a solution representation fitting this pass' input.
        """
        ...

    def _run(
        self, model: PyModel, cache: PyAnalysisCache
    ) -> (
        PyTransformationOutcome
        | tuple[PyModel, PyActionType]
        | tuple[PyModel, PyActionType, Any]
    ):
        inter = self.run(Model._from_pym(model), AnalysisCache._from_pyac(cache))
        if isinstance(inter, tuple) and len(inter) == 2:
            model, at = inter
            return model._m, at._val
        elif isinstance(inter, tuple) and len(inter) == 3:
            model, at, c = inter
            return model._m, at._val, c
        else:
            return inter._to

    def _backwards(self, solution: PySolution, cache: PyAnalysisCache) -> PySolution:
        return self.backwards(
            Solution._from_pys(solution), AnalysisCache._from_pyac(cache)
        )._s


class ConcreteTransformationPass(TransformationPass):
    @property
    def name(self) -> str:
        """Get the name of this pass."""
        return self._base.name

    def run(
        self, model: Model, cache: AnalysisCache
    ) -> (
        TransformationOutcome
        | tuple[PyModel, PyActionType]
        | tuple[PyModel, PyActionType, Any]
    ):
        """Run/Execute this transformation pass."""
        inter = self._base.run(model._m, cache._ac)
        if isinstance(inter, tuple) and len(inter) == 2:
            model, at = inter
            return Model._from_pym(model), ActionType._from_pyat(at)
        elif isinstance(inter, tuple) and len(inter) == 3:
            model, at, c = inter
            return Model._from_pym(model), ActionType._from_pyat(at), c
        else:
            return TransformationOutcome._from_pyto(inter)

    def backwards(self, solution: Solution, cache: AnalysisCache) -> Solution:
        """Convert a solution back to fit this pass' input.

        Convert a solution from a representation fitting this pass' output to
        a solution representation fitting this pass' input.
        """
        return Solution._from_pys(self._base.backwards(solution._s, cache._ac))
