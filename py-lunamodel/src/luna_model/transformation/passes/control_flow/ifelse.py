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

from typing import TYPE_CHECKING, Self

from luna_model._lm import PyIfElsePass, PyModel, PyPassContext
from luna_model.model.model import Model
from luna_model.transformation.context import PassContext
from luna_model.transformation.passes.control_flow.builtin import BuiltinControlFlow

if TYPE_CHECKING:
    from collections.abc import Callable

    from luna_model.transformation.pipeline import Pipeline
    from luna_model.transformation.typing import Pass


class IfElsePass(PyIfElsePass, BuiltinControlFlow):
    """Conditional pass that returns different branches based on a runtime condition.

    The ``IfElsePass`` evaluates a condition function against the model and the context at runtime
    and returns a plan containing either the ``then`` pipeline or the ``otherwise`` pipeline based
    on the result. This enables branching logic in transformation workflows, allowing different
    transformation strategies based on model properties discovered during analysis.

    Parameters
    ----------
    condition : Callable[[Model, PassContext], bool]
        A function that takes the current model and the ``PassContext`` and returns a boolean.
        If ``True`` the ``then`` pipeline is returned in the plan; otherwise, the ``otherwise``
        pipeline is returned in the plan.
    then : Pipeline | list[Pass]
        The pipeline or passes to execute when the condition evaluates to ``True``.
    otherwise : Pipeline
        The pipeline to execute when the condition evaluates to ``False``.
    name : str, optional
        Optional name for this pass. If not provided, a default name is generated.

    Examples
    --------
    Execute different transformations based on the maximum bias:

    >>> from luna_model import Model, Vtype
    >>> from luna_model.transformation import PassManager
    >>> from luna_model.transformation.passes import MaxBiasAnalysis, BinarySpinPass, IfElsePass
    >>> # Create conditional pass
    >>> conditional = IfElsePass(
    ...     condition=lambda _, ctx: ctx.require_analysis(MaxBiasAnalysis.key()).val > 10.0,
    ...     then=[BinarySpinPass(Vtype.BINARY)],
    ...     otherwise=[],
    ...     name="conditional-spin-conversion",
    ... )

    Let's create a model that satisfies the condition:

    >>> model = Model("example")
    >>> x = model.add_variable("x", vtype=Vtype.SPIN)
    >>> y = model.add_variable("y")
    >>> z = model.add_variable("z")
    >>> model.objective = 2 * x + 12 * y * z

    And use it in the pass manager:

    >>> pm = PassManager([MaxBiasAnalysis(), conditional])
    >>> result = pm.run(model)
    >>> print(result.model)
    Model: example
    Minimize
      12 * y * z - 4 * x_x + 2
    Binary
      y z x_x

    And now a model that does not satisfy the condition:

    >>> model = Model("example")
    >>> x = model.add_variable("x", vtype=Vtype.SPIN)
    >>> y = model.add_variable("y")
    >>> z = model.add_variable("z")
    >>> model.objective = 2 * x + 8 * y * z

    And use it in the pass manager:

    >>> pm = PassManager([MaxBiasAnalysis(), conditional])
    >>> result = pm.run(model)
    >>> print(result.model)
    Model: example
    Minimize
      8 * y * z + 2 * x
    Binary
      y z
    Spin
      x

    Notes
    -----
    Both pipelines can contain arbitrarily complex sequences
    of transformations and analyses.

    The condition is evaluated once per ``IfElsePass`` execution. If you need to
    re-evaluate during pipeline execution, nest multiple ``IfElsePass`` instances.
    """

    def __new__(
        cls,
        condition: Callable[[Model, PassContext], bool],
        then: Pipeline | list[Pass],
        otherwise: Pipeline | list[Pass],
        name: str | None = None,
    ) -> Self:
        """Construct a new BinarySpinPass instance."""
        return super().__new__(
            cls,
            condition=cls._mapped_predicate(condition),
            then=then,
            otherwise=otherwise,
            name=name,
        )

    @classmethod
    def _mapped_predicate(
        cls, predicate: Callable[[Model, PassContext], bool]
    ) -> Callable[[PyModel, PyPassContext], bool]:
        def _pred(model: PyModel, ctx: PyPassContext) -> bool:
            return predicate(Model._from_pym(model), PassContext._from_pyctx(ctx))

        return _pred
