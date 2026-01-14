import numpy as np
import pytest
from luna_model import Bounds, Model, Variable, Vtype
from luna_model.translator import NumpyTranslator
from numpy.typing import NDArray

from .fixtures import np_model, np_result


def test_numpy_translator(np_model: Model, np_result: tuple[NDArray, NDArray]):
    res, energies = np_result
    sol = NumpyTranslator.to_lm(res, energies, env=np_model.environment)
    (sol_agg, indices, num_counts) = np.unique(
        res, return_index=True, return_counts=True, axis=0
    )

    assert sol.samples.tolist() == sol_agg.tolist()
    for i, result in enumerate(sol.results):
        assert result.raw_energy == energies[indices[i]]
        assert result.obj_value is None
        assert result.constraints is None
        assert result.feasible is None
