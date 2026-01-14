import numpy as np
import pytest
from luna_model import Bounds, Model, Variable, Vtype
from luna_model.translator import AwsTranslator
from numpy.typing import NDArray

from .fixtures import aws_model, aws_result


def test_aws_translator(aws_model: Model, aws_result: dict[str, NDArray]):
    sol = AwsTranslator.to_lm(aws_result, env=aws_model.environment)
    (sol_agg, indices, num_counts) = np.unique(
        aws_result["samples"], return_index=True, return_counts=True, axis=0
    )

    assert sol.samples.tolist() == sol_agg.tolist()
    for i, result in enumerate(sol.results):
        assert result.raw_energy == aws_result["energies"][indices[i]]
        assert result.obj_value is None
        assert result.constraints is None
        assert result.feasible is None
