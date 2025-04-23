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


@pytest.mark.str_repr
def test_model():
    sol = Solution.build(
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
    assert str(sol) == sol_str_1

    sol_2 = Solution.build(
        component_types=[Vtype.Binary] * 30,
        binary_cols=[[1, 0]] * 30,
    )
    assert str(sol_2) == sol_str_2

    with does_not_raise():
        repr(sol)
        repr(sol_2)
