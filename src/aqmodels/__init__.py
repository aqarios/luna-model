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

from ._variable import Bounds, Variable, Vtype
from ._timing import Timing, Timer
from ._solution import Solution
from ._sample import Samples, Sample, SamplesIterator, SampleIterator
from ._result import ResultView, Result, ResultIterator
from ._model import Sense, Model
from ._expression import Expression
from ._environment import Environment
from ._core import (
    Bounds as __Bounds,
    Sample as __Sample,
    Comparator as __Comparator,
    Expression as __Expression,
    Variable as __Variable,
    Vtype as __Vtype,
    Result as __Result,
    Timing as __Timing,
    Environment as __Environment,
    SamplesIterator as __SamplesIterator,
    Constraint as __Constraint,
    Model as __Model,
    Timer as __Timer,
    ResultIterator as __ResultIterator,
    Samples as __Samples,
    ResultView as __ResultView,
    Sense as __Sense,
    Constraints as __Constraints,
    Solution as __Solution,
    SampleIterator as __SampleIterator,
)
from ._constraints import Comparator, Constraint, Constraints
from . import errors, translator

Comparator = __Comparator  # type: ignore[misc,assignment] # noqa: F811
Constraint = __Constraint  # type: ignore[misc,assignment] # noqa: F811
Constraints = __Constraints  # type: ignore[misc,assignment] # noqa: F811
Vtype = __Vtype  # type: ignore[misc,assignment] # noqa: F811
Bounds = __Bounds  # type: ignore[misc,assignment] # noqa: F811
Variable = __Variable  # type: ignore[misc,assignment] # noqa: F811
Timing = __Timing  # type: ignore[misc,assignment] # noqa: F811
Timer = __Timer  # type: ignore[misc,assignment] # noqa: F811
Sense = __Sense  # type: ignore[misc,assignment] # noqa: F811
Model = __Model  # type: ignore[misc,assignment] # noqa: F811
ResultIterator = __ResultIterator  # type: ignore[misc,assignment] # noqa: F811
Result = __Result  # type: ignore[misc,assignment] # noqa: F811
ResultView = __ResultView  # type: ignore[misc,assignment] # noqa: F811
Solution = __Solution  # type: ignore[misc,assignment] # noqa: F811
Environment = __Environment  # type: ignore[misc,assignment] # noqa: F811
SamplesIterator = __SamplesIterator  # type: ignore[misc,assignment] # noqa: F811
SampleIterator = __SampleIterator  # type: ignore[misc,assignment] # noqa: F811
Samples = __Samples  # type: ignore[misc,assignment] # noqa: F811
Sample = __Sample  # type: ignore[misc,assignment] # noqa: F811
Expression = __Expression  # type: ignore[misc,assignment] # noqa: F811
Qubo = translator.Qubo
QuboTranslator = translator.QuboTranslator
AwsTranslator = translator.AwsTranslator
ZibTranslator = translator.ZibTranslator
DwaveTranslator = translator.DwaveTranslator
BqmTranslator = translator.BqmTranslator
QctrlTranslator = translator.QctrlTranslator
LpTranslator = translator.LpTranslator
NumpyTranslator = translator.NumpyTranslator
IbmTranslator = translator.IbmTranslator
CqmTranslator = translator.CqmTranslator
VariableOutOfRangeError = errors.VariableOutOfRangeError
VariableExistsError = errors.VariableExistsError
VariableNotExistingError = errors.VariableNotExistingError
VariablesFromDifferentEnvsError = errors.VariablesFromDifferentEnvsError
DifferentEnvsError = errors.DifferentEnvsError
NoActiveEnvironmentFoundError = errors.NoActiveEnvironmentFoundError
MultipleActiveEnvironmentsError = errors.MultipleActiveEnvironmentsError
DecodeError = errors.DecodeError
ModelVtypeError = errors.ModelVtypeError
VariableNamesError = errors.VariableNamesError
TranslationError = errors.TranslationError
ModelNotQuadraticError = errors.ModelNotQuadraticError
ModelNotUnconstrainedError = errors.ModelNotUnconstrainedError
SolutionTranslationError = errors.SolutionTranslationError
SampleIncorrectLengthError = errors.SampleIncorrectLengthError
SampleIncompatibleVtypeError = errors.SampleIncompatibleVtypeError

__all__ = [
    "AwsTranslator",
    "Bounds",
    "BqmTranslator",
    "Comparator",
    "Constraint",
    "Constraints",
    "CqmTranslator",
    "DecodeError",
    "DifferentEnvsError",
    "DwaveTranslator",
    "Environment",
    "Expression",
    "IbmTranslator",
    "LpTranslator",
    "Model",
    "ModelNotQuadraticError",
    "ModelNotUnconstrainedError",
    "ModelVtypeError",
    "MultipleActiveEnvironmentsError",
    "NoActiveEnvironmentFoundError",
    "NumpyTranslator",
    "QctrlTranslator",
    "Qubo",
    "QuboTranslator",
    "Result",
    "ResultIterator",
    "ResultView",
    "Sample",
    "SampleIncompatibleVtypeError",
    "SampleIncorrectLengthError",
    "SampleIterator",
    "Samples",
    "SamplesIterator",
    "Sense",
    "Solution",
    "SolutionTranslationError",
    "Timer",
    "Timing",
    "TranslationError",
    "Variable",
    "VariableExistsError",
    "VariableNamesError",
    "VariableNotExistingError",
    "VariableOutOfRangeError",
    "VariablesFromDifferentEnvsError",
    "Vtype",
    "ZibTranslator",
    "errors",
    "translator",
]
