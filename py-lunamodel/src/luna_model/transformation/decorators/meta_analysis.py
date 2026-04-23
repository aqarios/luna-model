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

from collections.abc import Callable, Sequence
from typing import Generic, TypeAlias, TypeVar

from luna_model.transformation.meta_analysis import MetaAnalysisPass
from luna_model.transformation.typing import Pass

R = TypeVar("R")

MetaAnalysisSignature: TypeAlias = Callable[[Sequence[Pass]], R]


class _DynamicMetaAnalysisPass(MetaAnalysisPass[R], Generic[R]):
    _name: str
    _run_f: MetaAnalysisSignature[R]

    def __init__(self, name: str, run: MetaAnalysisSignature[R]) -> None:
        self._name = name
        self._run_f = run

    def name(self) -> str:
        return self._name

    def run(self, steps: Sequence[Pass]) -> R:
        return self._run_f(steps)


def meta_analyze(
    name: str | None = None, provides: str | None = None
) -> Callable[[MetaAnalysisSignature[R]], _DynamicMetaAnalysisPass[R]]:
    """Create a ``MetaAnalysisPass`` from a function.

    This decorator converts a plain Python function into a ``MetaAnalysisPass``.
    Meta-analysis passes inspect the upcoming pipeline steps and compute metadata
    that later passes can consume.

    Parameters
    ----------
    name : str, optional
        Pass name. If omitted, uses the function name with underscores replaced
        by hyphens.
    provides : str, optional
        Analysis key produced by this pass. If omitted, defaults to
        ``decorated_meta_analysis::<name>``.

    Returns
    -------
    Callable[[MetaAnalysisSignature[R]], _DynamicMetaAnalysisPass[R]]
        Decorator that returns a concrete ``MetaAnalysisPass`` instance.

    Examples
    --------
    Create a simple meta-analysis pass:

    >>> from luna_model.transformation import meta_analyze, TransformationPass
    >>> @meta_analyze(name="count-transforms")
    ... def count_transforms(steps):
    ...     return sum(1 for s in steps if isinstance(s, TransformationPass))

    Notes
    -----
    The decorated function must have the signature:

        ``def my_meta_analysis(steps: Sequence[Pass]) -> R``

    The return value is stored under ``provides`` and can be retrieved from
    ``PassContext`` by later passes.
    """

    def _decorator(
        func: MetaAnalysisSignature[R],
    ) -> _DynamicMetaAnalysisPass[R]:
        loc_name = name or func.__name__.replace("_", "-")

        class _TheMetaAnalysis(_DynamicMetaAnalysisPass):
            PROVIDES = f"decorated_meta_analysis::{loc_name}" if provides is None else provides

        return _TheMetaAnalysis(
            name=loc_name,
            run=func,
        )

    return _decorator
