from itertools import permutations


def check_equality(variables, p, f, value):
    permuts = permutations(variables, p)
    base = next(permuts)
    base_value = f(base)
    assert base_value == value
    for permut in permuts:
        assert f(permut) == base_value


def assert_offset(expr, value):
    assert expr.get_offset() == value


def assert_linear(expr, variables, value):
    check_equality(variables, 1, lambda v: expr.get_linear(v[0]), value)


def assert_quadratic(expr, variables, value):
    check_equality(variables, 2, lambda v: expr.get_quadratic(*v), value)


def assert_higher_order(expr, variables, value, p_size=None):
    if not p_size:
        check_equality(variables, len(variables), expr.get_higher_order, value)
    else:
        check_equality(variables, p_size, expr.get_higher_order, value)


def assert_higher_order_all(expr, variables, value):
    for p_size in range(3, len(variables) + 1):
        check_equality(variables, p_size, expr.get_higher_order, value)
