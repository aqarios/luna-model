from contextlib import nullcontext as does_not_raise

import pytest

from aqmodels import Solution, Vtype

samples_str = """{
  [1, 1, 2, 2.0]: 1,
  [0, -1, 3, 3.0]: 2,
  [1, 1, -4, 4.23]: 3,
}"""

sol_str_1 = """x_0 x_1 x_2   x_3 │ feas   raw obj count
  1   1   2 2.000 │    ? 6.000   ?     1
  0  -1   3 3.000 │    ? 5.000   ?     2
  1   1  -4 4.230 │    ? 2.000   ?     3

Total rows: 3
Total columns: 4"""

sol_str_2 = """b0 b1 b2 b3 b4 b5 b6 b7 b8 b9 b10 b11 b12 b13 b14 b15 b16     │ feas raw obj count
 1  1  1  1  1  1  1  1  1  1   1   1   1   1   1   1   1 ... │    ?   ?   ?     1
 0  0  0  0  0  0  0  0  0  0   0   0   0   0   0   0   0 ... │    ?   ?   ?     1

Total rows: 2
Total columns: 30"""


@pytest.fixture
def solution(request) -> Solution:
    return Solution.build(
        component_types=[
            Vtype.Binary,
            Vtype.Spin,
            Vtype.Integer,
            Vtype.Real,
        ],
        binary_cols=[[1, 0, 1]],
        spin_cols=[[+1, -1, +1]],
        int_cols=[[2, 3, -4]],
        real_cols=[[2.0, 3.0, 4.23]],
        counts=[1, 2, 3],
        raw_energies=[6.0, 5.0, 2.0],
        variable_names=list(f"x_{i}" for i in range(4))
    )


@pytest.mark.str_repr
@pytest.mark.parametrize("solution", [()], indirect=True)
def test_sample(solution: Solution):
    sample = list(solution.samples)[0]
    assert str(sample) == "[1, 1, 2, 2.0]"


@pytest.mark.str_repr
@pytest.mark.parametrize("solution", [()], indirect=True)
def test_samples(solution: Solution):
    samples = solution.samples
    assert str(samples) == samples_str


@pytest.mark.str_repr
@pytest.mark.parametrize("solution", [()], indirect=True)
def test_model(solution: Solution):
    assert str(solution) == sol_str_1

    solution_2 = Solution.build(
        component_types=[Vtype.Binary] * 30,
        binary_cols=[[1, 0]] * 30,
    )
    assert str(solution_2) == sol_str_2

    with does_not_raise():
        repr(solution)
        repr(solution_2)
