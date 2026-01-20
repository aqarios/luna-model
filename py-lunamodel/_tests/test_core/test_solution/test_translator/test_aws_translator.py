import numpy as np
import pytest
from luna_model import Bounds, Model, Variable, Vtype
from luna_model.translator import AwsTranslator
from numpy.typing import NDArray

from .fixtures import aws_model, aws_result


def test_aws_translator(aws_model: Model, aws_result: dict[str, NDArray]):
    sol = AwsTranslator.to_lm(aws_result, env=aws_model.environment)
    assert [
        [0, 1.0, 1, 0, 0],
        [1, 0.0, 1, 0, 0],
        [0, 0.0, 1, 0, 0],
    ] == sol.samples.tolist()
    assert sol.raw_energies is not None
    assert all([-2.0, -1.0, -1.0] == sol.raw_energies)
    for result in sol.results:
        assert result.obj_value is None
        assert result.constraints is None
        assert result.feasible is None
