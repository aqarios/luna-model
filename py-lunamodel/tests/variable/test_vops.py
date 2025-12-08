import pytest
import itertools

from luna_model import Variable, Vtype, Environment


vtypes = [Vtype.BINARY, Vtype.SPIN, Vtype.INTEGER, Vtype.REAL]


@pytest.mark.parametrize("lhs_vtype", vtypes)
@pytest.mark.parametrize("rhs_vtype", vtypes)
def test_add(lhs_vtype, rhs_vtype):
    with Environment():
        lhs = Variable("lhs", lhs_vtype)
        rhs = Variable("rhs", rhs_vtype)

    res = lhs + rhs
    assert 0 == res.get_offset()
    assert 1 == res.get_linear(lhs)
    assert 1 == res.get_linear(rhs)
    assert 0 == res.get_quadratic(lhs, rhs)
    assert 0 == res.get_higher_order(lhs, rhs)
    assert 0 == res.get_higher_order(rhs, lhs)
    # assert 2 == res.num_variables
    assert 1 == res.degree()
    assert lhs in res.variables()
    assert rhs in res.variables()


@pytest.mark.parametrize("lhs_vtype", vtypes)
@pytest.mark.parametrize("rhs_vtype", vtypes)
def test_sub(lhs_vtype, rhs_vtype):
    with Environment():
        lhs = Variable("lhs", lhs_vtype)
        rhs = Variable("rhs", rhs_vtype)

    res = lhs - rhs
    assert 0 == res.get_offset()
    assert 1 == res.get_linear(lhs)
    assert -1 == res.get_linear(rhs)
    assert 0 == res.get_quadratic(lhs, rhs)
    assert 0 == res.get_higher_order(lhs, rhs)
    assert 0 == res.get_higher_order(rhs, lhs)
    # assert 2 == res.num_variables
    assert 1 == res.degree()
    assert lhs in res.variables()
    assert rhs in res.variables()


@pytest.mark.parametrize("lhs_vtype", vtypes)
@pytest.mark.parametrize("rhs_vtype", vtypes)
def test_mul_2(lhs_vtype, rhs_vtype):
    with Environment():
        lhs = Variable("lhs", lhs_vtype)
        rhs = Variable("rhs", rhs_vtype)

    res = lhs * rhs
    assert 0 == res.get_offset()
    assert 0 == res.get_linear(lhs)
    assert 0 == res.get_linear(rhs)
    assert 1 == res.get_quadratic(lhs, rhs)
    assert 1 == res.get_quadratic(rhs, lhs)
    assert 0 == res.get_higher_order(lhs, rhs)
    assert 0 == res.get_higher_order(rhs, lhs)
    # assert 2 == res.num_variables
    assert 2 == res.degree()
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
    assert 3 == res.degree()
    assert 0 == res.get_offset()
    for e in [a, b, c]:
        assert e in res.variables()
        assert 0 == res.get_linear(a)
    for es in itertools.combinations([a, b, c], 2):
        assert 0 == res.get_quadratic(*es)
    for es in itertools.combinations([a, b, c], 3):
        assert 1 == res.get_higher_order(*es)


@pytest.mark.parametrize("vtype", vtypes)
@pytest.mark.parametrize("p", [0, 1, 2, 3, 4])
def test_pow(vtype, p):
    with Environment():
        a = Variable("a", vtype)
    res = a**p
    match (vtype, p):
        case (_, 0):
            assert 0 == res.degree()
        case (Vtype.BINARY, _):
            assert 1 == res.degree()
        case (Vtype.SPIN, 2 | 4):
            assert 0 == res.degree()
        case (Vtype.SPIN, 1 | 3):
            assert 1 == res.degree()
        case (Vtype.INTEGER | Vtype.REAL, _):
            assert p == res.degree()

@pytest.mark.parametrize("vtype", vtypes)
def test_neg(vtype):
    with Environment():
        a = Variable("a", vtype)

    res = -a
    assert 0 == res.get_offset()
    assert -1 == res.get_linear(a)
    assert 1 == res.degree()
    assert a in res.variables()
