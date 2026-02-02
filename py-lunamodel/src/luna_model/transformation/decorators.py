# Copyright 2025 Aqarios GmbH
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
"""Decorators."""

from collections.abc import Callable
from typing import Any, Generic, TypeAlias, TypeVar

from typing_extensions import override

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
    >>> 
    >>> @analyse(name="count-variables")
    ... def count_vars(model, cache):
    ...     return model.num_variables()

    Create an analysis that depends on another:

    >>> @analyse(name="variable-stats", requires=["count-variables"])
    ... def analyze_variables(model, cache):
    ...     count = cache["count-variables"]
    ...     return {
    ...         "count": count,
    ...         "binary": model.specs().num_binary,
    ...         "integer": model.specs().num_integer,
    ...     }

    Use in a PassManager:

    >>> from luna_model.transformation import PassManager
    >>> 
    >>> pm = PassManager([count_vars, analyze_variables])
    >>> result = pm.run(model)
    >>> stats = result.cache["variable-stats"]
    >>> print(f"Total variables: {stats['count']}")

    Notes
    -----
    The decorated function must have the signature::

        def my_analysis(model: Model, cache: AnalysisCache) -> Any:
            ...

    The return value is stored in the ``AnalysisCache`` under the pass's name
    and can be accessed by subsequent passes.

    See Also
    --------
    transform : Decorator for creating transformation passes
    meta_analyse : Decorator for meta-analysis passes
    AnalysisPass : Base class for analysis passes
    AnalysisCache : Storage for analysis results
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
    """Create an MetaAnalysisPass instance from a function.

    Parameters
    ----------
    name: str | None
        The name of the analysis pass. If no name provided, uses the function name.
    requires: list[str] | None
        List of required analysis passes (defaults to empty list)

    Returns
    -------
    Callable[[Callable[[list[BasePass], AnalysisCache], Any]], MetaAnalysisPass]
        An instance of a dynamically created AnalysisPass subclass
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
    >>> 
    >>> @transform(name="scale-objective")
    ... def scale_obj(model, cache):
    ...     model.objective = model.objective * 2.0
    ...     return model, ActionType.MODIFIED

    Create a transformation with backwards mapping:

    >>> def backwards_map(solution, cache):
    ...     # Map solution back to original variable space
    ...     return solution
    >>> 
    >>> @transform(
    ...     name="add-slack-variables",
    ...     requires=["constraint-analysis"],
    ...     invalidates=["variable-count"],
    ...     backwards=backwards_map
    ... )
    ... def add_slack(model, cache):
    ...     # Add slack variables to constraints
    ...     for i, constraint in enumerate(model.constraints):
    ...         slack = model.add_variable(f"slack_{i}", vtype=Vtype.REAL, lower=0)
    ...         # Modify constraint to include slack
    ...     return model, ActionType.MODIFIED

    Conditional transformation based on analysis:

    >>> @transform(name="simplify-if-small", requires=["variable-count"])
    ... def conditional_simplify(model, cache):
    ...     var_count = cache["variable-count"]
    ...     if var_count < 100:
    ...         # Apply aggressive simplification
    ...         model = simplify_model(model)
    ...         return model, ActionType.MODIFIED
    ...     return model, ActionType.UNCHANGED

    Notes
    -----
    The decorated function must return one of:

    - ``TransformationOutcome`` (from ``luna_model.transformation``)
    - ``(Model, ActionType)`` tuple
    - ``(Model, ActionType, metadata)`` tuple

    The ``ActionType`` indicates what happened:

    - ``ActionType.UNCHANGED``: Model was not modified
    - ``ActionType.MODIFIED``: Model was modified in-place
    - ``ActionType.REPLACED``: A new model instance was returned

    The backwards function is crucial when transformations change the variable space
    (e.g., adding/removing variables, changing variable types). It ensures solutions
    from downstream solvers can be correctly interpreted in the original model's context.

    See Also
    --------
    analyse : Decorator for creating analysis passes
    TransformationPass : Base class for transformation passes
    ActionType : Enumeration of transformation actions
    TransformationOutcome : Result type for transformations
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
