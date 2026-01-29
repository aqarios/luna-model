import pytest

from luna_model import Environment, Solution, Variable, Vtype


def test_add_var():
    env = Environment()
    with env:
        vars = [
            Variable("b"),
            Variable("s", vtype=Vtype.SPIN),
            Variable("i", vtype=Vtype.INTEGER),
            Variable("r", vtype=Vtype.REAL),
        ]

    base_names = [v.name for v in vars]
    base_vals = [0, -1, 3, 3.14]
    base = {n: v for n, v in zip(base_names, base_vals)}
    sol = Solution.from_dict(base, env=env)
    sol.add_var("b2", [1])
    assert sol.variable_names == [*base_names, "b2"]
    assert sol.samples.tolist()[0] == [*base_vals, 1]
    assert sol[0].sample.to_dict() == {**base, "b2": 1}

    s2 = Variable("s2", vtype=Vtype.SPIN, env=env)
    sol.add_var(s2, [+1])
    assert sol.variable_names == [*base_names, "b2", "s2"]
    assert sol.samples.tolist()[0] == [*base_vals, 1, +1]
    assert sol[0].sample.to_dict() == {**base, "b2": 1, "s2": +1}

    sol.add_var("i2", [6], vtype=Vtype.SPIN)
    assert sol.variable_names == [*base_names, "b2", "s2", "i2"]
    assert sol.samples.tolist()[0] == [*base_vals, 1, +1, 6]
    assert sol[0].sample.to_dict() == {**base, "b2": 1, "s2": +1, "i2": 6}

    sol.add_var("r2", [6.28], vtype=Vtype.REAL)
    assert sol.variable_names == [*base_names, "b2", "s2", "i2", "r2"]
    assert sol.samples.tolist()[0] == [*base_vals, 1, +1, 6, 6.28]
    assert sol[0].sample.to_dict() == {**base, "b2": 1, "s2": +1, "i2": 6, "r2": 6.28}


def test_add_vars():
    env = Environment()
    with env:
        vars = [
            Variable("b"),
            Variable("s", vtype=Vtype.SPIN),
            Variable("i", vtype=Vtype.INTEGER),
            Variable("r", vtype=Vtype.REAL),
        ]

    base_names = [v.name for v in vars]
    base_vals = [0, -1, 3, 3.14]
    base = {n: v for n, v in zip(base_names, base_vals)}
    sol = Solution.from_dict(base, env=env)
    i2 = Variable("i2", vtype=Vtype.INTEGER, env=env)
    sol.add_vars(
        ["b2", "s2", i2, "r2"],
        [[1], [+1], [6], [6.28]],
        vtypes=[Vtype.BINARY, Vtype.SPIN, None, Vtype.REAL],
    )
    assert sol.variable_names == [*base_names, "b2", "s2", "i2", "r2"]
    assert sol.samples.tolist()[0] == [*base_vals, 1, +1, 6, 6.28]
    assert sol[0].sample.to_dict() == {**base, "b2": 1, "s2": +1, "i2": 6, "r2": 6.28}


def test_add_vars_only_var():
    env = Environment()
    with env:
        vars = [
            Variable("b"),
            Variable("s", vtype=Vtype.SPIN),
            Variable("i", vtype=Vtype.INTEGER),
            Variable("r", vtype=Vtype.REAL),
        ]

    base_names = [v.name for v in vars]
    base_vals = [0, -1, 3, 3.14]
    base = {n: v for n, v in zip(base_names, base_vals)}
    sol = Solution.from_dict(base, env=env)

    b2 = Variable("b2", env=env)
    s2 = Variable("s2", vtype=Vtype.SPIN, env=env)
    i2 = Variable("i2", vtype=Vtype.INTEGER, env=env)
    r2 = Variable("r2", vtype=Vtype.REAL, env=env)

    sol.add_vars([b2, s2, i2, r2], [[1], [+1], [6], [6.28]])
    assert sol.variable_names == [*base_names, "b2", "s2", "i2", "r2"]
    assert sol.samples.tolist()[0] == [*base_vals, 1, +1, 6, 6.28]
    assert sol[0].sample.to_dict() == {**base, "b2": 1, "s2": +1, "i2": 6, "r2": 6.28}


def test_remove_var():
    env = Environment()
    with env:
        vars = [
            Variable("b"),
            Variable("s", vtype=Vtype.SPIN),
            Variable("i", vtype=Vtype.INTEGER),
            Variable("r", vtype=Vtype.REAL),
        ]

    base_names = [v.name for v in vars]
    base_vals = [0, -1, 3, 3.14]
    base = {n: v for n, v in zip(base_names, base_vals)}
    sol = Solution.from_dict(base, env=env)

    sol.remove_var("b")
    assert sol.variable_names == ["s", "i", "r"]
    assert sol.samples.tolist()[0] == [-1, 3, 3.14]
    assert sol[0].sample.to_dict() == {"s": -1, "i": 3, "r": 3.14}

    sol.remove_var(vars[2])
    assert sol.variable_names == ["s", "r"]
    assert sol.samples.tolist()[0] == [-1, 3.14]
    assert sol[0].sample.to_dict() == {"s": -1, "r": 3.14}

    sol.remove_var(vars[3].name)
    assert sol.variable_names == ["s"]
    assert sol.samples.tolist()[0] == [-1]
    assert sol[0].sample.to_dict() == {"s": -1}

    sol.remove_var("s")
    assert sol.variable_names == []
    assert sol.samples.tolist() == []
    with pytest.raises(IndexError):
        _ = sol[0].sample.to_dict()


def test_remove_vars():
    env = Environment()
    with env:
        vars = [
            Variable("b"),
            Variable("s", vtype=Vtype.SPIN),
            Variable("i", vtype=Vtype.INTEGER),
            Variable("r", vtype=Vtype.REAL),
        ]

    base_names = [v.name for v in vars]
    base_vals = [0, -1, 3, 3.14]
    base = {n: v for n, v in zip(base_names, base_vals)}
    sol = Solution.from_dict(base, env=env)

    sol.remove_vars([vars[0], "s", vars[2], "r"])
    assert sol.variable_names == []
    assert sol.samples.tolist() == []
    with pytest.raises(IndexError):
        _ = sol[0].sample.to_dict()
