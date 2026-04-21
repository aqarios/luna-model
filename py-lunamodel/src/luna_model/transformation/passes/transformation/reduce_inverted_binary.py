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

from luna_model._lm import PyReduceInvertedBinaryPass
from luna_model.transformation.artifact import TransformationPassArtifact
from luna_model.transformation.passes.transformation.builtin import BuiltinTransformation


class ReduceInvertedBinaryPassArtifact(TransformationPassArtifact, Protocol):
    """Artifact output of the ReduceInvertedBinaryPass.

    This protocol defines the interface for accessing information about ReduceInvertedBinaryPass transformations,
    including the source and target variable types and the mapping between variable names.
    """


class ReduceInvertedBinaryPass(PyReduceInvertedBinaryPass, BuiltinTransformation[ReduceInvertedBinaryPassArtifact]):
    """Convert the model's constraints by chaning all greater-equal (`>=`) constraints to less-equal (`<=`) constraints.

    Examples
    --------
    >>> from luna_model import Model, Vtype, Sense
    >>> from luna_model.transformation import PassManager
    >>> from luna_model.transformation.passes import ReduceInvertedBinaryPass
    >>> model = Model(sense=Sense.MAX)
    >>> x = model.add_variable("x", vtype=Vtype.BINARY)
    >>> y = model.add_variable("y", vtype=Vtype.BINARY)
    >>> model.objective = x * ~y + x * y + x * 2
    >>> pm = PassManager([ReduceInvertedBinaryPass()])
    >>> output = pm.run(model)
    >>> print(output.model.objective)
    2 x y + x
    """

    def __new__(cls) -> Self:
        """Create a new ge to le constraints pass."""
        return super().__new__(cls)
