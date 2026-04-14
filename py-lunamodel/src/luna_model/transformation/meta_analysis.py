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
from typing import Generic, Protocol, TypeVar, runtime_checkable

from luna_model._lm import PyMetaAnalysisPass
from luna_model.transformation.key import AnalysisKey

Result = TypeVar("Result")


@runtime_checkable
class StepView(Protocol):
    """Protocol describing one pipeline step visible to a ``MetaAnalysisPass``.

    A step is represented by one of the nested protocol variants below. Meta
    analysis implementations can inspect these views.
    """

    @runtime_checkable
    class Transform(Protocol):
        """View of a transformation step."""

        @property
        def name(self) -> str:
            """Transformation pass name."""
            ...

        @property
        def requires(self) -> list[str]:
            """Required keys that must be satisfied before this step can run."""
            ...

        @property
        def invalidates(self) -> list[str]:
            """Keys invalidated after this step executes."""
            ...

    @runtime_checkable
    class Analysis(Protocol):
        """View of an analysis step."""

        @property
        def name(self) -> str:
            """Analysis pass name."""
            ...

        @property
        def provides(self) -> str:
            """Key written by this analysis step."""
            ...

        @property
        def requires(self) -> list[str]:
            """Required keys that must be available before this step runs."""
            ...

    @runtime_checkable
    class MetaAnalysis(Protocol):
        """View of a meta-analysis step."""

        @property
        def name(self) -> str:
            """Meta-analysis pass name."""
            ...

        @property
        def provides(self) -> str:
            """Key written by this meta-analysis step."""
            ...

    @runtime_checkable
    class ControlFlow(Protocol):
        """View of a control-flow step."""

        @property
        def name(self) -> str:
            """Control-flow pass name."""
            ...

        @property
        def requires(self) -> list[str]:
            """Required keys that must be satisfied before this step can run."""
            ...

        @property
        def provides(self) -> list[str]:
            """Keys this control-flow step may provide."""
            ...

        @property
        def invalidates(self) -> list[str]:
            """Keys invalidated after this step executes."""
            ...

    @runtime_checkable
    class Composite(Protocol):
        """View of a composite step (transform + analysis result)."""

        @property
        def name(self) -> str:
            """Composite pass name."""
            ...

        @property
        def requires(self) -> list[str]:
            """Required keys that must be satisfied before this step can run."""
            ...

        @property
        def provides(self) -> str:
            """Key written by the analysis part of this composite step."""
            ...

        @property
        def invalidates(self) -> list[str]:
            """Keys invalidated after this step executes."""
            ...

    @runtime_checkable
    class Pipeline(Protocol):
        """View of a nested pipeline step."""

        @property
        def name(self) -> str:
            """Nested pipeline name."""
            ...

        @property
        def requires(self) -> list[str]:
            """Union of required keys across nested steps."""
            ...

        @property
        def provides(self) -> list[str]:
            """Union of keys provided across nested steps."""
            ...

        @property
        def invalidates(self) -> list[str]:
            """Union of keys invalidated across nested steps."""
            ...

        @property
        def nested(self) -> list[StepView]:
            """Direct child steps in this nested pipeline."""
            ...


class MetaAnalysisPass(PyMetaAnalysisPass, Generic[Result]):
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

    def _run(self, steps: list[StepView]) -> Result:
        return self.run(steps)
