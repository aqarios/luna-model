from random import Random

import numpy as np
import pytest

from aqmodels import BqmTranslator
from pytests.test_core.utils import make_seed, generate_bqms


@pytest.mark.translator
def test_bqm_to_model_to_bqm():
    rand = Random(make_seed())
    bqms = generate_bqms(20, rand)
    for bqm in bqms:
        model = BqmTranslator.to_model(bqm)
        # First let's make sure this translation works correctly.
        # We can query the biases for each linear and quadratic interaction
        # (and the offset) in the two models and compare the returned bias value (f64)
        assert bqm.offset == model.objective.get_offset()
        for v in bqm.variables:
            aqv = model.environment.get_variable(str(v))
            aqm_bias = model.objective.get_linear(aqv)
            bqm_bias = bqm.get_linear(v)
            assert bqm_bias == aqm_bias, f"linear bias does not match for '{v}'"

        for v in bqm.variables:
            aqv = model.environment.get_variable(str(v))
            for u in bqm.variables:
                if v == u:
                    continue
                aqu = model.environment.get_variable(str(u))
                aqm_q_bias = model.objective.get_quadratic(aqv, aqu)
                bqm_q_bias = bqm.get_quadratic(v, u, default=0)
                assert bqm_q_bias == aqm_q_bias, "quadratic bias does not match"

        bqm_back = BqmTranslator.to_bqm(model)

        bqm_np = bqm.to_numpy_vectors()
        bqm_back_np = bqm_back.to_numpy_vectors()

        assert bqm.variables.to_serializable() == bqm_back.variables.to_serializable()
        assert bqm.vartype == bqm_back.vartype
        assert np.isclose(bqm_np.offset, bqm_back_np.offset)
        assert np.allclose(bqm_np.linear_biases, bqm_back_np.linear_biases)
        assert np.allclose(bqm_np.quadratic.biases, bqm_back_np.quadratic.biases)
        assert np.allclose(
            bqm_np.quadratic.row_indices, bqm_back_np.quadratic.row_indices
        )
        assert np.allclose(
            bqm_np.quadratic.col_indices, bqm_back_np.quadratic.col_indices
        )
