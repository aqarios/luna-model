from contextlib import nullcontext as does_not_raise

import pytest

from aqmodels import Solution, Vtype

sol_str_1 = """{
  [1, 1, 2, 2.0]: 1,
  [0, -1, 3, 3.0]: 2,
  [1, 1, -4, 4.23]: 3,
}"""

sol_str_2 = """{
  [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1]: 1,
  [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0]: 1,
}"""


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
        num_occurrences=[1, 2, 3],
        raw_energies=[6.0, 5.0, 2.0],
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
    assert str(samples) == sol_str_1


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
