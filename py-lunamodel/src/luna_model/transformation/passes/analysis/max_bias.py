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

from typing import Protocol, runtime_checkable

from luna_model._lm import PyMaxBiasAnalysis
from luna_model.transformation.passes.analysis.builtin import BuiltinAnalysis


@runtime_checkable
class MaxBias(Protocol):
    """Protocol for maximum bias information stored in the analysis cache.

    This protocol defines the interface for accessing maximum bias values
    computed during model analysis.
    """

    @property
    def val(self) -> float:
        """Get the maximum bias value.

        Returns
        -------
        float
            The maximum bias value.
        """
        ...


class MaxBiasAnalysis(BuiltinAnalysis[MaxBias], PyMaxBiasAnalysis):
    """Analysis pass that computes the maximum absolute coefficient value in the objective.

    Examples
    --------
    >>> from luna_model import Model, Vtype
    >>> from luna_model.transformation import PassManager
    >>> from luna_model.transformation.passes import MaxBiasAnalysis
    >>> model = Model()
    >>> x = model.add_variable("x", vtype=Vtype.BINARY)
    >>> y = model.add_variable("y", vtype=Vtype.BINARY)
    >>> model.objective = x * y + x - 2 * y
    >>> pm = PassManager([MaxBiasAnalysis()])
    >>> output = pm.run(model)
    >>> output.context.require_analysis(MaxBiasAnalysis.key()).val
    2.0
    """
