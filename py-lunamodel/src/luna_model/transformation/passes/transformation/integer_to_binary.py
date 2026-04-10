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

from typing import Protocol

from luna_model._lm import PyIntegerToBinaryPass
from luna_model.transformation.artifact import TransformationPassArtifact
from luna_model.transformation.passes.transformation.builtin import BuiltinTransformation


class IntegerToBinaryPassArtifact(TransformationPassArtifact, Protocol):
    """Artifact output of the IntegerToBinaryPassArtifact.

    This protocol defines the interface for accessing information about IntegerToBinaryPassArtifact transformations,
    including the source and target variable types and the mapping between variable names.
    """

    @property
    def varmap(self) -> dict[str, dict[str, int]]:
        """Get the variable name mapping from old to new names with extra info used for reconstruction.

        Returns
        -------
        dict[str, dict[str, int]]
            Dictionary mapping old variable names to new variable names with extra info.
        """
        ...

    @property
    def offsetmap(self) -> dict[str, int]:
        """Get the offset mapping used in reconstruction.

        Returns
        -------
        dict[str, int]
            Dictionary mapping used in reconstruction.
        """
        ...


class IntegerToBinaryPass(BuiltinTransformation[IntegerToBinaryPassArtifact], PyIntegerToBinaryPass):
    """Convert integer variables to a binary representation.

    Transform the variables of type integer to be represented by binary typed
    variables.

    Examples
    --------
    >>> from luna_model import Model, Vtype
    >>> from luna_model.transformation import PassManager
    >>> from luna_model.transformation.passes import BinarySpinPass
    >>> model = Model()
    >>> x = model.add_variable("x", vtype=Vtype.INTEGER, lower=0, upper=3)
    >>> y = model.add_variable("y", vtype=Vtype.BINARY)
    >>> model.objective = x * y + x - 2 * y
    >>> pm = PassManager([IntegerToBinaryPass()])
    >>> output = pm.run(model)
    >>> print(output.model.objective)
    y x_b0 + 2 y x_b1 - 2 y + x_b0 + 2 x_b1
    """
