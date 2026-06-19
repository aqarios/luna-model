from luna_model import Variable, Environment, Solution

def test_eval_sample_var():
    with Environment():
        x = Variable("x")
        y = Variable("y")
        z = Variable("z")

    expr = x + y - 3 * z
    val = expr.evaluate_sample({x: 0, y: 1, z: 1})
    assert -2.0 == val


def test_eval_sample_str():
    with Environment():
        x = Variable("x")
        y = Variable("y")
        z = Variable("z")

    expr = x + y - 3 * z
    val = expr.evaluate_sample({"x": 0, "y": 1, "z": 1})
    assert -2.0 == val


def test_eval_sample_mixed():
    with Environment():
        x = Variable("x")
        y = Variable("y")
        z = Variable("z")
        a = Variable("a")

    expr = x + y - 3 * z + a
    val = expr.evaluate_sample({x: 0, "y": 1, "z": 1, a: 0})
    assert -2.0 == val



def test_eval_sample_view_1():
    with Environment():
        x = Variable("x")
        y = Variable("y")
        z = Variable("z")

        sol = Solution.from_dict({x: 0, y: 1, z: 1})

    expr = x + y - 3 * z

    val = expr.evaluate_sample(sol.samples[0])
    assert -2.0 == val

def test_eval_sample_view_2():
    with Environment():
        x = Variable("x")
        y = Variable("y")
        z = Variable("z")

        sol = Solution.from_dict({x: 0, y: 1, z: 1})

    expr = x + y - 3 * z
    val = expr.evaluate_sample(sol[0].sample)
    assert -2.0 == val

def test_eval_sample_more_vars_in_env():
    with Environment():
        x = Variable("x")
        y = Variable("y")
        z = Variable("z")
        _ = Variable("w")

    expr = x + y - 3 * z
    val = expr.evaluate_sample({x: 0, y: 1, z: 1})
    assert -2.0 == val

