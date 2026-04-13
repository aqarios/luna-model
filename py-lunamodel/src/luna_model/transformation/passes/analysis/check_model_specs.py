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

from typing import Self

from luna_model._lm import PyCheckModelSpecsAnalysis
from luna_model.model.specs import ModelSpecs
from luna_model.transformation.passes.analysis.builtin import BuiltinAnalysis


class CheckModelSpecsAnalysis(PyCheckModelSpecsAnalysis, BuiltinAnalysis[None]):
    """Analysis pass that checks the model's specs for correctness.

    This analysis pass checks if the input model satisfies the
    specifications (`ModelSpecs`) this pass is initilized with. If the specs
    are not satified by the model it raises an error during runtime.

    Examples
    --------
    >>> from luna_model import Model, Vtype
    >>> from luna_model.transformation import PassManager
    >>> from luna_model.transformation.passes import CheckModelSpecsAnalysis
    >>> model = Model()
    >>> x = model.add_variable("x", vtype=Vtype.BINARY)
    >>> y = model.add_variable("y", vtype=Vtype.BINARY)
    >>> model.objective = x * y + x - 2 * y
    >>> specs = ModelSpecs(max_degree=2)
    >>> pm = PassManager([CheckModelSpecsAnalysis(specs)])
    >>> output = pm.run(model)
    >>> # no errors raised
    """

    def __new__(cls, specs: ModelSpecs) -> Self:
        """Create a new check model specs analysis pass.

        Parameters
        ----------
        specs : ModelSpecs
            The model specs the model passed to this analysis pass has to fulfill.
        """
        return super().__new__(cls, specs._sp)
