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
def test_row_no_metadata(solution: Solution):
    s = solution.print(layout="row", show_metadata="hide")
    assert s == """
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

@pytest.mark.solution
def test_row_before_lines(solution: Solution):
    s = solution.print(layout="row", show_metadata="before", max_lines=7)
    assert s == """
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


def test_row_after_cols(solution: Solution):
    s = solution.print(layout="row", show_metadata="after", max_line_length=20)
    assert s == """
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


def test_col_no_metadata(solution: Solution):
    s = solution.print(layout="column", show_metadata="hide")
    assert s == """
b_0 b_1 s_0 s_1  i_0 i_1    r_0   r_1
  0   1  -1   1  -10  42 -3.2e2 1.0e8
  0   1  -1   1 -100  42  -3.12 -10.1
  1   1  -1   1 -100  42  -3.12 -10.1

Total samples: 3
Total variables: 8""".strip("\n")


def test_col_after_full(solution: Solution):
    s = solution.print(layout="column", show_metadata="after")
    assert s == """
b_0 b_1 s_0 s_1  i_0 i_1    r_0   r_1 │ feas raw obj count
  0   1  -1   1  -10  42 -3.2e2 1.0e8 │    ?   ?   ?     1
  0   1  -1   1 -100  42  -3.12 -10.1 │    ?   ?   ?     1
  1   1  -1   1 -100  42  -3.12 -10.1 │    ?   ?   ?     1

Total samples: 3
Total variables: 8""".strip("\n")


def test_col_before_full(solution: Solution):
    print()
    s = solution.print(layout="column", show_metadata="before")
    assert s == """
feas raw obj count │ b_0 b_1 s_0 s_1  i_0 i_1    r_0   r_1
   ?   ?   ?     1 │   0   1  -1   1  -10  42 -3.2e2 1.0e8
   ?   ?   ?     1 │   0   1  -1   1 -100  42  -3.12 -10.1
   ?   ?   ?     1 │   1   1  -1   1 -100  42  -3.12 -10.1

Total samples: 3
Total variables: 8""".strip("\n")


def test_col_after_truncated(solution: Solution):
    s = solution.print(layout="column", show_metadata="after", max_lines=2, max_line_length=50)
    assert s == """
b_0 b_1 s_0 s_1  i_0 i_1     │ feas raw obj count
  0   1  -1   1  -10  42 ... │    ?   ?   ?     1
  0   1  -1   1 -100  42 ... │    ?   ?   ?     1
...

Total samples: 3
Total variables: 8""".strip("\n")


def test_col_after_truncated_dots_too_long(solution: Solution):
    s = solution.print(layout="column", show_metadata="after", max_lines=2, max_line_length=48)
    assert s == """
b_0 b_1 s_0 s_1  i_0     │ feas raw obj count
  0   1  -1   1  -10 ... │    ?   ?   ?     1
  0   1  -1   1 -100 ... │    ?   ?   ?     1
...

Total samples: 3
Total variables: 8""".strip("\n")
