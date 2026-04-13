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

from luna_model._lm import PyModel, PyPassContext
from luna_model.model.model import Model
from luna_model.transformation.context import PassContext
from luna_model.transformation.key import AnalysisKey

Result = TypeVar("Result", covariant=True)  # noqa: PLC0105


class _BuiltinSuper(Protocol[Result]):
    @classmethod
    def provides(cls) -> str: ...
    def name(self) -> str: ...
    def run(self, model: PyModel, ctx: PyPassContext) -> Result: ...
    def requires(self) -> list[str]: ...


class BuiltinAnalysis(Generic[Result]):
    """A builtin analysis pass.

    Analysis passes retrieve information from models can used by transformation passes.
    """

    def __init__(self, *args: Any, **kwargs: Any) -> None:
        super().__init__(*args, **kwargs)

    def name(self) -> str:
        """
        Get the name for this pass.

        Returns
        -------
        str
            The unique pass name.
        """
        sup = cast("_BuiltinSuper[Result]", super())
        return sup.name()

    def run(self, model: Model, ctx: PassContext) -> Result:
        """
        Run/Execute this analysis pass.

        Parameters
        ----------
        model : Model
            The model to analyse.
        ctx : PassContext
            Context for this pass providing read-access to the analysis cache.

        Returns
        -------
        Result
            The analysis result.
        """
        sup = cast("_BuiltinSuper[Result]", super())
        return sup.run(model._m, ctx._c)

    def requires(self) -> list[str]:
        """
        List of passes that must run before this pass.

        Returns
        -------
        list[str]
            Pass names that must execute first, or empty list if no dependencies.
        """
        sup = cast("_BuiltinSuper[Result]", super())
        return sup.requires()

    @classmethod
    def provides(cls) -> str:
        """
        Get the identifier for the analysis cache elment this pass generates.

        Returns
        -------
        str
            The identifier of the cache element
        """
        sup = cast("_BuiltinSuper[Result]", super())
        return sup.provides()

    @classmethod
    def key(cls) -> AnalysisKey[Result]:
        """Get the analysis key used to access the analysis result from the PassContext."""
        return AnalysisKey(cls.provides())
