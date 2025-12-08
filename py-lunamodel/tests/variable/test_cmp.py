import pytest

from luna_model import Variable, Vtype, Environment, Comparator, Constraint, Expression

vtypes = [Vtype.BINARY, Vtype.SPIN, Vtype.INTEGER, Vtype.REAL]


@pytest.mark.parametrize("vtype", vtypes)
def test_cmp_slf(vtype):
    with Environment():
        lhs = Variable("lhs", vtype)
    assert lhs.is_equal(lhs)


@pytest.mark.parametrize("lhs_vtype", vtypes)
@pytest.mark.parametrize("rhs_vtype", vtypes)
def test_cmp(lhs_vtype, rhs_vtype):
    with Environment():
        lhs = Variable("lhs", lhs_vtype)
        rhs = Variable("rhs", rhs_vtype)
    assert not lhs.is_equal(rhs)
    assert not rhs.is_equal(lhs)


@pytest.mark.parametrize("lhs_vtype", vtypes)
@pytest.mark.parametrize("rhs_vtype", vtypes)
def test_cmp_to_expr_constr_eq(lhs_vtype, rhs_vtype):
    print()
    with Environment():
        lhs = Variable("lhs", lhs_vtype)
        rhs = Variable("rhs", rhs_vtype)

    res = lhs == (rhs * 1)
    assert isinstance(res, Constraint)
    assert (lhs - rhs).is_equal(res.lhs)
    assert 0 == res.rhs
    assert Comparator.EQ == res.comparator


@pytest.mark.parametrize("lhs_vtype", vtypes)
@pytest.mark.parametrize("rhs_vtype", vtypes)
def test_cmp_to_expr_constr_le(lhs_vtype, rhs_vtype):
    print()
    with Environment():
        lhs = Variable("lhs", lhs_vtype)
        rhs = Variable("rhs", rhs_vtype)

    res = lhs <= (rhs * 1)
    assert isinstance(res, Constraint)
    assert (lhs - rhs).is_equal(res.lhs)
    assert 0 == res.rhs
    assert Comparator.LE == res.comparator


@pytest.mark.parametrize("lhs_vtype", vtypes)
@pytest.mark.parametrize("rhs_vtype", vtypes)
def test_cmp_to_expr_constr_ge(lhs_vtype, rhs_vtype):
    print()
    with Environment():
        lhs = Variable("lhs", lhs_vtype)
        rhs = Variable("rhs", rhs_vtype)

    res = lhs >= (rhs * 1)
    assert isinstance(res, Constraint)
    assert (lhs - rhs).is_equal(res.lhs)
    assert 0 == res.rhs
    assert Comparator.GE == res.comparator
