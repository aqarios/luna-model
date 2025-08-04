"""Decorators."""

from collections.abc import Callable
from typing import Any, Generic, TypeVar, override

from . import Model, Solution
from .transformations import (
    ActionType,
    AnalysisCache,
    AnalysisPass,
    BasePass,
    MetaAnalysisPass,
    TransformationOutcome,
    TransformationPass,
)

T = TypeVar("T")


type AnalysisSignature[T] = Callable[[Model, AnalysisCache], T]

type MetaAnalysisSignature[T] = Callable[[list[BasePass], AnalysisCache], T]

type Outcome = (
    TransformationOutcome | tuple[Model, ActionType] | tuple[Model, ActionType, Any]
)
type TransformationSignature = Callable[
    [Model, AnalysisCache],
    Outcome,
]
type BackwardsSignature = Callable[[Solution, AnalysisCache], Solution]


def __identity_backwards(solution: Solution, _: AnalysisCache) -> Solution:
    return solution


class DynamicAnalysisPass(AnalysisPass, Generic[T]):
    def __init__(
        self,
        name: str,
        requires: list[str],
        func: AnalysisSignature[T],
    ) -> None:
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
        self._name = name
        self._requires = requires
        self._invalidates = invalidates
        self._func = func
        self._backwards = backwards

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
    """Create an AnalysisPass instance from a function.

    Parameters
    ----------
    name: str | None
        The name of the analysis pass. If no name provided, uses the function name.
    requires: list[str] | None
        List of required analysis passes (defaults to empty list)

    Returns
    -------
    Callable[[Callable[[Model, AnalysisCache], Any]], AnalysisPass]
        An instance of a dynamically created AnalysisPass subclass
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
    """Create an TransformationPass instance from a function.

    Parameters
    ----------
    name: str | None = None
        The name of the analysis pass. If no name provided, uses the function name.
    requires: list[str] | None = None
        List of required analysis passes (defaults to empty list)
    invalidates: list[str] | None = None
        List of analysis results to invalidate (defaults to empty list)
    backwards: BackwardsSignature | None = None,
        Solution backwards mapping function. If none provided, pass solution upstream
        without modification.

    Returns
    -------
    Callable[[TransformationSignature], DynamicTransformationPass]
        An instance of a dynamically created TransformationPass subclass
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
