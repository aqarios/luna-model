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

from luna_model._lm import PyAnalysisCache, PyMetaAnalysisPass

from .base import BasePass
from .cache import AnalysisCache

T = TypeVar("T")


class MetaAnalysisPass(PyMetaAnalysisPass, BasePass, Generic[T]):
    """MetaAnalysisPass."""

    _base: MetaAnalysisPass

    def __init__(self, base: MetaAnalysisPass | None = None) -> None:
        self._base = base if base else PyMetaAnalysisPass()

    @property
    @abstractmethod
    def name(self) -> str:
        """Get the name of this pass."""
        ...

    @property
    def requires(self) -> list[str]:
        """Get a list of required passes that need to be run before this pass."""
        return self._base.requires

    @abstractmethod
    def run(self, passes: list[BasePass], cache: AnalysisCache) -> T:
        """Run/Execute this analysis pass."""
        ...

    def _run(self, passes: list[BasePass], cache: PyAnalysisCache) -> T:
        return self.run(passes, AnalysisCache._from_pyac(cache))


class ConcreteMetaAnalysisPass(MetaAnalysisPass, Generic[T]):
    """ConcreteMetaAnalysisPass."""

    @property
    def name(self) -> str:
        """Get the name of this pass."""
        return self._base.name

    def run(self, passes: list[BasePass], cache: AnalysisCache) -> T:
        """Run/Execute this analysis pass."""
        return self._base.run(passes, cache._ac)
