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

from typing import TYPE_CHECKING, Any, Generic, Protocol, Self, TypeAlias, TypeVar, cast

from luna_model.transformation.analysis import AnalysisPass
from luna_model.transformation.composite import CompositePass
from luna_model.transformation.control_flow import ControlFlowPass
from luna_model.transformation.key import AnalysisKey
from luna_model.transformation.meta_analysis import MetaAnalysisPass
from luna_model.transformation.passes.analysis.builtin import BuiltinAnalysis
from luna_model.transformation.passes.composite.builtin import BuiltinComposite
from luna_model.transformation.passes.control_flow.builtin import BuiltinControlFlow
from luna_model.transformation.passes.transformation.builtin import BuiltinTransformation
from luna_model.transformation.transformation import TransformationPass

if TYPE_CHECKING:
    from collections.abc import Sequence

Result = TypeVar("Result", covariant=True)  # noqa: PLC0105


Pass: TypeAlias = (
    AnalysisPass
    | CompositePass
    | ControlFlowPass
    | MetaAnalysisPass
    | TransformationPass
    | BuiltinAnalysis
    | BuiltinComposite
    | BuiltinControlFlow
    | BuiltinTransformation
)


class _BuiltinMetaAnalysisMeta(type(MetaAnalysisPass)):
    def __instancecheck__(self, instance: object, /) -> bool:
        return isinstance(instance, MetaAnalysisPass) or super().__instancecheck__(instance)


class _BuiltinMetaAnalysisSuper(Protocol[Result]):
    def name(self) -> str: ...
    def run(self, steps: Sequence[Pass | Self]) -> Result: ...
    @classmethod
    def provides(cls) -> str: ...


class BuiltinMetaAnalysis(Generic[Result], metaclass=_BuiltinMetaAnalysisMeta):
    """A builtin meta-analysis pass.

    Meta-analysis passes inspect upcoming pipeline steps and produce analysis
    data for later passes.
    """

    def __init__(self, *args: Any, **kwargs: Any) -> None:
        super().__init__(*args, **kwargs)

    def name(self) -> str:
        """Get the unique pass name.

        Returns
        -------
        str
            The unique pass name.
        """
        sup = cast("_BuiltinMetaAnalysisSuper[Result]", super())
        return sup.name()

    def run(self, steps: Sequence[Pass | Self]) -> Result:
        """Execute this meta-analysis pass.

        Parameters
        ----------
        steps : Sequence[Pass | Self]
            Remaining pipeline steps (including nested pipelines) represented
            as typed step views.

        Returns
        -------
        Result
            Computed analysis result stored under ``PROVIDES``.
        """
        sup = cast("_BuiltinMetaAnalysisSuper[Result]", super())
        return sup.run(steps)

    @classmethod
    def provides(cls) -> str:
        """Get the analysis key written by this pass.

        Returns
        -------
        str
            Stable provides key.
        """
        sup = cast("_BuiltinMetaAnalysisSuper[Result]", super())
        return sup.provides()

    @classmethod
    def key(cls) -> AnalysisKey[Result]:
        """Get the typed key for retrieving this pass result."""
        return AnalysisKey(cls.provides())
