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
import sys
from collections.abc import Callable
from typing import Any, Generic, TypeAlias, TypeVar

if sys.version_info < (3, 12):
    from typing_extensions import override
else:
    from typing import override

from luna_model.model.model import Model
from luna_model.solution.sol import Solution

from .action_type import ActionType
from .analysis import AnalysisPass
from .base import BasePass
from .cache import AnalysisCache
from .meta_analysis import MetaAnalysisPass
from .transform import TransformationOutcome, TransformationPass

T = TypeVar("T")

AnalysisSignature: TypeAlias = Callable[[Model, AnalysisCache], T]

MetaAnalysisSignature: TypeAlias = Callable[[list[BasePass], AnalysisCache], T]

Outcome: TypeAlias = TransformationOutcome | tuple[Model, ActionType] | tuple[Model, ActionType, Any]
TransformationSignature: TypeAlias = Callable[
    [Model, AnalysisCache],
    Outcome,
]
BackwardsSignature: TypeAlias = Callable[[Solution, AnalysisCache], Solution]


def __identity_backwards(solution: Solution, _: AnalysisCache) -> Solution:
    return solution


class DynamicAnalysisPass(AnalysisPass, Generic[T]):
    def __init__(
        self,
        name: str,
        requires: list[str],
        func: AnalysisSignature[T],
    ) -> None:
        super().__init__()
        self._name = name
        self._requires = requires
        self._func = func

    @property
    def name(self) -> str:
        return self._name

    @property
    def requires(self) -> list[str]:
        return self._requires

    def __repr__(self) -> str:
        return f'FunctionAnalysis(name="{self.name}")'

    @override
    def run(self, model: Model, cache: AnalysisCache) -> T:
        return self._func(model, cache)

    def __call__(self, model: Model, cache: AnalysisCache) -> T:
        return self._func(model, cache)


class DynamicMetaAnalysisPass(MetaAnalysisPass, Generic[T]):
    def __init__(
        self,
        name: str,
        requires: list[str],
        func: MetaAnalysisSignature[T],
    ) -> None:
        super().__init__()
        self._name = name
        self._requires = requires
        self._func = func

    @property
    def name(self) -> str:
        return self._name

    @property
    def requires(self) -> list[str]:
        return self._requires

    def __repr__(self) -> str:
        return f'FunctionMetaAnalysis(name="{self.name}")'

    @override
    def run(self, passes: list[BasePass], cache: AnalysisCache) -> T:
        return self._func(passes, cache)

    def __call__(self, passes: list[BasePass], cache: AnalysisCache) -> T:
        return self._func(passes, cache)


class DynamicTransformationPass(TransformationPass):
    def __init__(
        self,
        name: str,
        requires: list[str],
        invalidates: list[str],
        func: TransformationSignature,
        backwards: BackwardsSignature,
    ) -> None:
        super().__init__()
        self._name = name
        self._requires = requires
        self._invalidates = invalidates
        self._func = func
        self._backwards = backwards  # type: ignore[reportAttributeAccessIssue]

    @property
    def name(self) -> str:
        return self._name

    @property
    def requires(self) -> list[str]:
        return self._requires

    @override
    def run(self, model: Model, cache: AnalysisCache) -> Outcome:
        return self._func(model, cache)

    @override
    def backwards(self, solution: Solution, cache: AnalysisCache) -> Solution:
        return self._backwards(solution, cache)

    def __call__(self, model: Model, cache: AnalysisCache) -> Outcome:
        return self._func(model, cache)

    def __repr__(self) -> str:
        return f'FunctionTransformation(name="{self.name}")'


def analyse(
    name: str | None = None, requires: list[str] | None = None
) -> Callable[[AnalysisSignature[T]], DynamicAnalysisPass[T]]:
    """Create an AnalysisPass from a function decorator.

    This decorator converts a regular function into an ``AnalysisPass`` that can be used
    in transformation pipelines. Analysis passes inspect models without modifying them,
    computing properties or metadata that other passes can use.

    Parameters
    ----------
    name : str, optional
        The name of the analysis pass. If not provided, uses the function name with
        underscores replaced by hyphens (e.g., ``my_analysis`` becomes ``my-analysis``).
    requires : list[str], optional
        List of analysis pass names that must run before this analysis. The results
        of required passes are available in the ``AnalysisCache``. Defaults to ``[]``.

    Returns
    -------
    Callable[[Callable[[Model, AnalysisCache], T]], DynamicAnalysisPass[T]]
        A decorator that transforms the decorated function into an ``AnalysisPass``.

    Examples
    --------
    Create a simple analysis pass:

    >>> from luna_model.transformation import analyse
    >>> @analyse(name="count-variables")
    ... def count_vars(model, cache):
    ...     return model.num_variables

    Notes
    -----
    The decorated function must have the signature::

        def my_analysis(model: Model, cache: AnalysisCache) -> Any: ...

    The return value is stored in the ``AnalysisCache`` under the pass's name
    and can be accessed by subsequent passes.
    """
    if requires is None:
        requires = []

    _T = TypeVar("_T")

    def _decorator(
        func: AnalysisSignature[_T],
    ) -> DynamicAnalysisPass[_T]:
        loc_name = name or func.__name__.replace("_", "-")

        return DynamicAnalysisPass(name=loc_name, requires=requires, func=func)

    return _decorator


