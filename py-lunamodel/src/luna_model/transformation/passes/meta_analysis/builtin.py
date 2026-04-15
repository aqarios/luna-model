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


from typing import Any, Generic, Protocol, TypeVar, cast

from luna_model.transformation.key import AnalysisKey
from luna_model.transformation.meta_analysis import StepView

Result = TypeVar("Result", covariant=True)  # noqa: PLC0105


class _BuiltinMetaAnalysisSuper(Protocol[Result]):
    def name(self) -> str: ...
    def run(self, steps: list[StepView]) -> Result: ...
    @classmethod
    def provides(cls) -> str: ...


class BuiltinMetaAnalysis(Generic[Result]):
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

    def run(self, steps: list[StepView]) -> Result:
        """Execute this meta-analysis pass.

        Parameters
        ----------
        steps : list[StepView]
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
