from luna_model import Environment, Variable


def test_var_hash():
    v = Variable("v", env=Environment())
    assert hash(v) is not None
