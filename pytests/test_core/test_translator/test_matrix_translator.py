import pytest
from itertools import product

import numpy as np
import scipy.sparse as sp  # type: ignore[import-untyped]
from numpy.typing import NDArray

from aq_models import MatrixTranslator, Variable, Vtype
from aq_models._core import ModelNotQuadraticException, ModelNotUnconstrainedException
from ..utils import make_seed


@pytest.fixture
def qubo(request) -> NDArray:
    size, density = request.param
    np.random.seed(make_seed())
    out = sp.random(size, size, density).todense()
    out += out.T
    return out


@pytest.fixture
def asymmetric_qubo(request) -> NDArray:
    size, density = request.param
    np.random.seed(make_seed())
    out = sp.random(size, size, density).todense()
    return np.triu(out)


@pytest.fixture
def linear_qubo(request) -> NDArray:
    size, density = request.param
    np.random.seed(make_seed())
    mat = sp.random(size, size, density).todense()
    mat = np.diag(np.diag(mat))
    return mat


@pytest.mark.translator
@pytest.mark.parametrize(
    "qubo",
    list(product([100, 200, 400, 800], [0.1, 0.5, 1.0])),
    indirect=True,
)
def test_translate_with_dense(qubo: NDArray):
    model = MatrixTranslator.to_model(qubo)
    back = MatrixTranslator.to_dense(model)
    assert np.allclose(qubo, back)


@pytest.mark.translator
@pytest.mark.parametrize("qubo", list(product([0], [0])), indirect=True)
def test_translate_with_dense_empty(qubo: NDArray):
    model = MatrixTranslator.to_model(qubo)
    back = MatrixTranslator.to_dense(model)
    assert np.allclose(qubo, back)


@pytest.mark.translator
@pytest.mark.parametrize(
    "linear_qubo",
    list(product([100, 200, 400, 800], [0.1, 0.5, 1.0])),
    indirect=True,
)
def test_translate_with_dense_linear(linear_qubo: NDArray):
    model = MatrixTranslator.to_model(linear_qubo)
    back = MatrixTranslator.to_dense(model)
    assert np.allclose(linear_qubo, back)


@pytest.mark.translator
@pytest.mark.parametrize(
    "qubo",
    list(product([100, 200, 400, 800], [0.1, 0.5, 1.0])),
    indirect=True,
)
def test_translate_from_non_fitting_constrained(qubo: NDArray):
    model = MatrixTranslator.to_model(qubo)
    with model.environment:
        b = Variable("b", vtype=Vtype.Binary)
        s = Variable("s", vtype=Vtype.Spin)
        i = Variable("i", vtype=Vtype.Integer)
        r = Variable("r", vtype=Vtype.Real)
        model.constraints += b + s + i + r <= 3
        model.constraints += b * s == 3
        model.constraints += b * i * r >= 3

    with pytest.raises(ModelNotUnconstrainedException):
        _ = MatrixTranslator.to_dense(model)


@pytest.mark.translator
@pytest.mark.parametrize(
    "qubo",
    list(product([100, 200, 400, 800], [0.1, 0.5, 1.0])),
    indirect=True,
)
def test_translate_from_non_fitting_higher_order(qubo: NDArray):
    model = MatrixTranslator.to_model(qubo)
    with model.environment:
        b = Variable("b", vtype=Vtype.Binary)
        model.objective *= b

    with pytest.raises(ModelNotQuadraticException):
        _ = MatrixTranslator.to_dense(model)


@pytest.mark.translator
@pytest.mark.parametrize(
    "asymmetric_qubo",
    list(product([100, 200, 400, 800], [0.1, 0.5, 1.0])),
    indirect=True,
)
def test_translator_symmetricizes(asymmetric_qubo: NDArray):
    model = MatrixTranslator.to_model(asymmetric_qubo)
    back = MatrixTranslator.to_dense(model)
    sym = (asymmetric_qubo + asymmetric_qubo.T) / 2
    assert np.allclose(sym, back)
