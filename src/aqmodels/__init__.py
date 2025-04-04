# This file is auto-generated.
# Do not edit manually.

"""
AqModels
========

Provides
  1. A model object to define arbitrary (constrained) optimization problems.
  2. A solution object to define arbitrary solutions to optimization problems.
  3. Extendable translators to map arbitrary models of other libraries to an aq model.
  4. Extendable transformers to transform arbitrary (constrained) optimization problems.


How to use the documentation
----------------------------
Documentation is available in two forms: docstrings provided with the code, and a
reference guide, available from `the Aqarios homepage <https://docs.aqarios.com>`_.

We recommend exploring the docstrings using
`IPython <https://ipython.org>`_, an advanced Python shell with
TAB-completion and introspection capabilities.  See below for further
instructions.

The docstring examples assume that `aqmodels` has been imported as ``aqm``::

  >>> import aqmodels as aqm

Code snippets are indicated by three greater-than signs::

  >>> x = 42
  >>> x = x + 1

Use the built-in ``help`` function to view a function's docstring::

  >>> help(aqm.Model)
  ... # doctest: +SKIP

Available subpackages
---------------------
translators
    Built-in translators to map a model of a (constrained) optimization problem from
    another library to an aq-models model.
transformers
    Built-in transformers to map a model of a (constrained) optimization problem to
    another aq-models model. Such a transformer for example can map a constrained
    optimization problem to an unconstrained optimization problem or a quadratic model
    to a linear model.
"""

from ._variable import (
  Vtype,
  Bounds,
  Variable
)
from ._solution import (
  ResultIterator,
  SampleIterator,
  Samples,
  Sample,
  Result,
  ResultView,
  Solution,
  Timing,
  Timer
)
from ._model import Model
from ._expression import Expression
from ._environment import Environment
from ._core import (
  Comparator as __Comparator,
  Constraint as __Constraint,
  Constraints as __Constraints,
  Vtype as __Vtype,
  Bounds as __Bounds,
  Variable as __Variable,
  Model as __Model,
  ResultIterator as __ResultIterator,
  SampleIterator as __SampleIterator,
  Samples as __Samples,
  Sample as __Sample,
  Result as __Result,
  ResultView as __ResultView,
  Solution as __Solution,
  Timing as __Timing,
  Timer as __Timer,
  Environment as __Environment,
  Expression as __Expression
)
from ._constraints import (
  Comparator,
  Constraint,
  Constraints
)
from . import (
  errors,
  translator
)

Comparator = __Comparator  # type: ignore[misc,assignment]
Constraint = __Constraint  # type: ignore[misc,assignment]
Constraints = __Constraints  # type: ignore[misc,assignment]
Vtype = __Vtype  # type: ignore[misc,assignment]
Bounds = __Bounds  # type: ignore[misc,assignment]
Variable = __Variable  # type: ignore[misc,assignment]
Model = __Model  # type: ignore[misc,assignment]
ResultIterator = __ResultIterator  # type: ignore[misc,assignment]
SampleIterator = __SampleIterator  # type: ignore[misc,assignment]
Samples = __Samples  # type: ignore[misc,assignment]
Sample = __Sample  # type: ignore[misc,assignment]
Result = __Result  # type: ignore[misc,assignment]
ResultView = __ResultView  # type: ignore[misc,assignment]
Solution = __Solution  # type: ignore[misc,assignment]
Timing = __Timing  # type: ignore[misc,assignment]
Timer = __Timer  # type: ignore[misc,assignment]
Environment = __Environment  # type: ignore[misc,assignment]
Expression = __Expression  # type: ignore[misc,assignment]
SampleSetTranslator = translator.SampleSetTranslator
MatrixTranslator = translator.MatrixTranslator
VariableOutOfRangeError = errors.VariableOutOfRangeError
VariableExistsError = errors.VariableExistsError
VariablesFromDifferentEnvsError = errors.VariablesFromDifferentEnvsError
DifferentEnvsError = errors.DifferentEnvsError
NoActiveEnvironmentFoundError = errors.NoActiveEnvironmentFoundError
MultipleActiveEnvironmentsError = errors.MultipleActiveEnvironmentsError
DecodeError = errors.DecodeError
ModelNotQuadraticError = errors.ModelNotQuadraticError
ModelNotUnconstrainedError = errors.ModelNotUnconstrainedError

__all__ = [
    "Bounds",
    "Comparator",
    "Constraint",
    "Constraints",
    "DecodeError",
    "DifferentEnvsError",
    "Environment",
    "Expression",
    "MatrixTranslator",
    "Model",
    "ModelNotQuadraticError",
    "ModelNotUnconstrainedError",
    "MultipleActiveEnvironmentsError",
    "NoActiveEnvironmentFoundError",
    "Result",
    "ResultIterator",
    "ResultView",
    "Sample",
    "SampleIterator",
    "SampleSetTranslator",
    "Samples",
    "Solution",
    "Timer",
    "Timing",
    "Variable",
    "VariableExistsError",
    "VariableOutOfRangeError",
    "VariablesFromDifferentEnvsError",
    "Vtype",
    "errors",
    "translator",
]
