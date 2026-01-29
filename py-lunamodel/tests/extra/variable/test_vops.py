import itertools

import pytest

from luna_model import Environment, Variable, Vtype

vtypes = [Vtype.BINARY, Vtype.SPIN, Vtype.INTEGER, Vtype.REAL]


@pytest.mark.parametrize("lhs_vtype", vtypes)
@pytest.mark.parametrize("rhs_vtype", vtypes)
def test_add(lhs_vtype, rhs_vtype):
    with Environment():
        lhs = Variable("lhs", lhs_vtype)
        rhs = Variable("rhs", rhs_vtype)

    res = lhs + rhs
    assert res.get_offset() == 0
    assert res.get_linear(lhs) == 1
    assert res.get_linear(rhs) == 1
    assert res.get_quadratic(lhs, rhs) == 0
    assert res.get_higher_order(lhs, rhs) == 0
    assert res.get_higher_order(rhs, lhs) == 0
    assert res.num_variables == 2
    assert res.degree() == 1
    assert lhs in res.variables()
    assert rhs in res.variables()


@pytest.mark.parametrize("lhs_vtype", vtypes)
@pytest.mark.parametrize("rhs_vtype", vtypes)
def test_sub(lhs_vtype, rhs_vtype):
    with Environment():
        lhs = Variable("lhs", lhs_vtype)
        rhs = Variable("rhs", rhs_vtype)

    res = lhs - rhs
    assert res.get_offset() == 0
    assert res.get_linear(lhs) == 1
    assert res.get_linear(rhs) == -1
    assert res.get_quadratic(lhs, rhs) == 0
    assert res.get_higher_order(lhs, rhs) == 0
    assert res.get_higher_order(rhs, lhs) == 0
    assert res.num_variables == 2
    assert res.degree() == 1
    assert lhs in res.variables()
    assert rhs in res.variables()


@pytest.mark.parametrize("lhs_vtype", vtypes)
@pytest.mark.parametrize("rhs_vtype", vtypes)
def test_mul_2(lhs_vtype, rhs_vtype):
    with Environment():
        lhs = Variable("lhs", lhs_vtype)
        rhs = Variable("rhs", rhs_vtype)

    res = lhs * rhs
    assert res.get_offset() == 0
    assert res.get_linear(lhs) == 0
    assert res.get_linear(rhs) == 0
    assert res.get_quadratic(lhs, rhs) == 1
    assert res.get_quadratic(rhs, lhs) == 1
    assert res.get_higher_order(lhs, rhs) == 0
    assert res.get_higher_order(rhs, lhs) == 0
    assert res.num_variables == 2
    assert res.degree() == 2
    assert lhs in res.variables()
    assert rhs in res.variables()


@pytest.mark.parametrize("a_vtype", vtypes)
@pytest.mark.parametrize("b_vtype", vtypes)
@pytest.mark.parametrize("c_vtype", vtypes)
def test_mul_3(a_vtype, b_vtype, c_vtype):
    with Environment():
        a = Variable("a", a_vtype)
        b = Variable("b", b_vtype)
        c = Variable("c", c_vtype)

    res = a * b * c
    assert res.degree() == 3
    assert res.get_offset() == 0
    assert res.num_variables == 3
    for e in [a, b, c]:
        assert e in res.variables()
        assert res.get_linear(a) == 0
    for es in itertools.combinations([a, b, c], 2):
        assert res.get_quadratic(*es) == 0
    for es in itertools.combinations([a, b, c], 3):
        assert res.get_higher_order(*es) == 1


@pytest.mark.parametrize("vtype", vtypes)
@pytest.mark.parametrize("p", [0, 1, 2, 3, 4])
def test_pow(vtype, p):
    with Environment():
        a = Variable("a", vtype)
    res = a**p
    match (vtype, p):
        case (_, 0):
            assert res.num_variables == 0
            assert res.degree() == 0
        case (Vtype.BINARY, _):
            assert res.num_variables == 1
            assert res.degree() == 1
        case (Vtype.SPIN, 2 | 4):
            assert res.num_variables == 0
            assert res.degree() == 0
        case (Vtype.SPIN, 1 | 3):
            assert res.num_variables == 1
            assert res.degree() == 1
        case (Vtype.INTEGER | Vtype.REAL, _):
            assert res.num_variables == 1
            assert p == res.degree()


@pytest.mark.parametrize("vtype", vtypes)
def test_neg(vtype):
    with Environment():
        a = Variable("a", vtype)

    res = -a
    assert res.get_offset() == 0
    assert res.get_linear(a) == -1
    assert res.degree() == 1
    assert a in res.variables()
