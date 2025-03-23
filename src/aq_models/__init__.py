import functools

import numpy as np
from dimod import SampleSet

from ._core import (
    Bounds,
    Comparator,
    Constraint,
    Constraints,
    DecodeException,
    DifferentEnvsException,
    Environment,
    Expression,
    MatrixTranslator,
    Model,
    ModelNotQuadraticException,
    ModelNotUnconstrainedException,
    MultipleActiveEnvironmentsException,
    NoActiveEnvironmentFoundException,
    Result,
    Results,
    SampleSetTranslator,
    Solution,
    Timer,
    Timing,
    Variable,
    VariableExistsException,
    VariableOutOfRangeException,
    VariablesFromDifferentEnvsException,
    Vtype,
)


def wrap_from_dimod_sample_set(f):
    @functools.wraps(SampleSetTranslator.from_dimod_sample_set)
    def inner(sample_set: SampleSet, timing: Timing | None = None) -> Solution:
        sample_set = sample_set.aggregate()
        record = sample_set.record
        sample = record.sample.astype(np.int64, order="C")
        num_occurrences = record.num_occurrences.astype(np.int64, order="C")

        return f(sample, num_occurrences, timing)

    return inner


SampleSetTranslator.from_dimod_sample_set = wrap_from_dimod_sample_set(
    SampleSetTranslator.from_dimod_sample_set
)

__all__ = [
    "Model",
    "Expression",
    "Constraint",
    "Constraints",
    "Comparator",
    "Variable",
    "Environment",
    "Vtype",
    "Bounds",
    "Result",
    "Results",
    "Solution",
    "Timing",
    "Timer",
    "MatrixTranslator",
    "VariableOutOfRangeException",
    "SampleSetTranslator",
    "VariableExistsException",
    "VariablesFromDifferentEnvsException",
    "DifferentEnvsException",
    "NoActiveEnvironmentFoundException",
    "MultipleActiveEnvironmentsException",
    "DecodeException",
    "ModelNotQuadraticException",
    "ModelNotUnconstrainedException",
]
