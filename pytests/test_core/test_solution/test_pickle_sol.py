import pytest
import pickle

from luna_model import Solution, Vtype


@pytest.fixture
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


def test_pickle_solution(solution: Solution):
    blob = pickle.dumps(solution)
    solution_loaded = pickle.loads(blob)
    assert solution == solution_loaded
