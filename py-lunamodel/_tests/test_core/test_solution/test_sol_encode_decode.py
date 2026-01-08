import pytest
from luna_model import Solution, Vtype


@pytest.fixture()
def solution() -> Solution:
    return Solution._build(  # type: ignore[reportAttributeAccessIssue]
        component_types=[
            Vtype.Binary,
            Vtype.Spin,
            Vtype.Integer,
            Vtype.Real,
            Vtype.Binary,
        ],
        binary_cols=[[1, 0, 1], [0, 0, 0]],
        spin_cols=[[+1, -1, +1]],
        int_cols=[[2, 3, -4]],
        real_cols=[[2.2, 3.3, -4.4]],
        raw_energies=[0.3, 1.2, -200],
        counts=[1, 2, 3],
        constraints=[
            [True, True, True],
            [False, True, False],
            [False, True, True],
        ],
        variable_bounds=[
            [True, True, True, True, True],
            [True, True, True, True, True],
            [True, True, True, True, True],
        ],
        feasible=[True, False, False],
    )


@pytest.fixture()
def solution_many_binary() -> Solution:
    return Solution._build(  # type: ignore[reportAttributeAccessIssue]
        component_types=[
            Vtype.Binary,
            Vtype.Binary,
            Vtype.Binary,
        ],
        binary_cols=[[0, 0, 0], [1, 0, 1], [1, 1, 0]],
        raw_energies=[-1, 0, -1],
        counts=[1, 2, 3],
    )


def test_solution_encoding_decoding(solution):
    blob = solution.encode()
    solution_back = Solution.decode(blob)
    assert solution == solution_back


def test_solution_encoding_decoding_many(solution_many_binary):
    blob = solution_many_binary.encode()
    solution_back = Solution.decode(blob)
    assert solution_many_binary == solution_back
