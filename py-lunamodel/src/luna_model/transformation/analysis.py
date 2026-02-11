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
from typing import Generic, TypeVar

from luna_model._lm import PyAnalysisCache, PyAnalysisPass, PyModel
from luna_model.model.model import Model

from .base import BasePass
from .cache import AnalysisCache

T = TypeVar("T")


class AnalysisPass(PyAnalysisPass, BasePass, Generic[T]):
    """Base class for analysis passes that compute information about models.

    Analysis passes inspect models and compute results without modifying them.
    They can depend on other analysis passes and cache their results for
    efficient access in subsequent passes.
    """

    _base: AnalysisPass

    def __init__(self, base: AnalysisPass | None = None) -> None:
        self._base = base if base else PyAnalysisPass()

    @property
    @abstractmethod
    def name(self) -> str:
        """Get the unique name of this analysis pass.

        Returns
        -------
        str
            A unique identifier for this pass.
        """
        ...

    @property
    def requires(self) -> list[str]:
        """Get the list of passes this pass depends on.

        Returns
        -------
        list[str]
            List of pass names that must be executed before this pass.
        """
        return self._base.requires

    @abstractmethod
    def run(self, model: Model, cache: AnalysisCache) -> T:
        """Execute this analysis pass on a model.

        Parameters
        ----------
        model : Model
            The model to analyze.
        cache : AnalysisCache
            Cache containing results from previous analysis passes.

        Returns
        -------
        T
            The result of the analysis.
        """
        ...

    def _run(self, model: PyModel, cache: PyAnalysisCache) -> T:
        return self.run(Model._from_pym(model), AnalysisCache._from_pyac(cache))


class ConcreteAnalysisPass(AnalysisPass, Generic[T]):
    """A concrete analysis pass that wraps an existing implementation.

    This class provides a concrete implementation of AnalysisPass by delegating
    to an underlying base pass.
    """

    @property
    def name(self) -> str:
        """Get the unique name of this analysis pass.

        Returns
        -------
        str
            The name from the underlying base pass.
        """
        return self._base.name

    def run(self, model: Model, cache: AnalysisCache) -> T:
        """Execute this analysis pass on a model.

        Parameters
        ----------
        model : Model
            The model to analyze.
        cache : AnalysisCache
            Cache containing results from previous analysis passes.

        Returns
        -------
        T
            The result of the analysis from the underlying base pass.
        """
        return self._base.run(model._m, cache._ac)
