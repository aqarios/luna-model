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
from typing import TYPE_CHECKING, Generic, TypeVar

from luna_model._lm import PyMetaAnalysisPass
from luna_model.transformation.key import AnalysisKey

if TYPE_CHECKING:
    from collections.abc import Sequence

    from luna_model.transformation.typing import Pass

Result = TypeVar("Result")


class _MetaAnalysisPassMeta(type(PyMetaAnalysisPass)):
    def __instancecheck__(self, instance: object, /) -> bool:
        return isinstance(instance, PyMetaAnalysisPass) or super().__instancecheck__(instance)


class MetaAnalysisPass(PyMetaAnalysisPass, Generic[Result], metaclass=_MetaAnalysisPassMeta):
    """Abstract base class for meta-analysis passes.

    Meta-analysis passes inspect upcoming pipeline steps and produce analysis
    data for later passes.

    Notes
    -----
    Subclasses must define ``PROVIDES`` and implement ``name`` and ``run``.
    """

    PROVIDES: str

    @abstractmethod
    def name(self) -> str:
        """Get the unique pass name.

        Returns
        -------
        str
            The unique pass name.
        """
        ...

    @abstractmethod
    def run(self, steps: Sequence[Pass]) -> Result:
        """Execute this meta-analysis pass.

        Parameters
        ----------
        steps : list[Pass]
            Remaining pipeline steps (including nested pipelines) represented
            as typed step views.

        Returns
        -------
        Result
            Computed analysis result stored under ``PROVIDES``.
        """
        ...

    @classmethod
    def provides(cls) -> str:
        """Get the analysis key written by this pass.

        Returns
        -------
        str
            Stable provides key.
        """
        return cls.PROVIDES

    @classmethod
    def key(cls) -> AnalysisKey[Result]:
        """Get the typed key for retrieving this pass result."""
        return AnalysisKey(cls.PROVIDES)

    def _run(self, steps: Sequence[Pass]) -> Result:
        return self.run(steps)
