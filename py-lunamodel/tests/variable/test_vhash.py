from luna_model import Variable, Environment


def test_var_hash():
    v = Variable("v", env=Environment())
    assert hash(v) is not None
