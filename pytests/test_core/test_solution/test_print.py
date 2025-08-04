import time

import pytest

from aqmodels import Environment, Model, Sense, Solution, Timer, Variable, Vtype


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


@pytest.fixture
def solution(request, model: Model):
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
        )
    return sol


@pytest.mark.solution
def test_row_hide(solution: Solution):
    s = solution.print(layout="row", show_metadata="hide")
    assert (
        s
        == """
b_0      0     0     1
b_1      1     1     1
s_0     -1    -1    -1
s_1      1     1     1
i_0    -10  -100  -100
i_1     42    42    42
r_0 -3.2e2 -3.12 -3.12
r_1  1.0e8 -10.1 -10.1

Total samples: 3
Total variables: 8""".strip("\n")
    )


@pytest.mark.solution
def test_row_before_max_lines(solution: Solution):
    s = solution.print(layout="row", show_metadata="before", max_lines=7)
    assert (
        s
        == """
  feasible      ?     ?     ?
raw energy      ?     ?     ?
 objective      ?     ?     ?
     count      1     1     1
─────────────────────────────
       b_0      0     0     1
       b_1      1     1     1
       s_0     -1    -1    -1
       s_1      1     1     1
       i_0    -10  -100  -100
       i_1     42    42    42
       r_0 -3.2e2 -3.12 -3.12
...

Total samples: 3
Total variables: 8""".strip("\n")
    )


def test_row_after_max_line_length(solution: Solution):
    s = solution.print(layout="row", show_metadata="after", max_line_length=27)
    assert (
        s
        == """
       b_0      0     0 ...
       b_1      1     1 ...
       s_0     -1    -1 ...
       s_1      1     1 ...
       i_0    -10  -100 ...
       i_1     42    42 ...
       r_0 -3.2e2 -3.12 ...
       r_1  1.0e8 -10.1 ...
───────────────────────────
  feasible      ?     ? ...
raw energy      ?     ? ...
 objective      ?     ? ...
     count      1     1 ...

Total samples: 3
Total variables: 8""".strip("\n")
    )


def test_row_after_max_line_length_dots_too_long(solution: Solution):
    s = solution.print(layout="row", show_metadata="after", max_line_length=26)
    assert (
        s
        == """
       b_0      0 ...
       b_1      1 ...
       s_0     -1 ...
       s_1      1 ...
       i_0    -10 ...
       i_1     42 ...
       r_0 -3.2e2 ...
       r_1  1.0e8 ...
─────────────────────
  feasible      ? ...
raw energy      ? ...
 objective      ? ...
     count      1 ...

Total samples: 3
Total variables: 8""".strip("\n")
    )


def test_col_hide(solution: Solution):
    s = solution.print(layout="column", show_metadata="hide")
    assert (
        s
        == """
b_0 b_1 s_0 s_1  i_0 i_1    r_0   r_1
  0   1  -1   1  -10  42 -3.2e2 1.0e8
  0   1  -1   1 -100  42  -3.12 -10.1
  1   1  -1   1 -100  42  -3.12 -10.1

Total samples: 3
Total variables: 8""".strip("\n")
    )


def test_col_after_full(solution: Solution):
    s = solution.print(layout="column", show_metadata="after")
    assert (
        s
        == """
b_0 b_1 s_0 s_1  i_0 i_1    r_0   r_1 │ feas raw obj count
  0   1  -1   1  -10  42 -3.2e2 1.0e8 │    ?   ?   ?     1
  0   1  -1   1 -100  42  -3.12 -10.1 │    ?   ?   ?     1
  1   1  -1   1 -100  42  -3.12 -10.1 │    ?   ?   ?     1

Total samples: 3
Total variables: 8""".strip("\n")
    )


def test_col_before_full(solution: Solution):
    print()
    s = solution.print(layout="column", show_metadata="before")
    assert (
        s
        == """
feas raw obj count │ b_0 b_1 s_0 s_1  i_0 i_1    r_0   r_1
   ?   ?   ?     1 │   0   1  -1   1  -10  42 -3.2e2 1.0e8
   ?   ?   ?     1 │   0   1  -1   1 -100  42  -3.12 -10.1
   ?   ?   ?     1 │   1   1  -1   1 -100  42  -3.12 -10.1

Total samples: 3
Total variables: 8""".strip("\n")
    )


def test_col_after_truncated(solution: Solution):
    s = solution.print(
        layout="column", show_metadata="after", max_lines=2, max_line_length=50
    )
    assert (
        s
        == """
b_0 b_1 s_0 s_1  i_0 i_1     │ feas raw obj count
  0   1  -1   1  -10  42 ... │    ?   ?   ?     1
  0   1  -1   1 -100  42 ... │    ?   ?   ?     1
...

Total samples: 3
Total variables: 8""".strip("\n")
    )


