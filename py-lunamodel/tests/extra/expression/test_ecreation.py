import random
from luna_model import Environment, Expression


def test_expr_init_in_context():
    with Environment():
        _ = Expression()


def test_expr_init_explicit():
    _ = Expression(Environment())


def test_expr_init_const_in_context():
    with Environment():
        _ = Expression.const(random.random())


def test_expr_init_const_explicit():
    _ = Expression.const(random.random(), Environment())
