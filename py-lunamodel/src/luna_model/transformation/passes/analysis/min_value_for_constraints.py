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
from luna_model._lm import PyMinValueForConstraintsAnalysis
from luna_model.transformation.analysis import ConcreteAnalysisPass


class MinValueForConstraintAnalysis(ConcreteAnalysisPass):
    """Analysis pass that computes the min value possible for all constraints.

    This analysis pass computes the minimal value possible for all constraints
    of the input model.

    Examples
    --------
    >>> from luna_model import Model, Vtype
    >>> from luna_model.transformation import PassManager
    >>> from luna_model.transformation.passes import CheckModelSpecsAnalysis
    >>> model = Model()
    >>> x = model.add_variable("x", vtype=Vtype.BINARY)
    >>> y = model.add_variable("y", vtype=Vtype.BINARY)
    >>> model.objective = x * y + x - 2 * y
    >>> model.constraints += -5 * x + y <= 2, "my-constraint"
    >>> pm = PassManager([MinValueForConstraintAnalysis()])
    >>> ir = pm.run(model)
    >>> ir.cache["min-value-for-constraint"].vals
    {'my-constraint': -5.0}
    """

    def __init__(self) -> None:
        super().__init__(base=PyMinValueForConstraintsAnalysis())
