from numpy.typing import NDArray

from luna_model import Model
from luna_model.translator import NumpyTranslator
from .fixtures import *  # noqa: F403


def test_numpy_translator(np_model: Model, np_result: tuple[NDArray, NDArray]):
    res, energies = np_result
    sol = NumpyTranslator.to_lm(res, energies, env=np_model.environment)
    assert sol.samples.tolist() == [
        [0, 1.0, 1, 0, 0],
        [1, 0.0, 1, 0, 0],
        [0, 0.0, 1, 0, 0],
    ]
    assert all(sol.raw_energies == [-2.0, -1.0, -1.0])
    for result in sol.results:
        assert result.obj_value is None
        assert result.constraints is None
        assert result.feasible is None
