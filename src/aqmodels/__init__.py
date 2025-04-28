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

from ._variable import Vtype, Variable, Bounds
from ._timing import Timer, Timing
from ._solution import Solution
from ._sample import SampleIterator, Sample, SamplesIterator, Samples
from ._result import Result, ResultIterator, ResultView
from ._model import Sense, Model
from ._expression import Expression
from ._environment import Environment
from ._core import (
    Constraint as __Constraint,
    Constraints as __Constraints,
    SampleIterator as __SampleIterator,
    Vtype as __Vtype,
    Sense as __Sense,
    Variable as __Variable,
    Samples as __Samples,
    Timing as __Timing,
    Environment as __Environment,
    Sample as __Sample,
    Model as __Model,
    Solution as __Solution,
    Expression as __Expression,
    SamplesIterator as __SamplesIterator,
    Timer as __Timer,
    Comparator as __Comparator,
    ResultView as __ResultView,
    Bounds as __Bounds,
    Result as __Result,
    ResultIterator as __ResultIterator,
)
from ._constraints import Comparator, Constraint, Constraints
from . import errors, translator

SamplesIterator = __SamplesIterator  # type: ignore[misc,assignment] # noqa: F811
SampleIterator = __SampleIterator  # type: ignore[misc,assignment] # noqa: F811
Samples = __Samples  # type: ignore[misc,assignment] # noqa: F811
Sample = __Sample  # type: ignore[misc,assignment] # noqa: F811
Timing = __Timing  # type: ignore[misc,assignment] # noqa: F811
Timer = __Timer  # type: ignore[misc,assignment] # noqa: F811
ResultIterator = __ResultIterator  # type: ignore[misc,assignment] # noqa: F811
Result = __Result  # type: ignore[misc,assignment] # noqa: F811
ResultView = __ResultView  # type: ignore[misc,assignment] # noqa: F811
Solution = __Solution  # type: ignore[misc,assignment] # noqa: F811
Expression = __Expression  # type: ignore[misc,assignment] # noqa: F811
Environment = __Environment  # type: ignore[misc,assignment] # noqa: F811
Vtype = __Vtype  # type: ignore[misc,assignment] # noqa: F811
Bounds = __Bounds  # type: ignore[misc,assignment] # noqa: F811
Variable = __Variable  # type: ignore[misc,assignment] # noqa: F811
Comparator = __Comparator  # type: ignore[misc,assignment] # noqa: F811
Constraint = __Constraint  # type: ignore[misc,assignment] # noqa: F811
Constraints = __Constraints  # type: ignore[misc,assignment] # noqa: F811
Sense = __Sense  # type: ignore[misc,assignment] # noqa: F811
Model = __Model  # type: ignore[misc,assignment] # noqa: F811
ZibTranslator = translator.ZibTranslator
CqmTranslator = translator.CqmTranslator
BqmTranslator = translator.BqmTranslator
DimodTranslator = translator.DimodTranslator
QctrlTranslator = translator.QctrlTranslator
IbmTranslator = translator.IbmTranslator
MatrixTranslator = translator.MatrixTranslator
LpTranslator = translator.LpTranslator
VariableOutOfRangeError = errors.VariableOutOfRangeError
VariableExistsError = errors.VariableExistsError
VariableNotExistingError = errors.VariableNotExistingError
VariablesFromDifferentEnvsError = errors.VariablesFromDifferentEnvsError
DifferentEnvsError = errors.DifferentEnvsError
NoActiveEnvironmentFoundError = errors.NoActiveEnvironmentFoundError
MultipleActiveEnvironmentsError = errors.MultipleActiveEnvironmentsError
DecodeError = errors.DecodeError
ModelNotQuadraticError = errors.ModelNotQuadraticError
ModelNotUnconstrainedError = errors.ModelNotUnconstrainedError
ModelVtypeError = errors.ModelVtypeError
SolutionCreationError = errors.SolutionCreationError
IllegalConstraintNameError = errors.IllegalConstraintNameError

__all__ = [
    "Bounds",
    "BqmTranslator",
    "Comparator",
    "Constraint",
    "Constraints",
    "CqmTranslator",
    "DecodeError",
    "DifferentEnvsError",
    "DimodTranslator",
    "Environment",
    "Expression",
    "IbmTranslator",
    "IllegalConstraintNameError",
    "LpTranslator",
    "MatrixTranslator",
    "Model",
    "ModelNotQuadraticError",
    "ModelNotUnconstrainedError",
    "ModelVtypeError",
    "MultipleActiveEnvironmentsError",
    "NoActiveEnvironmentFoundError",
    "QctrlTranslator",
    "Result",
    "ResultIterator",
    "ResultView",
    "Sample",
    "SampleIterator",
    "Samples",
    "SamplesIterator",
    "Sense",
    "Solution",
    "SolutionCreationError",
    "Timer",
    "Timing",
    "Variable",
    "VariableExistsError",
    "VariableNotExistingError",
    "VariableOutOfRangeError",
    "VariablesFromDifferentEnvsError",
    "Vtype",
    "ZibTranslator",
    "errors",
    "translator",
]
