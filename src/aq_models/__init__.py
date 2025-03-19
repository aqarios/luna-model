import functools

import numpy as np
from dimod import SampleSet

from ._core import Bounds
from ._core import Constraint
from ._core import Constraints
from ._core import Environment
from ._core import Expression
from ._core import MatrixTranslator
from ._core import Model
from ._core import MultipleActiveEnvironmentsException
from ._core import NoActiveEnvironmentFoundException
from ._core import Result
from ._core import Results
from ._core import Runtime
from ._core import SampleSetTranslator
from ._core import Solution
from ._core import Variable
from ._core import VariableExistsException
from ._core import Vtype


def wrap_from_dimod_sample_set(f):
    @functools.wraps(SampleSetTranslator.from_dimod_sample_set)
    def inner(sample_set: SampleSet, runtime: Runtime) -> Solution:
        sample_set = sample_set.aggregate()
        record = sample_set.record
        sample = record.sample.astype(np.int64, order="C")
        num_occurrences = record.num_occurrences.astype(np.int64, order="C")

        return f(
            sample,
            num_occurrences,
            runtime
        )

    return inner


SampleSetTranslator.from_dimod_sample_set = wrap_from_dimod_sample_set(
    SampleSetTranslator.from_dimod_sample_set
)

__all__ = [
    "Model",
    "Expression",
    "Constraint",
    "Constraints",
    "Variable",
    "Environment",
    "Vtype",
    "Bounds",
    "Result",
    "Results",
    "Runtime",
    "Solution",
    "MatrixTranslator",
    "SampleSetTranslator",
    "VariableExistsException",
    "NoActiveEnvironmentFoundException",
    "MultipleActiveEnvironmentsException",
]
