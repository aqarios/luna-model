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
        bqm_back = BqmTranslator.to_bqm(model)

        bqm_np = bqm.to_numpy_vectors()
        bqm_back_np = bqm_back.to_numpy_vectors()

        assert bqm.variables.to_serializable() == bqm_back.variables.to_serializable()
        assert bqm.vartype == bqm_back.vartype
        assert np.isclose(bqm_np.offset, bqm_back_np.offset)
        assert np.allclose(bqm_np.linear_biases, bqm_back_np.linear_biases)
        assert np.allclose(bqm_np.quadratic.biases, bqm_back_np.quadratic.biases)
        assert np.allclose(bqm_np.quadratic.row_indices, bqm_back_np.quadratic.row_indices)
        assert np.allclose(bqm_np.quadratic.col_indices, bqm_back_np.quadratic.col_indices)
