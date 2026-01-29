import os
from pathlib import Path
from random import Random

import numpy as np
import pytest
import scipy.sparse as sp  # type: ignore[import-untyped]
from dimod import BinaryQuadraticModel, ConstrainedQuadraticModel
from numpy.typing import NDArray

from luna_model import Model
from luna_model.translator import (
    BqmTranslator,
    CqmTranslator,
    LpTranslator,
    QuboTranslator,
    TranslationTarget,
)
from tests.test_core.utils import generate_bqms, generate_cqms, make_seed


@pytest.fixture()
def model() -> Model:
    model = Model("test_model")
    x1 = model.add_variable("x1")
    x2 = model.add_variable("x2")
    x3 = model.add_variable("x3")
    x4 = model.add_variable("x4")
    model.objective = x1 + x2 + x3 - x4 + x1 * x2 - x3 * x4
    return model


def qubo() -> NDArray:
    size, density = 100, 0.5
    np.random.seed(make_seed())
    out = sp.random(size, size, density).todense()
    out += out.T
    return out


def lp_str() -> str:
    return (Path(__file__).parent / "lp_string.lp").read_text()


def lp_path() -> Path:
    return Path(__file__).parent / "lp_string.lp"


def bqm() -> BinaryQuadraticModel:
    rand = Random(make_seed())
    return generate_bqms(1, rand)[0]


def cqm() -> ConstrainedQuadraticModel:
    rand = Random(make_seed())
    return generate_cqms(1, rand)[0]


def test_model_from_qubo():
    q = qubo()
    m = Model.from_(q)
    t = QuboTranslator.to_lm(q)
    assert m.equal_contents(t)


def test_model_from_lp_str():
    s = lp_str()
    m = Model.from_(s)
    t = LpTranslator.to_lm(s)
    assert m.equal_contents(t)


def test_model_from_lp_path():
    p = lp_path()
    m = Model.from_(p)
    t = LpTranslator.to_lm(p)
    assert m.equal_contents(t)


def test_model_from_bqm():
    b = bqm()
    m = Model.from_(b)
    t = BqmTranslator.to_lm(b)
    assert m.equal_contents(t)


def test_model_from_cqm():
    c = cqm()
    m = Model.from_(c)
    t = CqmTranslator.to_lm(c)
    assert m.equal_contents(t)


def test_model_to_qubo(model: Model):
    mq = model.to(TranslationTarget.QUBO)
    tq = QuboTranslator.from_lm(model)
    assert np.allclose(mq.matrix, tq.matrix)
    assert np.isclose(mq.offset, tq.offset)
    assert mq.name == tq.name
    assert mq.variable_names == tq.variable_names
    assert mq.vtype == tq.vtype


def test_model_to_lp_str(model: Model):
    mstr = model.to(TranslationTarget.LP)
    tstr = LpTranslator.from_lm(model)
    assert mstr == tstr


def test_model_to_lp_path(model: Model):
    mtmp = Path(__file__).parent / "mtmp"
    mtmp.touch(exist_ok=True)
    ttmp = Path(__file__).parent / "ttmp"
    ttmp.touch(exist_ok=True)
    model.to(TranslationTarget.LP, filepath=mtmp)
    LpTranslator.from_lm(model, filepath=ttmp)
    is_equal = mtmp.read_text() == ttmp.read_text()
    os.remove(mtmp)
    os.remove(ttmp)
    assert is_equal


def test_model_to_bqm(model: Model):
    mbqm = model.to(TranslationTarget.BQM)
    tbqm = BqmTranslator.from_lm(model)
    assert Model.from_(mbqm).equal_contents(Model.from_(tbqm))


def test_model_to_cqm(model: Model):
    mcqm = model.to(TranslationTarget.CQM)
    tcqm = CqmTranslator.from_lm(model)
    assert Model.from_(mcqm).equal_contents(Model.from_(tcqm))
