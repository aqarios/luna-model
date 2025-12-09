from luna_model import Variable, Environment


def test_var_str():
    v = Variable("v", env=Environment())
    assert "v" == str(v)

def test_var_repr():
    env = Environment()
    v = Variable("v", env=env)
    assert f"Var(id={v.id}, env_id={env.id})" == repr(v)
