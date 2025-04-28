import numpy as np
import pytest
from numpy.typing import NDArray

from aqmodels import Model, Variable, Bounds, Vtype, AwsTranslator


@pytest.fixture
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


@pytest.fixture
def aws_result():
    return {
        "samples": np.array(
            [
                [0, 1, 1, 0, 0],
                [1, 0, 1, 0, 0],
                [0, 1, 1, 0, 0],
                [0, 0, 1, 0, 0],
            ]
        ),
        "energies": np.array([-2.0, -1.0, -2.0, -1.0]),
    }


@pytest.mark.solution_translation
def test_zib_translator(model: Model, aws_result: dict[str, NDArray]):
    sol = AwsTranslator.from_aws_result(aws_result, env=model.environment)
    print(aws_result)
    print(sol)
    print("-" * 80)
    (sol_agg, indices, num_occ) = np.unique(
        aws_result["samples"], return_index=True, return_counts=True, axis=0
    )

    assert sol.samples.tolist() == sol_agg.tolist()
    for i, result in enumerate(sol.results):
        assert result.raw_energy == aws_result["energies"][indices[i]]
        assert result.obj_value is None
        assert result.constraints is None
        assert result.feasible is None
