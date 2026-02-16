# Copyright 2026 Aqarios GmbH
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.
from __future__ import annotations

from abc import abstractmethod
from typing import Any, overload, override

from luna_model._lm import (
    PyActionType,
    PyAnalysisCache,
    PyModel,
    PySolution,
    PyTransformationOutcome,
    PyTransformationPass,
)
from luna_model.model.model import Model
from luna_model.solution.sol import Solution

from .action_type import ActionType
from .base import BasePass
from .cache import AnalysisCache


class TransformationOutcome:
    """
    Output object for transformation pass.

    Encapsulates the result of applying a transformation pass, including
    the transformed model, the action taken, and optional analysis data.

    Parameters
    ----------
    model : Model
        The transformed model.
    action : ActionType
        The type of action performed during transformation.
    analysis : object, optional
        Additional analysis data produced during transformation.
    """

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
    def __init__(self, model: Model, action: ActionType, analysis: object | None = None) -> None:
        self._to = PyTransformationOutcome(model._m, action._val, analysis)

    @classmethod
    def nothing(cls, model: Model) -> TransformationOutcome:
        """
        Create a transformation outcome indicating no changes were made.

        Parameters
        ----------
        model : Model
            The unchanged model.

        Returns
        -------
        TransformationOutcome
            A transformation outcome with action type set to nothing.
        """
        return cls._from_pyto(PyTransformationOutcome.nothing(model._m))

    @property
    def model(self) -> Model:
        """
        Get the transformed model.

        Returns
        -------
        Model
            The model after transformation.
        """
        return Model._from_pym(self._to.model)

    @model.setter
    def model(self, model: Model) -> None:
        """
        Set the transformed model.

        Parameters
        ----------
        model : Model
            The model to set as the transformation result.
        """
        self._to.model = model._m

    @property
    def action(self) -> ActionType:
        """
        Get the action type performed.

        Returns
        -------
        ActionType
            The type of transformation action that was performed.
        """
        return ActionType._from_pyat(self._to.action)

    @action.setter
    def action(self, action_type: ActionType) -> None:
        """
        Set the action type.

        Parameters
        ----------
        action_type : ActionType
            The type of transformation action to set.
        """
        self._to.action = action_type._val

    @property
    def analysis(self) -> Any:  # noqa: ANN401
        """
        Get the analysis data.

        Returns
        -------
        Any
            Additional analysis data produced during transformation.
        """
        return self._to.analysis

    @analysis.setter
    def analysis(self, value: Any) -> None:  # noqa: ANN401
        """
        Set the analysis data.

        Parameters
        ----------
        value : Any
            Analysis data to associate with this transformation outcome.
        """
        self._to.analysis = value


class TransformationPass(PyTransformationPass, BasePass):
    """
    Base class for transformation passes that modify models.

    Transformation passes apply changes to models and can also convert
    solutions backwards to match the input representation.

    Notes
    -----
    This is an abstract class. Subclasses must implement the `name`, `run`,
    and `backwards` methods.
    """

    _base: TransformationPass

    def __init__(self, base: TransformationPass | None = None) -> None:
        self._base = base if base else PyTransformationPass()

    @property
    @override
    @abstractmethod
    def name(self) -> str:
        return self._base.name

    @property
    @override
    def requires(self) -> list[str]:
        return self._base.requires

    @property
    def invalidates(self) -> list[str]:
        """
        Get a list of passes that are invalidated by this pass.

        Returns
        -------
        list of str
            Names of passes whose results become invalid after this pass runs.
        """
        return self._base.invalidates

    @abstractmethod
    def run(
        self, model: Model, cache: AnalysisCache
    ) -> TransformationOutcome | tuple[PyModel, PyActionType] | tuple[PyModel, PyActionType, Any]:
        """
        Run/Execute this transformation pass.

        Parameters
        ----------
        model : Model
            The model to transform.
        cache : AnalysisCache
            Cache containing analysis results.

        Returns
        -------
        TransformationOutcome or tuple
            The transformation result, either as a TransformationOutcome object
            or as a tuple of (model, action_type) or (model, action_type, analysis).
        """
        ...

    @abstractmethod
    def backwards(self, solution: Solution, cache: AnalysisCache) -> Solution:
        """
        Convert a solution back to fit this pass' input.

        Converts a solution from a representation fitting this pass' output to
        a solution representation fitting this pass' input.

        Parameters
        ----------
        solution : Solution
            The solution in the output representation.
        cache : AnalysisCache
            Cache containing analysis results.

        Returns
        -------
        Solution
            The solution converted to the input representation.
        """
        ...

    def _run(
        self, model: PyModel, cache: PyAnalysisCache
    ) -> PyTransformationOutcome | tuple[PyModel, PyActionType] | tuple[PyModel, PyActionType, Any]:
        inter = self.run(Model._from_pym(model), AnalysisCache._from_pyac(cache))
        if isinstance(inter, tuple) and len(inter) == 2:  # noqa: PLR2004
            model, at = inter
            return model._m, at._val
        if isinstance(inter, tuple) and len(inter) == 3:  # noqa: PLR2004
            model, at, c = inter
            return model._m, at._val, c
        return inter._to

    def _backwards(self, solution: PySolution, cache: PyAnalysisCache) -> PySolution:
        return self.backwards(Solution._from_pys(solution), AnalysisCache._from_pyac(cache))._s


class ConcreteTransformationPass(TransformationPass):
    """Concrete implementation of a transformation pass."""

    @property
    @override
    def name(self) -> str:
        return self._base.name

    @override
    def run(
        self, model: Model, cache: AnalysisCache
    ) -> TransformationOutcome | tuple[PyModel, PyActionType] | tuple[PyModel, PyActionType, Any]:
        inter = self._base.run(model._m, cache._ac)
        if isinstance(inter, tuple) and len(inter) == 2:  # noqa: PLR2004
            model, at = inter
            return Model._from_pym(model), ActionType._from_pyat(at)
        if isinstance(inter, tuple) and len(inter) == 3:  # noqa: PLR2004
            model, at, c = inter
            return Model._from_pym(model), ActionType._from_pyat(at), c
        return TransformationOutcome._from_pyto(inter)

    @override
    def backwards(self, solution: Solution, cache: AnalysisCache) -> Solution:
        return Solution._from_pys(self._base.backwards(solution._s, cache._ac))
