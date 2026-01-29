from contextlib import nullcontext as does_not_raise
from itertools import product

import numpy as np
import pytest
import scipy.sparse as sp  # type: ignore[import-untyped]
from numpy.typing import NDArray

from luna_model import Model, Sense, Variable, Vtype
from luna_model.errors import (
    ModelNotQuadraticError,
    ModelNotUnconstrainedError,
    ModelSenseNotMinimizeError,
    ModelVtypeError,
    TranslationError,
    VariableExistsError,
    VariableNamesError,
)
from luna_model.translator import QuboTranslator

from ..utils import make_seed


@pytest.fixture()
def qubo(request) -> NDArray:
    size, density = request.param
    np.random.seed(make_seed())
    out = sp.random(size, size, density).todense()
    out += out.T
    return out


@pytest.fixture()
def asymmetric_qubo(request) -> NDArray:
    size, density = request.param
    np.random.seed(make_seed())
    out = sp.random(size, size, density).todense()
    return np.triu(out)


@pytest.fixture()
def linear_qubo(request) -> NDArray:
    size, density = request.param
    np.random.seed(make_seed())
    mat = sp.random(size, size, density).todense()
    mat = np.diag(np.diag(mat))
    return mat


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


@pytest.mark.parametrize(
    "qubo",
    list(product([100, 200, 400, 800], [0.1, 0.5, 1.0])),
    indirect=True,
)
def test_translate_with_dense(qubo: NDArray):
    model = QuboTranslator.to_lm(qubo)
    back = QuboTranslator.from_lm(model).matrix
    assert np.allclose(qubo, back)


@pytest.mark.parametrize(
    "qubo",
    list(product([100, 200, 400, 800], [0.1, 0.5, 1.0])),
    indirect=True,
)
def test_translate_with_dense_and_metadata(qubo: NDArray):
    offset = 4.2
    name = "test"
    vtype = Vtype.BINARY
    model = QuboTranslator.to_lm(qubo, offset=offset, name=name, vtype=vtype)
    back = QuboTranslator.from_lm(model)
    assert np.allclose(qubo, back.matrix)
    assert back.offset == offset
    assert back.vtype == vtype
    assert back.name == name
    assert back.variable_names == [f"x_{x}" for x in range(len(qubo))]


@pytest.mark.parametrize(
    "qubo",
    list(product([100, 200, 400, 800], [0.1, 0.5, 1.0])),
    indirect=True,
)
def test_translate_with_dense_and_valid_variable_names(qubo: NDArray):
    offset = 4.2
    name = "test"
    vtype = Vtype.BINARY
    variable_names = [f"x_{i},y_{i}" for i in range(len(qubo))]
    model = QuboTranslator.to_lm(qubo, offset=offset, name=name, vtype=vtype, variable_names=variable_names)
    back = QuboTranslator.from_lm(model)
    assert np.allclose(qubo, back.matrix)
    assert back.offset == offset
    assert back.vtype == vtype
    assert back.name == name
    assert back.variable_names == variable_names


@pytest.mark.parametrize(
    "qubo",
    list(product([100, 200, 400, 800], [0.1, 0.5, 1.0])),
    indirect=True,
)
def test_translate_with_dense_and_invalid_variable_names_non_alpha(qubo: NDArray):
    offset = 4.2
    name = "test"
    vtype = Vtype.BINARY
    variable_names = [str(i) for i in range(len(qubo))]
    with pytest.raises(TranslationError):
        _ = QuboTranslator.to_lm(qubo, offset=offset, name=name, vtype=vtype, variable_names=variable_names)


@pytest.mark.parametrize(
    "qubo",
    list(product([100, 200, 400, 800], [0.1, 0.5, 1.0])),
    indirect=True,
)
def test_translate_with_dense_and_invalid_variable_names(qubo: NDArray):
    offset = 4.2
    name = "test"
    vtype = Vtype.BINARY
    variable_names = [f"x_{i}+y_{i}" for i in range(len(qubo))]
    with pytest.raises(TranslationError):
        _ = QuboTranslator.to_lm(qubo, offset=offset, name=name, vtype=vtype, variable_names=variable_names)


@pytest.mark.parametrize("qubo", list(product([0], [0])), indirect=True)
def test_translate_with_dense_empty(qubo: NDArray):
    model = QuboTranslator.to_lm(qubo)
    back = QuboTranslator.from_lm(model).matrix
    assert np.allclose(qubo, back)


@pytest.mark.parametrize(
    "linear_qubo",
    list(product([100, 200, 400, 800], [0.1, 0.5, 1.0])),
    indirect=True,
)
def test_translate_with_dense_linear(linear_qubo: NDArray):
    model = QuboTranslator.to_lm(linear_qubo)
    back = QuboTranslator.from_lm(model).matrix
    assert np.allclose(linear_qubo, back)


