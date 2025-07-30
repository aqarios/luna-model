import numpy as np
import pytest
from dimod import SampleSet, as_samples

from aqmodels.translator import DwaveTranslator

from .common import do_checks  # type: ignore[reportMissingImports]


@pytest.fixture
def dwave_solution() -> SampleSet:
    samples_raw = [
        {
            "b1": 0,
            "b2": 1,
            "s1": -1,
            "s2": +1,
            "i1": 4,
            "i2": 3,
        }
    ]
    sampleset = SampleSet.from_samples(
        as_samples(samples_raw),
        "BINARY",
        3.14,
        num_occurrences=np.array([1]),
    )
    return sampleset


def test_dwave_sol_with_substituted_model(dwave_solution: SampleSet):
    do_checks(DwaveTranslator, dwave_solution)
