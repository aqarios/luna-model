from luna_model import Expression, Variable, Environment, Vtype


def test_imul_binaries():
    with Environment():
        a = Variable("a", Vtype.BINARY)
        expr = Expression()

    expr += 1
    expr *= a
    expr *= a

    assert 1 == expr.degree()


def test_imul_spins():
    with Environment():
        a = Variable("a", Vtype.SPIN)
        expr = Expression()

    expr += 1
    expr *= a
    expr *= a

    assert 0 == expr.degree()


def test_imul_ints():
    with Environment():
        a = Variable("a", Vtype.INTEGER)
        expr = Expression()

    expr += 1
    expr *= a
    expr *= a

    assert 2 == expr.degree()


def test_imul_reals():
    with Environment():
        a = Variable("a", Vtype.REAL)
        expr = Expression()

    expr += 1
    expr *= a
    expr *= a

    assert 2 == expr.degree()
