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


from typing import Protocol, cast

from luna_model.model.model import Model
from luna_model.transformation.context import PassContext
from luna_model.transformation.control_flow import ControlFlowPass, ControlFlowPlan


class _BuiltinControlFlowMeta(type(ControlFlowPass)):
    def __instancecheck__(self, instance: object, /) -> bool:
        return isinstance(instance, ControlFlowPass) or super().__instancecheck__(instance)


class _BuiltinControlFlowSuper(Protocol):
    def name(self) -> str: ...
    def requires(self) -> list[str]: ...
    def invalidates(self) -> list[str]: ...
    def provides(self) -> list[str]: ...
    def run(self, model: Model, ctx: PassContext) -> ControlFlowPlan: ...
    def __str__(self) -> str: ...


class BuiltinControlFlow(metaclass=_BuiltinControlFlowMeta):
    """A builtin control-flow pass.

    Control-Flow passes guide the transformation at runtime. Execution of a
    ``ControlFlowPass`` return a ``ControlFlowPlan`` that consist of transformation
    and analysis (or more control-flow passes) to be executed.
    """

    def run(self, model: Model, ctx: PassContext) -> ControlFlowPlan:
        """
        Run/Execute this control-flow pass.

        Parameters
        ----------
        model : Model
            The model to analyse.
        ctx : PassContext
            Context for this pass providing read-access to the analysis cache.

        Returns
        -------
        Result
            The plan to be executed.
        """
        sup = cast("_BuiltinControlFlowSuper", super())
        return sup.run(model._m, ctx._c)

    def name(self) -> str:
        """
        Get the unique identifier for this pass.

        Returns
        -------
        str
            The unique pass name.
        """
        sup = cast("_BuiltinControlFlowSuper", super())
        return sup.name()

    def requires(self) -> list[str]:
        """
        List of passes that must run before the passes of this control-flow's plan.

        Returns
        -------
        list[str]
            Pass names that must execute first, or empty list if no dependencies.
        """
        sup = cast("_BuiltinControlFlowSuper", super())
        return sup.requires()

    def invalidates(self) -> list[str]:
        """
        Get a list of passes that are invalidated by this control-flow's plan.

        Returns
        -------
        list of str
            Names of passes whose results become invalid after this pass runs.
        """
        sup = cast("_BuiltinControlFlowSuper", super())
        return sup.invalidates()

    def provides(self) -> list[str]:
        """
        Get the identifier for the analysis cache elments this control-flow's plan generates.

        Returns
        -------
        str
            The identifiers of the cache elements
        """
        sup = cast("_BuiltinControlFlowSuper", super())
        return sup.invalidates()

    def __str__(self) -> str:
        """Human readable string."""
        sup = cast("_BuiltinControlFlowSuper", super())
        return sup.__str__()
