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
        vtypes=[Vtype.BINARY, Vtype.SPIN, Vtype.INTEGER, Vtype.REAL, Vtype.BINARY],
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


@pytest.fixture()
def solution_many_binary() -> Solution:
    return Solution(
        samples=[
            {"b0": 0, "b1": 1, "b2": 1},
            {"b0": 0, "b1": 0, "b2": 1},
            {"b0": 0, "b1": 1, "b2": 0},
        ],
        vtypes=[Vtype.BINARY, Vtype.BINARY, Vtype.BINARY],
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
