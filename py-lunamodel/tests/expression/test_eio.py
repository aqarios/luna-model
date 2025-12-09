from luna_model import Environment, Variable, Expression, Vtype


def test_expr_str():
    empty = Expression(Environment())
    assert "0" == str(empty)


def test_expr_lin_str():
    with Environment():
        a = Variable("a")
        b = Variable("b")
        c = Variable("c")
        d = Variable("d")
        e = Variable("e")

    lin = a + b + c + d + e
    assert ["a", "b", "c", "d", "e"] == sorted(str(lin).split(" + "))


def test_expr_quad_str():
    with Environment():
        a = Variable("a")
        b = Variable("b")
        c = Variable("c")
        d = Variable("d")
        e = Variable("e")

    lin = a * b + c * d + e * a
    assert ["a * b", "a * e", "c * d"] == sorted(str(lin).split(" + "))


def test_expr_quad_ho():
    with Environment():
        a = Variable("a")
        b = Variable("b")
        c = Variable("c")
        d = Variable("d")
        e = Variable("e")

    lin = a * b * c + a * d * e
    assert ["a * b * c", "a * d * e"] == sorted(str(lin).split(" + "))


def test_expr_lin_quad():
    with Environment():
        a = Variable("a")
        b = Variable("b")
        c = Variable("c")
        d = Variable("d")
        e = Variable("e")

    lin = a + b + c + d * e
    assert ["a", "b", "c", "d * e"] == sorted(str(lin).split(" + "))


def test_expr_lin_ho():
    with Environment():
        a = Variable("a")
        b = Variable("b")
        c = Variable("c")
        d = Variable("d")
        e = Variable("e")

    lin = a + b + c * d * e
    assert ["a", "b", "c * d * e"] == sorted(str(lin).split(" + "))


def test_expr_lin_quad_ho():
    with Environment():
        a = Variable("a")
        b = Variable("b")
        c = Variable("c")
        d = Variable("d")
        e = Variable("e")

    lin = a + b + a * b + c * d * e
    assert ["a", "a * b", "b", "c * d * e"] == sorted(str(lin).split(" + "))


def test_expr_pow_bin():
    with Environment():
        a = Variable("a")
    lin = a**20
    assert "a" == str(lin)


def test_expr_pow_int():
    with Environment():
        a = Variable("a", Vtype.INTEGER)
    lin = a**20
    assert "a^20" == str(lin)


def test_expr_lin_quad_ho_pow_int():
    with Environment():
        a = Variable("a", Vtype.INTEGER)
        b = Variable("b", Vtype.REAL)
    res = b + a * b + a**20
    assert ["a * b", "a^20", "b"] == sorted(str(res).split(" + "))


def test_expr_neg():
    with Environment():
        a = Variable("a", Vtype.INTEGER)
    res = -a
    assert "-a" == str(res)


def test_expr_sub():
    with Environment():
        a = Variable("a", Vtype.INTEGER)
        b = Variable("b", Vtype.SPIN)
    res = a - b
    assert "a - b" == str(res)

def test_expr_sub2():
    with Environment():
        a = Variable("a", Vtype.INTEGER)
        b = Variable("b", Vtype.SPIN)
        c = Variable("c", Vtype.SPIN)
    res = a - b + c
    assert "a - b + c" == str(res)

def test_expr_sub3():
    with Environment():
        a = Variable("a", Vtype.INTEGER)
        b = Variable("b", Vtype.SPIN)
        c = Variable("c", Vtype.SPIN)
    res = c + a - b
    assert "a - b + c" == str(res)
