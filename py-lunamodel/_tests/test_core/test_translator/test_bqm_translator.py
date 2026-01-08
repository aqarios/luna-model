from random import Random

import numpy as np
import pytest
from dimod import BinaryQuadraticModel
from luna_model import Model, Sense, Variable
from luna_model.errors import (
    ModelSenseNotMinimizeError,
    TranslationError,
    VariableNamesError,
)
from luna_model.translator import BqmTranslator

from _tests.test_core.utils import generate_bqms, make_seed


@pytest.fixture()
def model() -> Model:
    model = Model("test_model")
    with model.environment:
        x1 = Variable("x1")
        x2 = Variable("x2")
        x3 = Variable("x3")
        x4 = Variable("x4")
    model.objective = x1 + x2 + x3 - x4 + x1 * x2 - x3 * x4
    model.set_sense(Sense.Max)
    return model


def test_bqm_to_model_to_bqm():
    rand = Random(make_seed())
    bqms = generate_bqms(20, rand)
    for bqm in bqms:
        model = BqmTranslator.to_aq(bqm)
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

        bqm_back = BqmTranslator.from_aq(model)

        bqm_np = bqm.to_numpy_vectors()
        bqm_back_np = bqm_back.to_numpy_vectors()

        assert bqm.variables.to_serializable() == bqm_back.variables.to_serializable()
        assert bqm.vartype == bqm_back.vartype
        assert np.isclose(bqm_np.offset, bqm_back_np.offset, atol=1e-5)
        assert np.allclose(bqm_np.linear_biases, bqm_back_np.linear_biases)
        assert np.allclose(bqm_np.quadratic.biases, bqm_back_np.quadratic.biases)
        assert np.allclose(
            bqm_np.quadratic.row_indices, bqm_back_np.quadratic.row_indices
        )
        assert np.allclose(
            bqm_np.quadratic.col_indices, bqm_back_np.quadratic.col_indices
        )


def test_bqm_translator_wrong_sense(model: Model):
    with pytest.raises(ModelSenseNotMinimizeError):
        _ = BqmTranslator.from_aq(model)

    with pytest.raises(TranslationError):
        _ = BqmTranslator.from_aq(model)


def test_invalid_var_name():
    bqm = BinaryQuadraticModel(
        {"0": 4.0, "1": -2.0, "2": 6.0, "3": 2.0, "4": 5.0},
        {("2", "3"): 6.0, ("0", "4"): 4.0},
        offset=0,
        vartype="BINARY",
    )
    with pytest.raises(VariableNamesError, match="variable creation failed:"):
        _ = BqmTranslator.to_aq(bqm)


def test_error_handling_int_vars():
    rand = Random(make_seed())
    bqms = generate_bqms(3, rand, int_vars=True)
    for bqm in bqms:
        with pytest.raises(
            TypeError, match="All BQM variables have to be of type str, received:"
        ):
            _ = BqmTranslator.to_aq(bqm)
