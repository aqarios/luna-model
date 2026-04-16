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

from abc import abstractmethod
from typing import Generic, TypeVar

from luna_model._lm import PyAnalysisPass, PyModel, PyPassContext
from luna_model.model.model import Model
from luna_model.transformation.context import PassContext
from luna_model.transformation.key import AnalysisKey

Result = TypeVar("Result")


class _AnalysisPassMeta(type(PyAnalysisPass)):
    def __instancecheck__(self, instance: object, /) -> bool:
        return isinstance(instance, PyAnalysisPass) or super().__instancecheck__(instance)


class AnalysisPass(PyAnalysisPass, Generic[Result], metaclass=_AnalysisPassMeta):
    """
    Abstract base class for analysis passes that analyse models.

    Analysis passes retrieve information from models can used by transformation passes.

    Notes
    -----
    This is an abstract class. Subclasses must implement the `name` and `run` methods and
    the `PROVIDES` class variable.
    Additionally, the `requires` method can be implemented to indicate which passes must
    be executed before the analysis is run.
    """

    PROVIDES: str

    @abstractmethod
    def name(self) -> str:
        """
        Get the name for this pass.

        Returns
        -------
        str
            The unique pass name.
        """
        ...

    @abstractmethod
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
        ...

    def requires(self) -> list[str]:
        """
        List of passes that must run before this pass.

        Returns
        -------
        list[str]
            Pass names that must execute first, or empty list if no dependencies.
        """
        return []

    @classmethod
    def provides(cls) -> str:
        """
        Get the identifier for the analysis cache elment this pass generates.

        Returns
        -------
        str
            The identifier of the cache elment
        """
        return cls.PROVIDES

    @classmethod
    def key(cls) -> AnalysisKey[Result]:
        """Get the analysis key used to access the analysis result from the PassContext."""
        return AnalysisKey(cls.PROVIDES)

    def _run(self, model: PyModel, ctx: PyPassContext) -> Result:
        return self.run(Model._from_pym(model), PassContext._from_pyctx(ctx))
