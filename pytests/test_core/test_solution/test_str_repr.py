import time

import pytest

from aqmodels import Model, Variable, Environment, Vtype, Solution, Timer


def vars() -> tuple[tuple[Variable, ...], Environment]:
    env = Environment()
    with env:
        variables = [
            Variable("b_0"),
            Variable("b_1"),
            Variable("s_0", vtype=Vtype.Spin),
            Variable("s_1", vtype=Vtype.Spin),
            Variable("i_0", vtype=Vtype.Integer),
            Variable("i_1", vtype=Vtype.Integer),
            Variable("r_0", vtype=Vtype.Real),
            Variable("r_1", vtype=Vtype.Real),
        ]

    return tuple(variables), env


@pytest.fixture
def model(request) -> tuple[Model, tuple[Variable, ...]]:
    variables, env = vars()
    model = Model(env=env)
    model.objective = variables[0] * 1
    for v in variables[1:]:
        model.objective -= v
    return model, variables


@pytest.mark.solution
def test_simple_solution_print(model: tuple[Model, tuple[Variable, ...]]):
    m, (b0, b1, s0, s1, i0, i1, r0, r1) = model
    timer = Timer.start()
    time.sleep(1)
    timing = timer.stop()
    timing.qpu = 0.012231
    with m.environment:
        sol = Solution.from_dicts(
            [
                {
                    b0: 0,
                    b1: 1,
                    s0: -1,
                    s1: 1,
                    i0: -10,
                    i1: 42,
                    r0: -324.12,
                    r1: 100_000_000,
                },
                {b0: 0, b1: 1, s0: -1, s1: 1, i0: -100, i1: 42, r0: -3.12, r1: -10.1},
                {b0: 1, b1: 1, s0: -1, s1: 1, i0: -100, i1: 42, r0: -3.12, r1: -10.1},
            ],
            timing=timing,
        )
    print()
    print(sol.print(max_line_length=50, max_chars_per_var=5, show_metadata="left", max_lines=2))
    print()
    print('=' * 80)
    print()
    print(sol.print(max_line_length=50, max_chars_per_var=5, show_metadata="right", max_lines=2))
