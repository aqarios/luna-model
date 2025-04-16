import pytest
from aqmodels import (
    Solution,
    Vtype,
)


@pytest.fixture
def solution() -> Solution:
    return Solution.build(
        component_types=[Vtype.Binary, Vtype.Spin, Vtype.Integer, Vtype.Real, Vtype.Binary],
        binary_cols=[[1, 0, 1], [0, 0, 0]],
        spin_cols=[[+1, -1, +1]],
        int_cols=[[2, 3, -4]],
        real_cols=[[2.2, 3.3, -4.4]],
        raw_energies=[0.3, 1.2, -200],
        num_occurrences=[1, 2, 3],
    )




@pytest.mark.solution_translation
def test_solution_encoding_decoding(solution):
    blob = solution.encode()
    solution_back = Solution.decode(blob)
    assert solution == solution_back
