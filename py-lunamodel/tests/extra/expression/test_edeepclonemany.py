import pytest
from luna_model import Expression, Environment, Variable
from luna_model.errors import DifferentEnvsError

def test_same_env():
    env = Environment()
    with env:
        a = Variable("a")
        b = Variable("b")
        c = Variable("c")
        d = Variable("d")

    expr_a = a * b
    expr_b = a * c
    expr_c = b * d
    expr_d = a + b + c * d
    exprs = [expr_a, expr_b, expr_c, expr_d]
    exprs = [expr_a]

    env_ids_in = [e.environment.id for e in exprs]
    cloned = Expression.deep_clone_many(exprs)
    env_ids_out = [e.environment.id for e in cloned]

    assert env_ids_in != env_ids_out
    assert 1 == len(set(env_ids_out))

def test_diff_env():
    with Environment():
        a = Variable("a")
        b = Variable("b")
        c = Variable("c")
        d = Variable("d")
    
    expr_a = a * b
    expr_b = a * c
    expr_c = b * d
    expr_d = a + b + c * d

    with Environment():
        x = Variable("x")
        y = Variable("y")
        odd_one_out = x * y

    exprs = [expr_a, expr_b, odd_one_out, expr_c, expr_d]

    with pytest.raises(DifferentEnvsError):
        _ = Expression.deep_clone_many(exprs)

