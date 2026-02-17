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

import sys
from abc import abstractmethod
from typing import Generic, TypeVar

if sys.version_info < (3, 12):
    from typing_extensions import override
else:
    from typing import override

from luna_model._lm import PyAnalysisCache, PyMetaAnalysisPass

from .base import BasePass
from .cache import AnalysisCache

T = TypeVar("T")


class MetaAnalysisPass(PyMetaAnalysisPass, BasePass, Generic[T]):
    """Base class for meta-analysis passes that analyze other passes.

    Meta-analysis passes can examine the results and metadata of other passes
    to produce higher-level insights or summaries.

    Notes
    -----
    This is an abstract class. Subclasses must implement the `name` property
    and `run` method.
    """

    _base: MetaAnalysisPass

    def __init__(self, base: MetaAnalysisPass | None = None) -> None:
        self._base = base if base else PyMetaAnalysisPass()

    @property
    @override
    @abstractmethod
    def name(self) -> str:
        return self._base.name

    @property
    @override
    def requires(self) -> list[str]:
        return self._base.requires

    @abstractmethod
    def run(self, passes: list[BasePass], cache: AnalysisCache) -> T:
        """
        Run/Execute this meta-analysis pass.

        Parameters
        ----------
        passes : list[BasePass]
            List of passes to analyze.
        cache : AnalysisCache
            Cache containing analysis results.

        Returns
        -------
        T
            The result of the meta-analysis.
        """
        ...

    def _run(self, passes: list[BasePass], cache: PyAnalysisCache) -> T:
        return self.run(passes, AnalysisCache._from_pyac(cache))


class ConcreteMetaAnalysisPass(MetaAnalysisPass, Generic[T]):
    """Concrete implementation of a meta-analysis pass."""

    @property
    def name(self) -> str:
        """
        Get the name of this pass.

        Returns
        -------
        str
            The unique identifier name for this pass.
        """
        return self._base.name

    def run(self, passes: list[BasePass], cache: AnalysisCache) -> T:
        """
        Run/Execute this meta-analysis pass.

        Parameters
        ----------
        passes : list[BasePass]
            List of passes to analyze.
        cache : AnalysisCache
            Cache containing analysis results.

        Returns
        -------
        T
            The result of the meta-analysis.
        """
        return self._base.run(passes, cache._ac)
