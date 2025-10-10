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

The docstring examples assume that `aqmodels` has been imported as `aqm`::

  >>> import aqmodels as aqm

Code snippets are indicated by three greater-than signs::

  >>> x = 42
  >>> x = x + 1

Use the built-in `help` function to view a function's docstring::

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

from ._core import (
    Bounds,
    Comparator,
    Constant,
    Constraint,
    Constraints,
    Environment,
    Expression,
    ExpressionIterator,
    HigherOrder,
    Linear,
    Model,
    Quadratic,
    Result,
    ResultIterator,
    ResultView,
    Sample,
    SampleIterator,
    Samples,
    SamplesIterator,
    Sense,
    Solution,
    Timer,
    Timing,
    Unbounded,
    Variable,
    ValueToggle,
    Vtype,
    errors,
    transformations,
    translator,
    utils,
)
from .utils import quicksum

__all__ = [
    "Bounds",
    "Comparator",
    "Constant",
    "Constraint",
    "Constraints",
    "Environment",
    "Expression",
    "ExpressionIterator",
    "HigherOrder",
    "Linear",
    "Model",
    "Quadratic",
    "Result",
    "ResultIterator",
    "ResultView",
    "Sample",
    "SampleIterator",
    "Samples",
    "SamplesIterator",
    "Sense",
    "Solution",
    "Timer",
    "Timing",
    "Unbounded",
    "Variable",
    "ValueToggle",
    "Vtype",
    "errors",
    "quicksum",
    "transformations",
    "translator",
    "utils",
]
