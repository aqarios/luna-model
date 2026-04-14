from collections.abc import Callable
from typing import Generic, TypeAlias, TypeVar

from luna_model.model.model import Model
from luna_model.solution.sol import Solution
from luna_model.transformation.composite import CompositePass
from luna_model.transformation.context import PassContext
from luna_model.transformation.decorators.transformation import (
    A,
    BackwardSignature,
    __identity_backward,
    _ArtifactEnvelope,
    _validate_backward,
)

R = TypeVar("R")

CompositeSignature: TypeAlias = Callable[[Model, PassContext], tuple[Model, A, R]]


class _DynamicCompositePass(CompositePass, Generic[A, R]):
    _name: str
    _requires: list[str]
    _invalidates: list[str]
    _forward_f: CompositeSignature
    _backward_f: BackwardSignature

    def __init__(
        self,
        name: str,
        requires: list[str],
        invalidates: list[str],
        forward: CompositeSignature,
        backward: BackwardSignature,
    ) -> None:
        super().__init__()
        self._name = name
        self._requires = requires
        self._invalidates = invalidates
        self._forward_f = forward
        self._backward_f = backward

    def name(self) -> str:
        return self._name

    def requires(self) -> list[str]:
        return self._requires

    def invalidates(self) -> list[str]:
        return self._invalidates

    def forward(self, model: Model, ctx: PassContext) -> tuple[Model, _ArtifactEnvelope[A], R]:
        result: tuple[Model, A, R] = self._forward_f(model, ctx)
        model, artifact, res = result
        return model, _ArtifactEnvelope.from_parts(artifact, self._backward_f), res

    @classmethod
    def backward(cls, artifact: _ArtifactEnvelope[A], solution: Solution) -> Solution:
        return artifact.backward_fn(artifact.artifact, solution)


def composite(
    name: str | None = None,
    requires: list[str] | None = None,
    provides: str | None = None,
    invalidates: list[str] | None = None,
    backward: BackwardSignature[A] | None = None,
) -> Callable[[CompositeSignature[A, R]], _DynamicCompositePass[A, R]]:
    """Create a CompositePass from a function decorator.

    This decorator converts a regular function into a ``CompositePass`` that modifies
    and analyzes models in transformation pipelines. Composite passes can restructure
    models, add/remove constraints, change variable types, or perform other model
    modifications and analysis.

    !!! warning "Disclaimer"
        Dynamic artifact/backward resolution is restricted to an import allowlist.
        By default, only modules under ``luna_model.`` are allowed.
        Use ``register_allowed_import_prefix(...)`` to allow custom plugin namespaces.

    Parameters
    ----------
    name : str, optional
        The name of the composite pass. If not provided, uses the function name
        with underscores replaced by hyphens.
    requires : list[str], optional
        List of pass names that must run before this composite pass. Defaults to ``[]``.
    provides : str, optional
        The key for the result the analysis pass provides. If not specified, uses the function
        name with underscores replaced by hyphens prefixed with ``decorated_analysis::``
        (e.g., ``my_analysis`` becomes ``decorated_analysis::my-analysis``).
    invalidates : list[str], optional
        List of analysis pass names whose results become invalid after this composite pass. Defaults to ``[]``.
    backward : Callable[[A, Solution], Solution], optional
        Optional function to map solutions from the transformed model back to the
        original model's variable space. If not provided, solutions pass through unchanged.

    Returns
    -------
    Callable[[CompositeSignature[A, R]], _DynamicCompositePass[A, R]]
        A decorator that transforms the decorated function into a ``CompositePass``.
        The generic ``A`` is the artifact produced by this composite pass and the
        generic ``R`` the analysis result produced by this composite pass.

    Examples
    --------
    Create a simple composite:

    >>> from luna_model.transformation import composite
    >>> @composite(name="scale-objective")
    ... def scale_obj(model: Model, ctx: PassContext) -> tuple[model, NothingArtifact, float]:
    ...     model.objective = model.objective * 2.0
    ...     return model, NothingArtifact(), model.num_variables

    Notes
    -----
    Dynamic artifact/backward resolution is restricted to an import allowlist.
    By default, only modules under ``luna_model.`` are allowed.
    Use ``register_allowed_import_prefix(...)`` to allow custom plugin namespaces.

    The decorated function must return:

    - ``(Model, Artifact, Result)`` tuple

    The backwards function is crucial when transformations change the variable space
    (e.g., adding/removing variables, changing variable types). It ensures solutions
    from downstream solvers can be correctly interpreted in the original model's context.
    """
    if requires is None:
        requires = []
    if invalidates is None:
        invalidates = []

    if backward is None:
        backward = __identity_backward
    else:
        _validate_backward(backward)

    def _decorator(forward: CompositeSignature[A, R]) -> _DynamicCompositePass[A, R]:
        loc_name = name or forward.__name__.replace("_", "-")

        class _TheComposite(_DynamicCompositePass):
            PROVIDES = f"decorated_composite::{loc_name}" if provides is None else provides

        return _TheComposite(
            name=loc_name,
            requires=requires,
            invalidates=invalidates,
            forward=forward,
            backward=backward,
        )

    return _decorator