@pytest.mark.parametrize(
    "qubo",
    list(product([100, 200, 400, 800], [0.1, 0.5, 1.0])),
    indirect=True,
)
def test_translate_from_non_fitting_constrained(qubo: NDArray):
    model = QuboTranslator.to_lm(qubo)
    with model.environment:
        b = Variable("b", vtype=Vtype.BINARY)
        s = Variable("s", vtype=Vtype.SPIN)
        i = Variable("i", vtype=Vtype.INTEGER)
        r = Variable("r", vtype=Vtype.REAL)
        model.constraints += b + s + i + r <= 3
        model.constraints += b * s == 3
        model.constraints += b * i * r >= 3

    with pytest.raises(ModelNotUnconstrainedError):
        _ = QuboTranslator.from_lm(model)

    with pytest.raises(TranslationError):
        _ = QuboTranslator.from_lm(model)


@pytest.mark.parametrize(
    "qubo",
    list(product([100, 200, 400, 800], [0.1, 0.5, 1.0])),
    indirect=True,
)
def test_translate_from_non_fitting_higher_order(qubo: NDArray):
    model = QuboTranslator.to_lm(qubo)
    with model.environment:
        b = Variable("b", vtype=Vtype.BINARY)
        model.objective *= b

    with pytest.raises(ModelNotQuadraticError):
        _ = QuboTranslator.from_lm(model)

    with pytest.raises(TranslationError):
        _ = QuboTranslator.from_lm(model) @ pytest.mark.translator


@pytest.mark.parametrize(
    "qubo",
    [(100, 0.1)],
    indirect=True,
)
def test_translate_from_non_fitting_vtype(qubo: NDArray):
    model = QuboTranslator.to_lm(qubo)
    with model.environment:
        r = Variable("r", vtype=Vtype.REAL)
        model.objective += r

    with pytest.raises(ModelVtypeError):
        _ = QuboTranslator.from_lm(model)

    with pytest.raises(TranslationError):
        _ = QuboTranslator.from_lm(model)

    model_2 = QuboTranslator.to_lm(qubo, vtype=Vtype.BINARY)

    with model_2.environment:
        s = Variable("s", vtype=Vtype.SPIN)
        model_2.objective += s

    with pytest.raises(ModelVtypeError):
        _ = QuboTranslator.from_lm(model_2)

    with pytest.raises(TranslationError):
        _ = QuboTranslator.from_lm(model_2)


def test_translate_from_maximization_sense(model: Model):
    with pytest.raises(ModelSenseNotMinimizeError):
        _ = QuboTranslator.from_lm(model)

    with pytest.raises(TranslationError):
        _ = QuboTranslator.from_lm(model)


@pytest.mark.parametrize(
    "asymmetric_qubo",
    list(product([100, 200, 400, 800], [0.1, 0.5, 1.0])),
    indirect=True,
)
def test_translator_symmetricizes(asymmetric_qubo: NDArray):
    model = QuboTranslator.to_lm(asymmetric_qubo)
    back = QuboTranslator.from_lm(model).matrix
    sym = (asymmetric_qubo + asymmetric_qubo.T) / 2
    assert np.allclose(sym, back)


@pytest.mark.parametrize("qubo", [(4, 0.2)], indirect=True)
def test_variable_names_param(qubo: NDArray):
    with does_not_raise():
        _ = QuboTranslator.to_lm(qubo)

    model_1 = QuboTranslator.to_lm(qubo, variable_names=["a", "b", "c", "d"])
    assert model_1.environment.get_variable("a").name == "a"
    assert model_1.environment.get_variable("b").name == "b"
    assert model_1.environment.get_variable("c").name == "c"
    assert model_1.environment.get_variable("d").name == "d"

    num_vars_msg = "number of variable names does not match number of variables"
    with pytest.raises(VariableNamesError, match=num_vars_msg):
        _ = QuboTranslator.to_lm(qubo, variable_names=[])
    with pytest.raises(VariableNamesError, match=num_vars_msg):
        _ = QuboTranslator.to_lm(qubo, variable_names=["a", "b", "c"])
    with pytest.raises(VariableNamesError, match=num_vars_msg):
        _ = QuboTranslator.to_lm(qubo, variable_names=["a", "b", "c", "d", "e"])
    with pytest.raises(VariableNamesError, match=num_vars_msg):
        _ = QuboTranslator.to_lm(qubo, variable_names=["a", "b", "c", "d", "a"])
    with pytest.raises(VariableNamesError, match=num_vars_msg):
        _ = QuboTranslator.to_lm(qubo, variable_names=["a", "a"])

    with pytest.raises(
        VariableExistsError,
        match="variable exists: a",
    ):
        _ = QuboTranslator.to_lm(qubo, variable_names=["a", "a", "c", "d"])
