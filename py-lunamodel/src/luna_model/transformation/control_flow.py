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
from typing import TYPE_CHECKING

from luna_model._lm import PyControlFlowPass, PyControlFlowPlan, PyModel, PyPassContext
from luna_model.model.model import Model
from luna_model.transformation.context import PassContext

if TYPE_CHECKING:
    from collections.abc import Sequence

    from luna_model.transformation.pipeline import Pipeline
    from luna_model.transformation.typing import Pass


class ControlFlowPlan:
    """Execution plan produced by a control-flow pass.

    A ``ControlFlowPlan`` describes which sub-pipeline should be executed next
    based on a runtime decision (for example, an if/else condition). It is the
    output of a control-flow pass and contains a plan name plus an ordered list
    of passes/steps to run.

    This object does not execute anything itself. Instead, the pass manager
    consumes it and runs the selected steps, then records the nested execution
    in the transformation record so backward replay remains deterministic after
    serialization/deserialization.

    Parameters
    ----------
    name : str
        Human-readable identifier for the selected branch/plan.
    steps : Sequence[Pass]
        Ordered steps that should be executed for this plan.

    Notes
    -----
    The plan should be deterministic for a given model/context state, and should
    only include steps that are valid within the current pipeline scope.
    """

    _p: PyControlFlowPlan

    def __init__(self, name: str, steps: Sequence[Pass] | Pipeline) -> None:
        self._p = PyControlFlowPlan(name, steps)

    @classmethod
    def _from_pyp(cls, py_plan: PyControlFlowPlan) -> ControlFlowPlan:
        p = cls.__new__(cls)
        p._p = py_plan
        return p


class _ControlFlowPassMeta(type(PyControlFlowPass)):
    def __instancecheck__(self, instance: object, /) -> bool:
        return isinstance(instance, PyControlFlowPass) or super().__instancecheck__(instance)


class ControlFlowPass(PyControlFlowPass, metaclass=_ControlFlowPassMeta):
    """
    Abstract base class for control-flow passes.

    Control-Flow passes guide the transformation at runtime. Execution of a
    ``ControlFlowPass`` return a ``ControlFlowPlan`` that consist of transformation
    and analysis (or more control-flow passes) to be executed.

    Notes
    -----
    This is an abstract class. Subclasses must implement the `name`, `run` methods.
    Additionally, the `requires` and `invalidates` and `provides` methods can be implemented.
    """

    @abstractmethod
    def name(self) -> str:
        """
        Get the unique identifier for this pass.

        Returns
        -------
        str
            The unique pass name.
        """
        ...

    @abstractmethod
    def run(self, model: Model, ctx: PassContext) -> ControlFlowPlan:
        """
        Run/Execute this transformation pass.

        Parameters
        ----------
        model : Model
            The model to transform.
        ctx : PassContext
            Context for this pass providing read-access to the analysis cache.

        Returns
        -------
        tuple[Model, Artifact]
            The transformation result containing the model and the artifact
            used for running the backward pass.
        """
        ...

    def requires(self) -> list[str]:
        """
        List of passes that must run before the passes of this control-flow's plan.

        Returns
        -------
        list[str]
            Pass names that must execute first, or empty list if no dependencies.
        """
        return []

    def invalidates(self) -> list[str]:
        """
        Get a list of passes that are invalidated by this control-flow's plan.

        Returns
        -------
        list of str
            Names of passes whose results become invalid after this pass runs.
        """
        return []

    def provides(self) -> list[str]:
        """
        Get the identifier for the analysis cache elments this control-flow's plan generates.

        Returns
        -------
        str
            The identifiers of the cache elements
        """
        return []

    def _run(self, model: PyModel, ctx: PyPassContext) -> PyControlFlowPlan:
        plan = self.run(Model._from_pym(model), PassContext._from_pyctx(ctx))
        return plan._p
