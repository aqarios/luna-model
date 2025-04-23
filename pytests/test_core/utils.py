import random as r
import sys
import numpy as np

from itertools import permutations
from dimod import generators, BinaryQuadraticModel, Vartype, ConstrainedQuadraticModel


def todo(msg: str | None = None):
    assert False, msg


def make_seed() -> int:
    seed = r.randint(0, 2**32 - 1)
    print(
        f"""
************************
Random Seed = {seed}
************************
""",
        file=sys.stderr,
    )
    return seed


def random(seed: int) -> float:
    rand = r.Random(seed)
    return rand.random()


def random_int(rand: r.Random):
    return rand.randint(0, 2**16 - 1)


def random_bool(rand: r.Random):
    return bool(rand.randint(0, 1))


def check_equality(variables, p, f, value):
    permuts = permutations(variables, p)
    base = next(permuts)
    base_value = f(base)
    assert base_value == value
    for permut in permuts:
        assert f(permut) == base_value


def assert_offset(expr, value):
    assert expr.get_offset() == value, f"offset != {value}, is {expr.get_offset()}"


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


def generate_bqms(
    n_models: int, rand: r.Random, n_vars_max: int = 100
) -> list[BinaryQuadraticModel]:
    out = []
    for _ in range(n_models):
        n_vars = rand.randint(1, n_vars_max)
        density = rand.random() * (1 - 1 / n_vars)
        num_interactions = int(density * n_vars**2 / 2)
        vartype = Vartype.BINARY if rand.randint(0, 1) == 0 else Vartype.SPIN
        bqm = generators.gnm_random_bqm(
            [f"x{i}" for i in range(n_vars)],
            num_interactions,
            vartype,
            random_state=random_int(rand),
        )
        out.append(bqm)
    return out


def generate_cqms(n_models: int, rand: r.Random) -> list[ConstrainedQuadraticModel]:
    out = []
    # LINEAR
    n_lin = n_models // 2
    n_quad = n_models - n_lin
    for _ in range(n_lin):
        n_items = rand.randint(1, 20)
        cqm = generators.random_knapsack(n_items, seed=random_int(rand))
        out.append(cqm)
    # QUADRATIC
    for _ in range(n_quad):
        num_items = rand.randint(1, 10)
        values = [rand.randint(1, 10) for _ in range(num_items)]
        weights = [rand.randint(1, 10) for _ in range(num_items)]

        # Generate a symmetric profit matrix with zeros on the diagonal
        profits = np.array(
            [[rand.randint(0, 5) for _ in range(num_items)] for _ in range(num_items)]
        )
        profits = np.triu(profits, 1)
        profits += profits.T
        profits = profits
        capacity = int(0.5 * sum(weights))
        cqm = generators.quadratic_knapsack(values, weights, profits, capacity)
        out.append(cqm)
    return out
