import numpy as np
import pytest
from luna_model import Bounds, Model, Variable, Vtype
from luna_model.translator import NumpyTranslator
from numpy.typing import NDArray


@pytest.fixture()
def model() -> Model:
    m = Model(name="TestModel")
    with m.environment:
        x0 = Variable("x0")
        m.objective = x0 * 1
        x1 = Variable("x1", vtype=Vtype.Real)
        m.objective += x0 * x1 * -1
        x2 = Variable("x2")
        x3 = Variable("x3", vtype=Vtype.Integer, bounds=Bounds(0, 30))
        x4 = Variable("x4")
        m.objective += (
            x0 * x1 * 12.213
            + x1 * x2 * 0.5
            + x0 * x2 * -3
            + 1
            + x0 * x3 * 1848482
            + x1 * x4
        )
        m.constraints.add_constraint(x0 + x2 <= 1)
        m.constraints.add_constraint(x0 + x2 <= 1, "my_constraint")
    return m


@pytest.fixture()
def result() -> tuple[NDArray, NDArray]:
    return (
        np.array(
            [
                [0, 1, 1, 0, 0],
                [1, 0, 1, 0, 0],
                [0, 1, 1, 0, 0],
                [0, 0, 1, 0, 0],
            ]
        ),
        np.array([-2.0, -1.0, -2.0, -1.0]),
    )


@pytest.mark.solution_translation()
def test_numpy_translator(model: Model, result: tuple[NDArray, NDArray]):
    res, energies = result
    sol = NumpyTranslator.to_aq(res, energies, env=model.environment)
    (sol_agg, indices, num_counts) = np.unique(
        res, return_index=True, return_counts=True, axis=0
    )

    assert sol.samples.tolist() == sol_agg.tolist()
    for i, result in enumerate(sol.results):
        assert result.raw_energy == energies[indices[i]]
        assert result.obj_value is None
        assert result.constraints is None
        assert result.feasible is None
