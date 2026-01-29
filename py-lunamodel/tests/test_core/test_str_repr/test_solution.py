from contextlib import nullcontext as does_not_raise

import pytest

from luna_model import Solution, Vtype

samples_str = """
{
  [1, 1, 2, 2.0]: 1,
  [0, -1, 3, 3.0]: 2,
  [1, 1, 4, -4.23]: 3
}""".strip("\n")

sol_str_1 = """
x0 x1 x2    x3 │ feas raw obj count
 1  1  4 -4.23 │    ? 2.0   ?     3
 0 -1  3   3.0 │    ? 5.0   ?     2
 1  1  2   2.0 │    ? 6.0   ?     1

Total samples: 3
Total variables: 4""".strip("\n")

sol_str_2 = """
b0 b1 b2 b3 b4 b5 b6 b7 b8 b9 b10 b11 b12 b13 b14 b15     │ feas raw obj count
 1  1  1  1  1  1  1  1  1  1   1   1   1   1   1   1 ... │    ?   ?   ?     1
 0  0  0  0  0  0  0  0  0  0   0   0   0   0   0   0 ... │    ?   ?   ?     1

Total samples: 2
Total variables: 30""".strip("\n")


@pytest.fixture()
def solution(request) -> Solution:
    _ = request
    return Solution(
        samples=[
            {"x0": 1, "x1": +1, "x2": 2, "x3": +2.0},
            {"x0": 0, "x1": -1, "x2": 3, "x3": +3.0},
            {"x0": 1, "x1": +1, "x2": 4, "x3": -4.23},
        ],
        vtypes=[Vtype.BINARY, Vtype.SPIN, Vtype.INTEGER, Vtype.REAL],
        counts=[1, 2, 3],
        raw_energies=[6.0, 5.0, 2.0],
    )


@pytest.mark.parametrize("solution", [()], indirect=True)
def test_sample(solution: Solution):
    sample = list(solution.samples)[0]
    assert str(sample) == "[1, 1, 2, 2.0]"


@pytest.mark.parametrize("solution", [()], indirect=True)
def test_samples(solution: Solution):
    samples = solution.samples
    assert str(samples) == samples_str


@pytest.mark.parametrize("solution", [()], indirect=True)
def test_model(solution: Solution):
    assert str(solution) == sol_str_1

    solution_2 = Solution(
        samples=[
            {f"b{i}": 1 for i in range(30)},
            {f"b{i}": 0 for i in range(30)},
        ],
        vtypes=[Vtype.BINARY] * 30,
    )
    assert str(solution_2) == sol_str_2

    with does_not_raise():
        repr(solution)
        repr(solution_2)
