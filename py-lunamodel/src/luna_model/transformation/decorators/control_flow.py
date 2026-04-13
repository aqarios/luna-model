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

from collections.abc import Callable
from typing import TypeAlias

from luna_model.model.model import Model
from luna_model.transformation.context import PassContext
from luna_model.transformation.control_flow import ControlFlowPass, ControlFlowPlan

ControlFlowSignature: TypeAlias = Callable[[Model, PassContext], ControlFlowPlan]


class _DynamicControlFlowPass(ControlFlowPass):
    _name: str
    _requires: list[str]
    _invalidates: list[str]
    _provides: list[str]
    _run_f: ControlFlowSignature

    def __init__(
        self, name: str, requires: list[str], invalidates: list[str], provides: list[str], run: ControlFlowSignature
    ) -> None:
        self._name = name
        self._requires = requires
        self._invalidates = invalidates
        self._provides = provides
        self._run_f = run

    def name(self) -> str:
        return self._name

    def run(self, model: Model, ctx: PassContext) -> ControlFlowPlan:
        return self._run_f(model, ctx)

    def requires(self) -> list[str]:
        return self._requires

    def invalidates(self) -> list[str]:
        return self._invalidates

    def provides(self) -> list[str]:
        return self._provides


def control_flow(
    name: str | None = None,
    requires: list[str] | None = None,
    invalidates: list[str] | None = None,
    provides: list[str] | None = None,
) -> Callable[[ControlFlowSignature], _DynamicControlFlowPass]:
    """Create a ``ControlFlowPass`` from a function decorator.

    This decorator converts a regular Python function into a ``ControlFlowPass``.
    A control-flow pass does not directly transform the model. Instead, it selects
    which sub-pipeline (plan) should run at runtime based on the current model and
    pass context.

    Parameters
    ----------
    name : str, optional
        Name of the control-flow pass. If not provided, the function name is used
        with underscores replaced by hyphens.
    requires : list[str], optional
        Pass/analysis keys that must be satisfied before this pass can execute.
        Defaults to ``[]``.
    invalidates : list[str], optional
        Analysis keys invalidated after the selected plan is executed.
        Defaults to ``[]``.

    Returns
    -------
    Callable[[ControlFlowSignature], DynamicControlFlowPass]
        A decorator that converts the decorated function into a ``ControlFlowPass``.

    Examples
    --------
    Create a simple conditional branch selector:

    >>> from luna_model.transformation import control_flow, ControlFlowPlan
    >>> @control_flow(name="has-binary")
    ... def choose_branch(model, ctx):
    ...     if model.has_vtype("BINARY"):
    ...         return ControlFlowPlan("then", [BinarySpinPass(...)])
    ...     return ControlFlowPlan("else", [])

    Notes
    -----
    The decorated function must return a ``ControlFlowPlan`` containing:
    - a plan name
    - an ordered list of steps to execute

    Backward replay is derived from the recorded execution of the selected plan.
    """
    if requires is None:
        requires = []
    if invalidates is None:
        invalidates = []
    if provides is None:
        provides = []

    def _decorator(func: ControlFlowSignature) -> _DynamicControlFlowPass:
        loc_name = name or func.__name__.replace("_", "-")
        return _DynamicControlFlowPass(
            name=loc_name,
            requires=requires,
            invalidates=invalidates,
            provides=provides,
            run=func,
        )

    return _decorator
