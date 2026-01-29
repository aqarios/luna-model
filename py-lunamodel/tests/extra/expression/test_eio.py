from luna_model import Environment, Expression, Variable, Vtype


def test_expr_str():
    empty = Expression(Environment())
    assert str(empty) == "0"


def test_expr_lin_str():
    with Environment():
        a = Variable("a")
        b = Variable("b")
        c = Variable("c")
        d = Variable("d")
        e = Variable("e")

    lin = a + b + c + d + e
    assert sorted(str(lin).split(" + ")) == ["a", "b", "c", "d", "e"]


def test_expr_quad_str():
    with Environment():
        a = Variable("a")
        b = Variable("b")
        c = Variable("c")
        d = Variable("d")
        e = Variable("e")

    lin = a * b + c * d + e * a
    assert sorted(str(lin).split(" + ")) == ["a b", "a e", "c d"]


def test_expr_quad_ho():
    with Environment():
        a = Variable("a")
        b = Variable("b")
        c = Variable("c")
        d = Variable("d")
        e = Variable("e")

    lin = a * b * c + a * d * e
    assert sorted(str(lin).split(" + ")) == ["a b c", "a d e"]


def test_expr_lin_quad():
    with Environment():
        a = Variable("a")
        b = Variable("b")
        c = Variable("c")
        d = Variable("d")
        e = Variable("e")

    lin = a + b + c + d * e
    assert sorted(str(lin).split(" + ")) == ["a", "b", "c", "d e"]


def test_expr_lin_ho():
    with Environment():
        a = Variable("a")
        b = Variable("b")
        c = Variable("c")
        d = Variable("d")
        e = Variable("e")

    lin = a + b + c * d * e
    assert sorted(str(lin).split(" + ")) == ["a", "b", "c d e"]


def test_expr_lin_quad_ho():
    with Environment():
        a = Variable("a")
        b = Variable("b")
        c = Variable("c")
        d = Variable("d")
        e = Variable("e")

    lin = a + b + a * b + c * d * e
    assert sorted(str(lin).split(" + ")) == ["a", "a b", "b", "c d e"]


def test_expr_pow_bin():
    with Environment():
        a = Variable("a")
    lin = a**20
    assert str(lin) == "a"


def test_expr_pow_int():
    with Environment():
        a = Variable("a", Vtype.INTEGER)
    lin = a**20
    assert str(lin) == "a^20"


def test_expr_lin_quad_ho_pow_int():
    with Environment():
        a = Variable("a", Vtype.INTEGER)
        b = Variable("b", Vtype.REAL)
    res = b + a * b + a**20
    assert sorted(str(res).split(" + ")) == ["a b", "a^20", "b"]


def test_expr_neg():
    with Environment():
        a = Variable("a", Vtype.INTEGER)
    res = -a
    assert str(res) == "-a"


def test_expr_sub():
    with Environment():
        a = Variable("a", Vtype.INTEGER)
        b = Variable("b", Vtype.SPIN)
    res = a - b
    assert str(res) == "a - b"


def test_expr_sub2():
    with Environment():
        a = Variable("a", Vtype.INTEGER)
        b = Variable("b", Vtype.SPIN)
        c = Variable("c", Vtype.SPIN)
    res = a - b + c
    assert str(res) == "a - b + c"


def test_expr_sub3():
    with Environment():
        a = Variable("a", Vtype.INTEGER)
        b = Variable("b", Vtype.SPIN)
        c = Variable("c", Vtype.SPIN)
    res = c + a - b
    assert str(res) == "a - b + c"
