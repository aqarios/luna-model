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
from luna_model._lm import PyLeToEqConstraintsPass
from luna_model.transformation.transform import ConcreteTransformationPass


class LeToEqConstraintsPass(ConcreteTransformationPass):
    """Convert the model's constraints by chaning all less-equal (`<=`) constraints to equality (`==`) constraints.

    Examples
    --------
    >>> from luna_model import Model, Vtype, Sense
    >>> from luna_model.transformation import PassManager
    >>> from luna_model.transformation.passes import MinValueForConstraintAnalysis
    >>> from luna_model.transformation.passes import LeToEqConstraintsPass
    >>> model = Model(sense=Sense.MAX)
    >>> x = model.add_variable("x", vtype=Vtype.BINARY)
    >>> y = model.add_variable("y", vtype=Vtype.BINARY)
    >>> model.objective = x * y + x - 2 * y
    >>> model.constraints += x + y <= 3, "le_constr"
    >>> pm = PassManager([MinValueForConstraintAnalysis(), LeToEqConstraintsPass()])
    >>> ir = pm.run(model)
    >>> ir.model.constraints["le_constr"]
    x + y + slack_le_constr == 0
    >>> print(ir.model.get_variable("slack_le_constr"))
    slack_le_constr: Integer(lower=0, upper=3)
    """

    def __init__(self) -> None:
        super().__init__(base=PyLeToEqConstraintsPass())
