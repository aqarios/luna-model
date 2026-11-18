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

from collections.abc import Callable, Iterable
from typing import TYPE_CHECKING, Generic, TypeAlias, TypeVar

from luna_model.model.model import Model
from luna_model.transformation.analysis import AnalysisPass
from luna_model.transformation.context import PassContext
from luna_model.transformation.decorators.transformation import _model_parts

if TYPE_CHECKING:
    from luna_model.transformation.model_part import ModelPart

R = TypeVar("R")

AnalysisSignature: TypeAlias = Callable[[Model, PassContext], R]


class _DynamicAnalysisPass(AnalysisPass[R], Generic[R]):
    _name: str
    _requires: list[str]
    _depends_on: set[ModelPart]
    _run_f: AnalysisSignature[R]

    def __init__(self, name: str, requires: list[str], depends_on: set[ModelPart], run: AnalysisSignature[R]) -> None:
        self._name = name
        self._requires = requires
        self._depends_on = depends_on
        self._run_f = run

    def name(self) -> str:
        return self._name

    def run(self, model: Model, ctx: PassContext) -> R:
        return self._run_f(model, ctx)

    def requires(self) -> list[str]:
        return self._requires

    def depends_on(self) -> set[ModelPart]:
        return self._depends_on


def analyze(
    name: str | None = None,
    provides: str | None = None,
    requires: list[str] | None = None,
    *,
    depends_on: Iterable[ModelPart] | None = None,
) -> Callable[[AnalysisSignature[R]], _DynamicAnalysisPass[R]]:
    """Create an AnalysisPass from a function decorator.

    This decorator converts a regular function into an ``AnalysisPass`` that can be used
    in transformation pipelines. Analysis passes inspect models without modifying them,
    computing properties or metadata that other passes can use.

    Parameters
    ----------
    name : str, optional
        The name of the analysis pass. If not provided, uses the function name with
        underscores replaced by hyphens (e.g., ``my_analysis`` becomes ``my-analysis``).
    provides : str, optional
        The key for the result the analysis pass provides. If not specified, uses the function
        name with underscores replaced by hyphens prefixed with ``decorated_analysis::``
        (e.g., ``my_analysis`` becomes ``decorated_analysis::my-analysis``).
    requires : list[str], optional
        List of analysis pass names that must run before this analysis. The results
        of required passes are available in the ``AnalysisCache``. Defaults to ``[]``.
    depends_on : Iterable[ModelPart], optional
        Model parts the result is computed from; it stays valid until a transformation
        modifies one of them. Defaults to every part.

    Returns
    -------
    Callable[[Callable[[Model, PassContext], R]], _DynamicAnalysisPass[R]]
        A decorator that transforms the decorated function into an ``AnalysisPass``.

    Examples
    --------
    Create a simple analysis pass:

    >>> from luna_model.transformation import analyze
    >>> @analyze(name="count-variables")
    ... def count_vars(model, cache) -> float:
    ...     return model.num_variables

    Notes
    -----
    The decorated function must have the signature::

        def my_analysis(model: Model, ctx: PassContext) -> R: ...

    The return value is stored in the ``AnalysisCache`` under the pass's name
    and can be accessed by subsequent passes.
    """
    if requires is None:
        requires = []

    def _decorator(
        func: AnalysisSignature[R],
    ) -> _DynamicAnalysisPass[R]:
        loc_name = name or func.__name__.replace("_", "-")

        class _TheAnalysis(_DynamicAnalysisPass):
            PROVIDES = f"decorated_analysis::{loc_name}" if provides is None else provides

        return _TheAnalysis(
            name=loc_name,
            requires=requires,
            depends_on=_model_parts(depends_on),
            run=func,
        )

    return _decorator
