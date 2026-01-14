import pickle

import pytest
from luna_model import Solution, Vtype


@pytest.fixture()
def solution() -> Solution:
    return Solution(
        samples=[
            {"b0": 1, "s0": +1, "i0": 2, "r0": +2.2, "b1": 0},
            {"b0": 0, "s0": -1, "i0": 3, "r0": +3.3, "b1": 0},
            {"b0": 1, "s0": +1, "i0": 4, "r0": -4.4, "b1": 0},
        ],
        vtypes=[Vtype.Binary, Vtype.Spin, Vtype.Integer, Vtype.Real, Vtype.Binary],
        raw_energies=[0.3, 1.2, -200],
        counts=[1, 2, 3],
        constraints=[
            {"c0": True, "c1": True, "c2": True},
            {"c0": False, "c1": True, "c2": False},
            {"c0": False, "c1": True, "c2": True},
        ],
        variables_bounds={
            "b0": [True, True, True],
            "s0": [True, True, True],
            "i0": [True, True, True],
            "r0": [True, True, True],
            "b1": [True, True, True],
        },
        feasible=[True, False, False],
    )


def test_pickle_solution(solution: Solution):
    blob = pickle.dumps(solution)
    solution_loaded = pickle.loads(blob)
    print(solution)
    print(solution_loaded)
    assert solution == solution_loaded
