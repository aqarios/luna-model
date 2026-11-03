from luna_model import Model, Solution, Vtype


def _evaluate(samples):
    m = Model()
    x = m.add_variable("x", vtype=Vtype.BINARY)
    y = m.add_variable("y", vtype=Vtype.BINARY)
    m.objective = x + y
    m.add_constraint(x <= 0, name="b_x")
    m.add_constraint(y <= 0, name="a_y")
    m.add_constraint(x + y <= 1, name="c_sum")
    return m.evaluate(Solution.from_dicts(samples, env=m.environment))


def test_no_violation_returns_none():
    sol = _evaluate([{"x": 0, "y": 0}])
    assert sol.feasibility_ratio() == 1.0
    assert sol.highest_constraint_violation() is None


def test_no_constraint_information_returns_none():
    m = Model()
    m.add_variable("x")
    sol = Solution.from_dicts([{"x": 1}], env=m.environment)
    assert sol.highest_constraint_violation() is None


def test_most_violated_constraint_is_returned():
    # b_x is violated twice, c_sum once, a_y once.
    sol = _evaluate([{"x": 1, "y": 0}, {"x": 1, "y": 1}])
    assert sol.highest_constraint_violation() == "b_x"


def test_tie_goes_to_alphabetically_first_name():
    # a_y and b_x are both violated once.
    sol = _evaluate([{"x": 1, "y": 0}, {"x": 0, "y": 1}])
    assert sol.highest_constraint_violation() == "a_y"