def meta_analyse(
    name: str | None = None, requires: list[str] | None = None
) -> Callable[[MetaAnalysisSignature[T]], DynamicMetaAnalysisPass[T]]:
    """Create a MetaAnalysisPass from a function decorator.

    This decorator converts a regular function into a ``MetaAnalysisPass`` that analyzes
    the structure and properties of transformation pipelines themselves. Unlike regular
    analysis passes that inspect models, meta-analysis passes inspect the list of passes
    in a pipeline to optimize execution order, detect conflicts, or compute pipeline metadata.

    Parameters
    ----------
    name : str, optional
        The name of the meta-analysis pass. If not provided, uses the function name
        with underscores replaced by hyphens (e.g., ``my_meta`` becomes ``my-meta``).
    requires : list[str], optional
        List of analysis or meta-analysis pass names that must run before this pass.
        Required passes' results are available in the ``AnalysisCache``. Defaults to ``[]``.

    Returns
    -------
    Callable[[Callable[[list[BasePass], AnalysisCache], T]], DynamicMetaAnalysisPass[T]]
        A decorator that transforms the decorated function into a ``MetaAnalysisPass``.

    Examples
    --------
    Create a pass that counts transformation passes in a pipeline:

    >>> from luna_model.transformation import meta_analyse
    >>> @meta_analyse(name="count-transformations")
    ... def count_transforms(passes, cache):
    ...     from luna_model.transformation import TransformationPass
    ...
    ...     return sum(1 for p in passes if isinstance(p, TransformationPass))

    Notes
    -----
    The decorated function must have the signature::

        def my_meta_analysis(passes: list[BasePass], cache: AnalysisCache) -> Any: ...

    The return value is stored in the ``AnalysisCache`` under the pass's name.
    Meta-analysis passes are useful for pipeline optimization, validation, and debugging.
    """
    if requires is None:
        requires = []

    _T = TypeVar("_T")

    def _decorator(
        func: MetaAnalysisSignature[_T],
    ) -> DynamicMetaAnalysisPass[_T]:
        loc_name = name or func.__name__.replace("_", "-")

        return DynamicMetaAnalysisPass(name=loc_name, requires=requires, func=func)

    return _decorator


def transform(
    name: str | None = None,
    requires: list[str] | None = None,
    invalidates: list[str] | None = None,
    backwards: BackwardsSignature | None = None,
) -> Callable[[TransformationSignature], DynamicTransformationPass]:
    """Create a TransformationPass from a function decorator.

    This decorator converts a regular function into a ``TransformationPass`` that modifies
    models in transformation pipelines. Transformation passes can restructure models,
    add/remove constraints, change variable types, or perform other model modifications.

    Parameters
    ----------
    name : str, optional
        The name of the transformation pass. If not provided, uses the function name
        with underscores replaced by hyphens.
    requires : list[str], optional
        List of analysis pass names that must run before this transformation.
        Required analyses are available in the ``AnalysisCache``. Defaults to ``[]``.
    invalidates : list[str], optional
        List of analysis pass names whose results become invalid after this transformation.
        These analyses will be recomputed if needed later. Defaults to ``[]``.
    backwards : Callable[[Solution, AnalysisCache], Solution], optional
        Optional function to map solutions from the transformed model back to the
        original model's variable space. If not provided, solutions pass through unchanged.

    Returns
    -------
    Callable[[TransformationSignature], DynamicTransformationPass]
        A decorator that transforms the decorated function into a ``TransformationPass``.

    Examples
    --------
    Create a simple transformation:

    >>> from luna_model.transformation import transform, ActionType
    >>> @transform(name="scale-objective")
    ... def scale_obj(model, cache):
    ...     model.objective = model.objective * 2.0
    ...     return model, ActionType.DID_TRANSFORM

    Notes
    -----
    The decorated function must return one of:

    - ``TransformationOutcome`` (from ``luna_model.transformation``)
    - ``(Model, ActionType)`` tuple
    - ``(Model, ActionType, metadata)`` tuple

    The ``ActionType`` indicates what happened:

    - ``ActionType.DID_TRANSFORM``: Pass transformed the model
    - ``ActionType.DID_ANALYSIS_TRANSFORM``: Pass analyzed and transformed
    - ``ActionType.DID_NOTHING``: Pass did nothing

    The backwards function is crucial when transformations change the variable space
    (e.g., adding/removing variables, changing variable types). It ensures solutions
    from downstream solvers can be correctly interpreted in the original model's context.
    """
    if requires is None:
        requires = []
    if invalidates is None:
        invalidates = []

    if backwards is None:
        backwards = __identity_backwards

    def _decorator(func: TransformationSignature) -> DynamicTransformationPass:
        loc_name = name or func.__name__.replace("_", "-")

        return DynamicTransformationPass(
            name=loc_name,
            requires=requires,
            invalidates=invalidates,
            func=func,
            backwards=backwards,
        )

    return _decorator


__all__ = ["analyse", "transform"]
