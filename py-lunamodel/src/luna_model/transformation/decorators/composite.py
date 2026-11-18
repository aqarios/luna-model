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

from collections.abc import Callable, Iterable
from hashlib import sha256
from typing import Generic, TypeAlias, TypeVar, overload

from luna_model.model.model import Model
from luna_model.solution.sol import Solution
from luna_model.transformation.artifact import TransformationPassArtifact
from luna_model.transformation.composite import CompositePass
from luna_model.transformation.context import PassContext
from luna_model.transformation.decorators.transformation import (
    BackwardSignature,
    __identity_backward,
    _ArtifactEnvelope,
    _model_parts,
    _validate_backward,
    _warn_invalidates,
)
from luna_model.transformation.model_part import ModelPart

A = TypeVar("A", bound=TransformationPassArtifact)
R = TypeVar("R")

CompositeSignature: TypeAlias = Callable[[Model, PassContext], tuple[Model, A, R]]


class _DynamicCompositePass(CompositePass[_ArtifactEnvelope, R], Generic[A, R]):
    _name: str
    _requires: list[str]
    _modifies: set[ModelPart]
    _depends_on: set[ModelPart]
    _forward_f: CompositeSignature[A, R]
    _backward_f: BackwardSignature[A]

    def __init__(  # noqa: PLR0913
        self,
        name: str,
        requires: list[str],
        modifies: set[ModelPart],
        depends_on: set[ModelPart],
        forward: CompositeSignature[A, R],
        backward: BackwardSignature[A],
    ) -> None:
        super().__init__()
        self._name = name
        self._requires = requires
        self._modifies = modifies
        self._depends_on = depends_on
        self._forward_f = forward
        self._backward_f = backward

    def name(self) -> str:
        return self._name

    def requires(self) -> list[str]:
        return self._requires

    def modifies(self) -> set[ModelPart]:
        return self._modifies

    def depends_on(self) -> set[ModelPart]:
        return self._depends_on

    def forward(self, model: Model, ctx: PassContext) -> tuple[Model, _ArtifactEnvelope[A], R]:
        model, artifact, res = self._forward_f(model, ctx)
        envelope = _ArtifactEnvelope.from_parts(artifact, self._backward_f)
        return model, envelope, res

    @classmethod
    def backward(cls, artifact: _ArtifactEnvelope[A], solution: Solution) -> Solution:
        return artifact.backward_fn(artifact.artifact, solution)


@overload
def composite(
    name: str | None = ...,
    requires: list[str] | None = ...,
    provides: str | None = ...,
    invalidates: list[str] | None = ...,
    *,
    modifies: Iterable[ModelPart] | None = ...,
    depends_on: Iterable[ModelPart] | None = ...,
) -> Callable[[CompositeSignature[A, R]], _DynamicCompositePass[A, R]]: ...


@overload
def composite(
    name: str | None = ...,
    requires: list[str] | None = ...,
    provides: str | None = ...,
    invalidates: list[str] | None = ...,
    *,
    backward: BackwardSignature[A],
    modifies: Iterable[ModelPart] | None = ...,
    depends_on: Iterable[ModelPart] | None = ...,
) -> Callable[[CompositeSignature[A, R]], _DynamicCompositePass[A, R]]: ...


def composite(  # noqa: PLR0913
    name: str | None = None,
    requires: list[str] | None = None,
    provides: str | None = None,
    invalidates: list[str] | None = None,
    backward: BackwardSignature[A] | None = None,
    *,
    modifies: Iterable[ModelPart] | None = None,
    depends_on: Iterable[ModelPart] | None = None,
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
        Deprecated and ignored; use ``modifies``.
    backward : Callable[[A, Solution], Solution], optional
        Optional function to map solutions from the transformed model back to the
        original model's variable space. If not provided, solutions pass through unchanged.
    modifies : Iterable[ModelPart], optional
        Model parts the pass may modify; analyses depending on any of them are invalidated
        after it runs. Defaults to every part.
    depends_on : Iterable[ModelPart], optional
        Model parts the provided result is computed from. Defaults to every part.

    Returns
    -------
    Callable[[CompositeSignature[A, R]], _DynamicCompositePass[A, R]]
        A decorator that transforms the decorated function into a ``CompositePass``.
        The generic ``A`` is the artifact produced by this composite pass and the
        generic ``R`` the analysis result produced by this composite pass.

    Examples
    --------
    Create a simple composite:

    >>> from luna_model.transformation import composite, NothingArtifact
    >>> @composite(name="scale-objective")
    ... def scale_obj(model: Model, ctx: PassContext) -> tuple[Model, NothingArtifact, float]:
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
    _warn_invalidates(invalidates)

    if backward is None:
        backward = __identity_backward
    else:
        _validate_backward(backward)

    def _decorator(forward: CompositeSignature[A, R]) -> _DynamicCompositePass[A, R]:
        loc_name = name or forward.__name__.replace("_", "-")
        provides_key = f"decorated_composite::{loc_name}" if provides is None else provides

        seed = f"{forward.__module__}:{forward.__qualname__}:{loc_name}:{provides_key}"
        cls_name = f"_DecoratedComposite_{sha256(seed.encode()).hexdigest()}"

        the_composite = type(
            cls_name,
            (_DynamicCompositePass,),
            {
                "PROVIDES": provides_key,
                "__module__": __name__,
                "__qualname__": cls_name,
            },
        )
        globals()[cls_name] = the_composite  # critical: makes it importable by module attr lookup

        return the_composite(
            name=loc_name,
            requires=requires,
            modifies=_model_parts(modifies),
            depends_on=_model_parts(depends_on),
            forward=forward,
            backward=backward,
        )

    return _decorator
