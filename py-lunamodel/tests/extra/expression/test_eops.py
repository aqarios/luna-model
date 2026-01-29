from luna_model import Environment, Expression, Variable, Vtype


def test_imul_binaries():
    with Environment():
        a = Variable("a", Vtype.BINARY)
        expr = Expression()

    expr += 1
    expr *= a
    expr *= a

    assert expr.degree() == 1


def test_imul_spins():
    with Environment():
        a = Variable("a", Vtype.SPIN)
        expr = Expression()

    expr += 1
    expr *= a
    expr *= a

    assert expr.degree() == 0


def test_imul_ints():
    with Environment():
        a = Variable("a", Vtype.INTEGER)
        expr = Expression()

    expr += 1
    expr *= a
    expr *= a

    assert expr.degree() == 2


def test_imul_reals():
    with Environment():
        a = Variable("a", Vtype.REAL)
        expr = Expression()

    expr += 1
    expr *= a
    expr *= a

    assert expr.degree() == 2
