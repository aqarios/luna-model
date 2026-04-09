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
from luna_model._lm import PySpecsAnalysis
from luna_model.transformation.analysis import ConcreteAnalysisPass


class SpecsAnalysis(ConcreteAnalysisPass):
    """Analysis pass that computes the model's specs and stores them in the cache.

    This analysis pass computes the `ModelSpecs` of the input model and writes it
    to the `AnalysisCache` to be used by transformation passes following it during
    the `PassManager` execution. This analysis pass can, for example, be used in
    combination with the `IfElsePass` to guide the transformation depending on the
    specifications (`ModelSpecs`) of a model.

    Notes
    -----
    This pass is not to be confused with the `CheckModelSpecsAnalysis`, which checks
    if an input model's specs satisfy a given set of `ModelSpecs`.

    Examples
    --------
    >>> from luna_model import Model, Vtype
    >>> from luna_model.transformation import PassManager
    >>> from luna_model.transformation.passes import SpecsAnalysis
    >>> model = Model()
    >>> x = model.add_variable("x", vtype=Vtype.BINARY)
    >>> y = model.add_variable("y", vtype=Vtype.BINARY)
    >>> model.objective = x * y + x - 2 * y
    >>> pm = PassManager([SpecsAnalysis()])
    >>> ir = pm.run(model)
    >>> ir.cache["specs"]
    ModelSpecs(sense=Minimize, vtype=Binary, constraints=, max_degree=2, max_constraint_degree=0, max_num_variables=2)
    """

    def __init__(self) -> None:
        super().__init__(base=PySpecsAnalysis())
