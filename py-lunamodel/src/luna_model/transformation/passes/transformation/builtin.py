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

from typing import Generic, Protocol, TypeVar, cast

from luna_model._lm import PyModel, PyPassContext, PySolution
from luna_model.model.model import Model
from luna_model.solution.sol import Solution
from luna_model.transformation.artifact import TransformationPassArtifact
from luna_model.transformation.context import PassContext

Artifact = TypeVar("Artifact", bound=TransformationPassArtifact)


class _BuiltinSuper(Protocol[Artifact]):
    @classmethod
    def backward(cls, artifact: Artifact, solution: PySolution) -> PySolution: ...
    def name(self) -> str: ...
    def forward(self, model: PyModel, ctx: PyPassContext) -> tuple[Model, Artifact]: ...
    def requires(self) -> list[str]: ...
    def invalidates(self) -> list[str]: ...


class BuiltinTransformation(Generic[Artifact]):
    """A builtin transformation pass.

    Transformation passes apply changes to models and can also convert
    solutions backwards to match the input representation.
    """

    def __init__(self, *args: *tuple, **kwargs: dict) -> None:
        super().__init__(*args, **kwargs)

    def name(self) -> str:
        """
        Get the unique identifier for this pass.

        Returns
        -------
        str
            The unique pass name.
        """
        sup = cast("_BuiltinSuper[Artifact]", super())
        return sup.name()

    def forward(self, model: Model, ctx: PassContext) -> tuple[Model, Artifact]:
        """
        Run/Execute this transformation pass.

        Parameters
        ----------
        model : Model
            The model to transform.
        ctx : PassContext
            Context for this pass providing read-access to the analysis cache.

        Returns
        -------
        tuple[Model, Artifact]
            The transformation result containing the model and the artifact
            used for running the backward pass.
        """
        sup = cast("_BuiltinSuper[Artifact]", super())
        result: tuple[PyModel, Artifact] = sup.forward(model._m, ctx._c)
        model, artifact = result
        return Model._from_pym(model), artifact

    @classmethod
    def backward(cls, artifact: Artifact, solution: Solution) -> Solution:
        """
        Apply the back transformation to the given solution.

        Parameters
        ----------
        artifact : Artifact
            The artifact produced by the forward execution.
        solution : Solution
            The solution to transform back to a representation fitting the original
            (input) model transformed by the forward method.

        Returns
        -------
        Solution
            A solution object representing a solution to the original problem passed
            to this TransformationPass' forward method.
        """
        sup = cast("_BuiltinSuper[Artifact]", super())
        return Solution._from_pys(sup.backward(artifact, solution._s))

    def requires(self) -> list[str]:
        """
        List of passes that must run before this pass.

        Returns
        -------
        list[str]
            Pass names that must execute first, or empty list if no dependencies.
        """
        sup = cast("_BuiltinSuper[Artifact]", super())
        return sup.requires()

    def invalidates(self) -> list[str]:
        """
        Get a list of passes that are invalidated by this pass.

        Returns
        -------
        list of str
            Names of passes whose results become invalid after this pass runs.
        """
        sup = cast("_BuiltinSuper[Artifact]", super())
        return sup.invalidates()