def test_col_after_truncated_dots_too_long(solution: Solution):
    s = solution.print(
        layout="column", show_metadata="after", max_lines=2, max_line_length=48
    )
    assert (
        s
        == """
b_0 b_1 s_0 s_1  i_0     │ feas raw obj count
  0   1  -1   1  -10 ... │    ?   ?   ?     1
  0   1  -1   1 -100 ... │    ?   ?   ?     1
...

Total samples: 3
Total variables: 8""".strip("\n")
    )


@pytest.fixture
def model_with_sol(request) -> tuple[Model, Solution]:
    minimize: bool = request.param
    m = Model()
    with m.environment:
        a = Variable("a")
        b = Variable("b")
        c = Variable("c")
        d = Variable("d")
        e = Variable("e")
    m.objective = -a - 5 * b - 6 * c + 2 * d + 2 * e
    m.constraints += a + b <= 1
    if not minimize:
        m.set_sense(Sense.Max)

    sol = Solution.from_dicts(
        [
            # infeasible, e: -10
            *(2 * [{a: 1, b: 1, c: 1, d: 1, e: 0}]),
            {a: 1, b: 1, c: 1, d: 0, e: 1},
            # feasible, e: -9
            {a: 0, b: 1, c: 1, d: 1, e: 0},
            *(3 * [{a: 0, b: 1, c: 1, d: 0, e: 1}]),
            # infeasible, e: -6
            {a: 1, b: 1, c: 0, d: 0, e: 0},
            # feasible, e: -5
            {a: 0, b: 1, c: 0, d: 0, e: 0},
            # feasible, e: -11
            {a: 0, b: 1, c: 1, d: 0, e: 0},
        ],
        model=m,
    )

    return m, sol


@pytest.mark.parametrize("model_with_sol", [True], indirect=True)
def test_sorted_solution_minimize_col(model_with_sol: tuple[Model, Solution]):
    _, sol = model_with_sol
    assert (
        sol.print()
        == """
a b c d e │ feas raw   obj count
0 1 1 0 0 │    t   ? -11.0     1
0 1 1 0 1 │    t   ?  -9.0     3
0 1 1 1 0 │    t   ?  -9.0     1
0 1 0 0 0 │    t   ?  -5.0     1
1 1 1 1 0 │    f   ? -10.0     2
1 1 1 0 1 │    f   ? -10.0     1
1 1 0 0 0 │    f   ?  -6.0     1

Total samples: 7
Total variables: 5""".strip("\n")
    )


@pytest.mark.parametrize("model_with_sol", [True], indirect=True)
def test_sorted_solution_minimize_row(model_with_sol: tuple[Model, Solution]):
    _, sol = model_with_sol
    assert (
        sol.print(layout="row")
        == """
         a     0     0     0     0     1     1     1
         b     1     1     1     1     1     1     1
         c     1     1     1     0     1     1     0
         d     0     0     1     0     1     0     0
         e     0     1     0     0     0     1     0
────────────────────────────────────────────────────
  feasible     t     t     t     t     f     f     f
raw energy     ?     ?     ?     ?     ?     ?     ?
 objective -11.0  -9.0  -9.0  -5.0 -10.0 -10.0  -6.0
     count     1     3     1     1     2     1     1

Total samples: 7
Total variables: 5""".strip("\n")
    )


@pytest.mark.parametrize("model_with_sol", [False], indirect=True)
def test_sorted_solution_maximize(model_with_sol: tuple[Model, Solution]):
    _, sol = model_with_sol
    assert (
        sol.print()
        == """
a b c d e │ feas raw   obj count
0 1 0 0 0 │    t   ?  -5.0     1
0 1 1 0 1 │    t   ?  -9.0     3
0 1 1 1 0 │    t   ?  -9.0     1
0 1 1 0 0 │    t   ? -11.0     1
1 1 0 0 0 │    f   ?  -6.0     1
1 1 1 1 0 │    f   ? -10.0     2
1 1 1 0 1 │    f   ? -10.0     1

Total samples: 7
Total variables: 5""".strip("\n")
    )


@pytest.mark.parametrize("model_with_sol", [False], indirect=True)
def test_sorted_solution_maximize_row(model_with_sol: tuple[Model, Solution]):
    _, sol = model_with_sol
    print()
    assert (
        sol.print(layout="row")
        == """
         a     0     0     0     0     1     1     1
         b     1     1     1     1     1     1     1
         c     0     1     1     1     0     1     1
         d     0     0     1     0     0     1     0
         e     0     1     0     0     0     0     1
────────────────────────────────────────────────────
  feasible     t     t     t     t     f     f     f
raw energy     ?     ?     ?     ?     ?     ?     ?
 objective  -5.0  -9.0  -9.0 -11.0  -6.0 -10.0 -10.0
     count     1     3     1     1     1     2     1

Total samples: 7
Total variables: 5""".strip("\n")
    )
