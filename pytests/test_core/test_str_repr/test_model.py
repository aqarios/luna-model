import re
import string
from contextlib import nullcontext as does_not_raise

import pytest

from aqmodels import Bounds, Environment, Expression, Model, Variable, Vtype

_model_str_1 = """Model: TestModel
Minimize
  x0
Bounds
  0 <= x0 <= 1
Binary
  x0"""
_model_str_2 = """Model: TestModel
Minimize
  -x0 * x1 + x0
Bounds
  0 <= x0 <= 1
  0 <= x1
Binary
  x0
Real
  x1"""
_model_str_3 = """Model: TestModel
Minimize
  12.213 * x0 * x1 * x2 - x0 * x1 - 3 * x0 * x2 + 1848482 * x0 * x3 
  + 0.5 * x1 * x2 + x1 * x4 + x0 + 1
Subject To
  c0: x0 + x2 <= 1
Bounds
  0 <= x0 <= 1
  0 <= x1
  0 <= x2 <= 1
  0 <= x3 <= 30
  -1 <= x4 <= 1
Binary
  x0 x2
Spin
  x4
Integer
  x3
Real
  x1"""
_model_str_4 = """Model: TestModel
Minimize
  12.213 * x0 * x1 * x2 - x0 * x1 - 3 * x0 * x2 + 1848482 * x0 * x3 
  + 0.5 * x1 * x2 + x1 * x4 + x0 + 1
Subject To
  c0: x0 + x2 <= 1
  my_constraint: x0 + x2 <= 1
Bounds
  0 <= x0 <= 1
  0 <= x1
  0 <= x2 <= 1
  0 <= x3 <= 30
  -1 <= x4 <= 1
Binary
  x0 x2
Spin
  x4
Integer
  x3
Real
  x1"""
_model_repr_1 = "Model(name=MyModel, sense=Minimize, objective=a + 2 b + 2 c + 2 d + 2 e + 2 f + 2 g + 2 h + 2 i + 2 j + 2 k + 2 l + 2 m + 2 n + 2 o + 2 p + 2 q + 2 r + 2 s + 2 t, constraints=[])"


@pytest.fixture
def variables(request) -> tuple[Variable, ...]:
    with Environment():
        variables = [
            Variable(f"{string.ascii_lowercase[i]}") for i in range(request.param)
        ]
    return tuple(variables)


@pytest.mark.str_repr
def test_vtype():
    assert str(Vtype.Real) == "Real"
    assert str(Vtype.Binary) == "Binary"
    assert str(Vtype.Spin) == "Spin"
    assert str(Vtype.Integer) == "Integer"

    with does_not_raise():
        repr(Vtype.Real)
        repr(Vtype.Binary)
        repr(Vtype.Spin)
        repr(Vtype.Integer)


@pytest.mark.str_repr
def test_variable():
    with Environment():
        a = Variable("a")
        assert str(a) == "a: Binary"
        b = Variable("b", vtype=Vtype.Spin)
        assert str(b) == "b: Spin"
        c = Variable("c", vtype=Vtype.Integer)
        assert str(c) == "c: Integer { lower: 0 }"
        d = Variable("d", vtype=Vtype.Integer, bounds=Bounds(lower=0, upper=10))
        assert str(d) == "d: Integer { lower: 0, upper: 10 }"
        e = Variable("e", vtype=Vtype.Integer, bounds=Bounds(lower=3))
        assert str(e) == "e: Integer { lower: 3 }"
        f = Variable("f", vtype=Vtype.Integer, bounds=Bounds(upper=10))
        assert str(f) == "f: Integer { lower: 0, upper: 10 }"
        g = Variable("g", vtype=Vtype.Real)
        assert str(g) == "g: Real { lower: 0 }"
        h = Variable("h", vtype=Vtype.Real, bounds=Bounds(lower=-1.5, upper=1))
        assert str(h) == "h: Real { lower: -1.5, upper: 1 }"
        i = Variable("i", vtype=Vtype.Real, bounds=Bounds(lower=10))
        assert str(i) == "i: Real { lower: 10 }"
        j = Variable("j", vtype=Vtype.Real, bounds=Bounds(upper=3.8))
        assert str(j) == "j: Real { lower: 0, upper: 3.8 }"

        with does_not_raise():
            repr(a)
            repr(b)
            repr(c)
            repr(d)
            repr(e)
            repr(f)
            repr(g)
            repr(h)
            repr(i)
            repr(j)


@pytest.mark.str_repr
def test_bounds():
    bounds_1 = Bounds(lower=0, upper=1.5)
    assert str(bounds_1) == "{ lower: 0, upper: 1.5 }"
    bounds_2 = Bounds(lower=-1, upper=10)
    assert str(bounds_2) == "{ lower: -1, upper: 10 }"

    with does_not_raise():
        repr(bounds_1)
        repr(bounds_2)


