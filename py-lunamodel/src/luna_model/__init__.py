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
"""LunaModel: Blazingly fast symbolic modeling for optimization.

LunaModel is a high-performance symbolic modeling library for describing, translating
and transforming optimization problems. It provides the following high-level features:

- System for defining symbolic algebraic expressions of arbitrary degree,
  constraints and optimization models (like dimod, gurobi or cplex)
- Translations from and to an LunaModel for many common optimization model formats (like LP)
- Transformations to map an LunaModel from a general model to a specific model, such as transforming
  a Constrained (Binary) Quadratic Model (CQM) to a (Unconstrained) Binary Quadratic Model (BQM),
  or from an Integer Model to a Binary Model.
- Builtin serialization for maximum portability
- Python-first development experience

"""

from luna_model._lm import __version__
from luna_model.constraint import (
    Comparator,
    Constraint,
    ConstraintCollection,
    ConstraintCollectionIter,
)
from luna_model.environment import Environment
from luna_model.expression import (
    Constant,
    Expression,
    ExprIter,
    HigherOrder,
    Linear,
    Quadratic,
)
from luna_model.model import (
    Ctype,
    Model,
    ModelSpecs,
    Sense,
)
from luna_model.solution import (
    Result,
    ResultIter,
    ResultView,
    Sample,
    SampleIter,
    Samples,
    SamplesIter,
    Solution,
    ValueSource,
)
from luna_model.timer import (
    Timer,
    Timing,
)
from luna_model.ttarget import TranslationTarget
from luna_model.utils import quicksum
from luna_model.variable import (
    Bounds,
    Unbounded,
    Variable,
    Vtype,
)

from . import errors, transformation, translator, utils

__all__ = [
    "Bounds",
    "Comparator",
    "Constant",
    "Constraint",
    "ConstraintCollection",
    "ConstraintCollectionIter",
    "Ctype",
    "Environment",
    "ExprIter",
    "Expression",
    "HigherOrder",
    "Linear",
    "Model",
    "ModelSpecs",
    "Quadratic",
    "Result",
    "ResultIter",
    "ResultView",
    "Sample",
    "SampleIter",
    "Samples",
    "SamplesIter",
    "Sense",
    "Solution",
    "Timer",
    "Timing",
    "TranslationTarget",
    "Unbounded",
    "ValueSource",
    "Variable",
    "Vtype",
    "__version__",
    "errors",
    "quicksum",
    "transformation",
    "translator",
    "utils",
]
