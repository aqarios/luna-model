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

from typing import Protocol, Self

from luna_model._lm import PyEqualityConstraintsToQuadraticPenaltyPass
from luna_model.transformation.artifact import TransformationPassArtifact
from luna_model.transformation.passes.transformation.builtin import BuiltinTransformation


class EqualityConstraintsToQuadraticPenaltyPassArtifact(TransformationPassArtifact, Protocol):
    """Artifact output of the EqualityConstraintsToQuadraticPenaltyPass.

    This protocol defines the interface for accessing information about EqualityConstraintsToQuadraticPenaltyPass
    transformations, including the source and target variable types and the mapping between variable names.
    """


class EqualityConstraintsToQuadraticPenaltyPass(
    PyEqualityConstraintsToQuadraticPenaltyPass,
    BuiltinTransformation[EqualityConstraintsToQuadraticPenaltyPassArtifact],
):
    """Move all equality constraints to the model's objective as a quadratic penalties.

    Converts the model by moving all equality constraints to the objective as quadratic penalties.
    Requires the `MaxBiasAnalysis` pass to be executed before this pass.

    Less-equal (`<=`) or greater-equal (`>=`) constraints are not respected by this transformation
    and have to be handled before this pass using, e.g., the `GeToLeConstraintsPass` and the
    `LeToEqConstraintsPass`.

    Notes
    -----
    This pass requires the `MaxBiasAnalysis` pass to be executed before this pass.

    Parameters
    ----------
    penalty_scaling : float
        The factor used to scale the penalties with, default 10.0.

    Examples
    --------
    >>> from luna_model import Model, Vtype, Sense
    >>> from luna_model.transformation import PassManager
    >>> from luna_model.transformation.passes import MaxBiasAnalysis
    >>> from luna_model.transformation.passes import EqualityConstraintsToQuadraticPenaltyPass
    >>> model = Model(sense=Sense.MAX)
    >>> x = model.add_variable("x", vtype=Vtype.BINARY)
    >>> y = model.add_variable("y", vtype=Vtype.BINARY)
    >>> model.objective = x * y + x - 2 * y
    >>> model.constraints += x + y == 0
    >>> pm = PassManager(
    ...     [
    ...         MaxBiasAnalysis(),
    ...         EqualityConstraintsToQuadraticPenaltyPass(),
    ...     ]
    ... )
    >>> output = pm.run(model)
    >>> print(output.model.objective)
    41 x y + 21 x + 18 y
    """

    def __new__(cls, penalty_scaling: float = 10.0) -> Self:
        """Create a new equality constraints to quadratic penalty pass.

        Parameters
        ----------
        penalty_scaling : float
            The factor used to scale the penalties with, default 10.0.
        """
        return super().__new__(cls, penalty_scaling)

    @property
    def penalty_scaling(self) -> float:
        """Get the penalty scaling factor."""
        return super().penalty_scaling
