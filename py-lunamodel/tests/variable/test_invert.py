from luna_model import Variable, Environment, Vtype


def test_inverse_binary():
    env = Environment()
    b = Variable("b", vtype=Vtype.BINARY, env=env)
    inv_b = ~b
    assert f"~{b.name}" == inv_b.name
    assert b != inv_b
    assert inv_b != b
    assert ~inv_b == b
    assert b == ~inv_b
    assert ~(~b) == ~inv_b
    assert ~(~b) == b
    assert ~inv_b == ~(~b)
    assert b == ~(~b)
