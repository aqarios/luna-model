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
from luna_model._lm import PyIntegerToBinaryPass
from luna_model.transformation.transform import ConcreteTransformationPass


class IntegerToBinaryPass(ConcreteTransformationPass):
    """Convert Integer variables to a binary representation.

    Transform the variables of type integer to be represented by binary typed
    variables.

    Examples
    --------
    >>> from luna_model import Model, Vtype
    >>> from luna_model.transformation import PassManager
    >>> from luna_model.transformation.passes import BinarySpinPass
    >>> model = Model()
    >>> x = model.add_variable("x", vtype=Vtype.INTEGER)
    >>> y = model.add_variable("y", vtype=Vtype.BINARY)
    >>> model.objective = x * y + x - 2 * y
    >>> pm = PassManager([IntegerToBinaryPass()])
    >>> ir = pm.run(model)
    >>> ir.model
    """

    def __init__(self) -> None:
        super().__init__(base=PyIntegerToBinaryPass())
