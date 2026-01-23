import numpy as np
import pytest
from luna_model import Bounds, Model, Variable, Vtype
from luna_model.translator import NumpyTranslator
from numpy.typing import NDArray

from .fixtures import np_model, np_result


def test_numpy_translator(np_model: Model, np_result: tuple[NDArray, NDArray]):
    res, energies = np_result
    sol = NumpyTranslator.to_lm(res, energies, env=np_model.environment)
    assert [
        [0, 1.0, 1, 0, 0],
        [1, 0.0, 1, 0, 0],
        [0, 0.0, 1, 0, 0],
    ] == sol.samples.tolist()
    assert all([-2.0, -1.0, -1.0] == sol.raw_energies)
    for result in sol.results:
        assert result.obj_value is None
        assert result.constraints is None
        assert result.feasible is None
