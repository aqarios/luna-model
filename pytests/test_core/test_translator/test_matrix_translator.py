import numpy as np
import pytest
import scipy.sparse as sp  # type: ignore[import-untyped]
from itertools import product
from numpy.typing import NDArray

from aqmodels import (
    ModelNotQuadraticError,
    ModelNotUnconstrainedError,
    TranslationError,
)
from aqmodels import QuboTranslator, Variable, Vtype, ModelVtypeError
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
    model = QuboTranslator.to_aq(qubo)
    back = QuboTranslator.from_aq(model).matrix
    assert np.allclose(qubo, back)


@pytest.mark.translator
@pytest.mark.parametrize(
    "qubo",
    list(product([100, 200, 400, 800], [0.1, 0.5, 1.0])),
    indirect=True,
)
def test_translate_with_dense_and_metadata(qubo: NDArray):
    offset = 4.2
    name = "test"
    vtype = Vtype.Binary
    model = QuboTranslator.to_aq(qubo, offset=offset, name=name, vtype=vtype)
    back = QuboTranslator.from_aq(model)
    assert np.allclose(qubo, back.matrix)
    assert back.offset == offset
    assert back.vtype == vtype
    assert back.name == name
    assert back.variable_names == [f"x_{x}" for x in range(len(qubo))]


@pytest.mark.translator
@pytest.mark.parametrize(
    "qubo",
    list(product([100, 200, 400, 800], [0.1, 0.5, 1.0])),
    indirect=True,
)
def test_translate_with_dense_and_valid_variable_names(qubo: NDArray):
    offset = 4.2
    name = "test"
    vtype = Vtype.Binary
    variable_names = [f"x_{i},y_{i}" for i in range(len(qubo))]
    model = QuboTranslator.to_aq(
        qubo, offset=offset, name=name, vtype=vtype, variable_names=variable_names
    )
    back = QuboTranslator.from_aq(model)
    assert np.allclose(qubo, back.matrix)
    assert back.offset == offset
    assert back.vtype == vtype
    assert back.name == name
    assert back.variable_names == variable_names


@pytest.mark.translator
@pytest.mark.parametrize(
    "qubo",
    list(product([100, 200, 400, 800], [0.1, 0.5, 1.0])),
    indirect=True,
)
def test_translate_with_dense_and_invalid_variable_names_non_alpha(qubo: NDArray):
    offset = 4.2
    name = "test"
    vtype = Vtype.Binary
    variable_names = [str(i) for i in range(len(qubo))]
    with pytest.raises(TranslationError):
        _ = QuboTranslator.to_aq(
            qubo, offset=offset, name=name, vtype=vtype, variable_names=variable_names
        )


@pytest.mark.translator
@pytest.mark.parametrize(
    "qubo",
    list(product([100, 200, 400, 800], [0.1, 0.5, 1.0])),
    indirect=True,
)
def test_translate_with_dense_and_invalid_variable_names(qubo: NDArray):
    offset = 4.2
    name = "test"
    vtype = Vtype.Binary
    variable_names = [f"x_{i}+y_{i}" for i in range(len(qubo))]
    with pytest.raises(TranslationError):
        _ = QuboTranslator.to_aq(
            qubo, offset=offset, name=name, vtype=vtype, variable_names=variable_names
        )


@pytest.mark.translator
@pytest.mark.parametrize("qubo", list(product([0], [0])), indirect=True)
def test_translate_with_dense_empty(qubo: NDArray):
    model = QuboTranslator.to_aq(qubo)
    back = QuboTranslator.from_aq(model).matrix
    assert np.allclose(qubo, back)


@pytest.mark.translator
@pytest.mark.parametrize(
    "linear_qubo",
    list(product([100, 200, 400, 800], [0.1, 0.5, 1.0])),
    indirect=True,
)
def test_translate_with_dense_linear(linear_qubo: NDArray):
    model = QuboTranslator.to_aq(linear_qubo)
    back = QuboTranslator.from_aq(model).matrix
    assert np.allclose(linear_qubo, back)


@pytest.mark.translator
@pytest.mark.parametrize(
    "qubo",
    list(product([100, 200, 400, 800], [0.1, 0.5, 1.0])),
    indirect=True,
)
def test_translate_from_non_fitting_constrained(qubo: NDArray):
    model = QuboTranslator.to_aq(qubo)
    with model.environment:
        b = Variable("b", vtype=Vtype.Binary)
        s = Variable("s", vtype=Vtype.Spin)
        i = Variable("i", vtype=Vtype.Integer)
        r = Variable("r", vtype=Vtype.Real)
        model.constraints += b + s + i + r <= 3
        model.constraints += b * s == 3
        model.constraints += b * i * r >= 3

    with pytest.raises(ModelNotUnconstrainedError):
        _ = QuboTranslator.from_aq(model)

    with pytest.raises(TranslationError):
        _ = QuboTranslator.from_aq(model)


@pytest.mark.translator
@pytest.mark.parametrize(
    "qubo",
    list(product([100, 200, 400, 800], [0.1, 0.5, 1.0])),
    indirect=True,
)
def test_translate_from_non_fitting_higher_order(qubo: NDArray):
    model = QuboTranslator.to_aq(qubo)
    with model.environment:
        b = Variable("b", vtype=Vtype.Binary)
        model.objective *= b

    with pytest.raises(ModelNotQuadraticError):
        _ = QuboTranslator.from_aq(model)

    with pytest.raises(TranslationError):
        _ = QuboTranslator.from_aq(model) @ pytest.mark.translator


@pytest.mark.parametrize(
    "qubo",
    [(100, 0.1)],
    indirect=True,
)
def test_translate_from_non_fitting_vtype(qubo: NDArray):
    model = QuboTranslator.to_aq(qubo)
    with model.environment:
        r = Variable("r", vtype=Vtype.Real)
        model.objective += r

    with pytest.raises(ModelVtypeError):
        _ = QuboTranslator.from_aq(model)

    with pytest.raises(TranslationError):
        _ = QuboTranslator.from_aq(model)

    model_2 = QuboTranslator.to_aq(qubo, vtype=Vtype.Binary)
    with model.environment:
        s = Variable("s", vtype=Vtype.Spin)
        model.objective += s

    with pytest.raises(ModelVtypeError):
        _ = QuboTranslator.from_aq(model)

    with pytest.raises(TranslationError):
        _ = QuboTranslator.from_aq(model)


@pytest.mark.translator
@pytest.mark.parametrize(
    "asymmetric_qubo",
    list(product([100, 200, 400, 800], [0.1, 0.5, 1.0])),
    indirect=True,
)
def test_translator_symmetricizes(asymmetric_qubo: NDArray):
    model = QuboTranslator.to_aq(asymmetric_qubo)
    back = QuboTranslator.from_aq(model).matrix
    sym = (asymmetric_qubo + asymmetric_qubo.T) / 2
    assert np.allclose(sym, back)
