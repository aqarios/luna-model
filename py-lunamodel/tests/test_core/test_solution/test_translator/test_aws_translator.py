from numpy.typing import NDArray

from luna_model import Model
from luna_model.translator import AwsTranslator
from .fixtures import *  # noqa: F403


def test_aws_translator(aws_model: Model, aws_result: dict[str, NDArray]):
    sol = AwsTranslator.to_lm(aws_result, env=aws_model.environment)
    assert sol.samples.tolist() == [
        [0, 1.0, 1, 0, 0],
        [1, 0.0, 1, 0, 0],
        [0, 0.0, 1, 0, 0],
    ]
    assert sol.raw_energies is not None
    assert all(sol.raw_energies == [-2.0, -1.0, -1.0])
    for result in sol.results:
        assert result.obj_value is None
        assert result.constraints is None
        assert result.feasible is None
