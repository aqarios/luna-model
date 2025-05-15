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

from ._variable import Vtype, Bounds, Variable
from ._timing import Timer, Timing
from ._solution import Solution
from ._sample import SampleIterator, Sample, SamplesIterator, Samples
from ._result import ResultView, Result, ResultIterator
from ._model import Model, Sense
from ._expression import Expression
from ._environment import Environment
from ._core import (
    Sample as __Sample,
    ResultIterator as __ResultIterator,
    Sense as __Sense,
    Comparator as __Comparator,
    Bounds as __Bounds,
    Samples as __Samples,
    Timing as __Timing,
    Expression as __Expression,
    SampleIterator as __SampleIterator,
    Model as __Model,
    Constraints as __Constraints,
    Constraint as __Constraint,
    SamplesIterator as __SamplesIterator,
    Solution as __Solution,
    Timer as __Timer,
    Vtype as __Vtype,
    Result as __Result,
    ResultView as __ResultView,
    Variable as __Variable,
    Environment as __Environment,
)
from ._constraints import Constraints, Comparator, Constraint
from . import translator, errors

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
VariableCreationError = errors.VariableCreationError
VariableNotExistingError = errors.VariableNotExistingError
VariablesFromDifferentEnvsError = errors.VariablesFromDifferentEnvsError
DifferentEnvsError = errors.DifferentEnvsError
NoActiveEnvironmentFoundError = errors.NoActiveEnvironmentFoundError
MultipleActiveEnvironmentsError = errors.MultipleActiveEnvironmentsError
DecodeError = errors.DecodeError
ModelVtypeError = errors.ModelVtypeError
VariableNamesError = errors.VariableNamesError
IllegalConstraintNameError = errors.IllegalConstraintNameError
TranslationError = errors.TranslationError
ModelNotQuadraticError = errors.ModelNotQuadraticError
ModelNotUnconstrainedError = errors.ModelNotUnconstrainedError
ModelSenseNotMinimizeError = errors.ModelSenseNotMinimizeError
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
    "IllegalConstraintNameError",
    "LpTranslator",
    "Model",
    "ModelNotQuadraticError",
    "ModelNotUnconstrainedError",
    "ModelSenseNotMinimizeError",
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
    "VariableCreationError",
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