@pytest.mark.str_repr
@pytest.mark.parametrize("variables", [3], indirect=True)
def test_expression(variables: tuple[Variable, ...]):
    a, b, c = variables

    expressions: list[Expression] = []

    # linear
    expressions.append(a + b)
    assert str(expressions[-1]) == "a + b"
    expressions.append(a + b * -1)
    assert str(expressions[-1]) == "a - b"
    expressions.append(b + a)
    assert str(expressions[-1]) == "a + b"
    expressions.append(a * -1 + b)
    assert str(expressions[-1]) == "-a + b"
    expressions.append(a * 2 + b)
    assert str(expressions[-1]) == "2 * a + b"
    expressions.append(a * 1.5 + b)
    assert str(expressions[-1]) == "1.5 * a + b"
    expressions.append(a * 2 + b + -1)
    assert str(expressions[-1]) == "2 * a + b - 1"
    expressions.append(a * 2 + b + 1)
    assert str(expressions[-1]) == "2 * a + b + 1"
    expressions.append(a * 2 + b + 1.5)
    assert str(expressions[-1]) == "2 * a + b + 1.5"
    expressions.append(a * 2 + b + 0)
    assert str(expressions[-1]) == "2 * a + b"

    # quadratic
    expressions.append(a * b)
    assert str(expressions[-1]) == "a * b"
    expressions.append(a * b * -1)
    assert str(expressions[-1]) == "-a * b"
    expressions.append(a * b * 2 + a)
    assert str(expressions[-1]) == "2 * a * b + a"
    expressions.append(a * b * -2 + a * -1)
    assert str(expressions[-1]) == "-2 * a * b - a"
    expressions.append(a * b * -2 + 5)
    assert str(expressions[-1]) == "-2 * a * b + 5"
    expressions.append(a * c + a * b)
    assert str(expressions[-1]) == "a * b + a * c"

    # higher order
    expressions.append(a * b * c)
    assert str(expressions[-1]) == "a * b * c"
    expressions.append(a * b * 2 * c)
    assert str(expressions[-1]) == "2 * a * b * c"
    expressions.append(a * b * c * -1)
    assert str(expressions[-1]) == "-a * b * c"
    expressions.append(a * b * c + a * b + c + 1)
    assert str(expressions[-1]) == "a * b * c + a * b + c + 1"

    with does_not_raise():
        for expr in expressions:
            repr(expr)

    # for expr in expressions:
    #     print(repr(expr))
    # raise  Exception


@pytest.mark.str_repr
@pytest.mark.parametrize("variables", [2], indirect=True)
def test_constraints(variables: tuple[Variable, ...]):
    a, b = variables
    c1 = a * 1 == 0
    assert str(c1) == "a == 0"
    c2 = a + b <= 10
    assert str(c2) == "a + b <= 10"
    c3 = a * b * 2 + 1 >= -1
    assert str(c3) == "2 * a * b >= -2"

    with does_not_raise():
        repr(c1)
        repr(c2)
        repr(c3)


@pytest.mark.str_repr
def test_environment():
    with Environment() as env:
        _ = Variable("a")
        _ = Variable("b", vtype=Vtype.Integer)
        _ = Variable("c")
        env_str = re.sub(r"(Environment\s+)[^\n]+", r"\1?", str(env))
        assert env_str == "Environment ?\n  a, b, c"


@pytest.mark.str_repr
def test_model():
    with Environment():
        x0 = Variable("x0")
        m = Model(name="TestModel")
        m.objective = x0 * 1
        assert str(m) == _model_str_1
        x1 = Variable("x1", vtype=Vtype.Real)
        m.objective += x0 * x1 * -1
        assert str(m) == _model_str_2
        x2 = Variable("x2")
        x3 = Variable("x3", vtype=Vtype.Integer, bounds=Bounds(0, 30))
        x4 = Variable("x4", vtype=Vtype.Spin)
        m.objective += (
            x0 * x1 * x2 * 12.213
            + x1 * x2 * 0.5
            + x0 * x2 * -3
            + 1
            + x0 * x3 * 1848482
            + x1 * x4
        )
        m.constraints.add_constraint(x0 + x2 <= 1)
        assert str(m) == _model_str_3
        m.constraints.add_constraint(x0 + x2 <= 1, "my_constraint")
        assert str(m) == _model_str_4

    with does_not_raise():
        repr(m)


@pytest.mark.str_repr
@pytest.mark.parametrize("variables", [20], indirect=True)
def test_expression_repr(variables: tuple[Variable, ...]):
    m = Model(name="MyModel")
    m.objective = variables[0] * 1
    for v in variables[1:]:
        m.objective += v * 2

    assert repr(m) == _model_repr_1
