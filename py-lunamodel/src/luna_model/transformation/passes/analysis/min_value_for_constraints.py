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

from typing import Protocol, Self, runtime_checkable

from luna_model._lm import PyMinValueForConstraintAnalysis
from luna_model.transformation.passes.analysis.builtin import BuiltinAnalysis


@runtime_checkable
class MinConstraintValues(Protocol):
    """Protocol for MinValueForConstraints information stored in the analysis cache.

    This protocol defines the interface for accessing min value for constraints values
    computed during model analysis.
    """

    @property
    def vals(self) -> dict[str, float]:
        """Get the minimum values possible for the constraints.

        Returns
        -------
        dict[str, float]
            The minimum possible value for all constraints.
        """
        ...


class MinValueForConstraintAnalysis(PyMinValueForConstraintAnalysis, BuiltinAnalysis[MinConstraintValues]):
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
    >>> output = pm.run(model)
    >>> output.cache.require_analysis(MinValueForConstraintAnalysis.key()).vals
    {'my-constraint': -5.0}
    """

    def __new__(cls) -> Self:
        """Create a new min value for constraint analysis pass."""
        return super().__init__(cls)
