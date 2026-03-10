import pytest
import time
import random
import numpy as np
from luna_model import Solution, Model, Vtype, Timer, Sense, Timing
from luna_model import quicksum


@pytest.fixture
def model() -> Model:
    model = Model()
    x = model.add_variable("x")
    y = model.add_variable("y")
    i = model.add_variable("i", vtype=Vtype.INTEGER)
    s = model.add_variable("s", vtype=Vtype.SPIN)
    r = model.add_variable("r", vtype=Vtype.REAL)
    model.objective = quicksum([x, y, i, s, r])
    model.constraints += x + y <= 4
    model.constraints += s + r >= 0
    return model

def make_timing() -> Timing:
    t = Timer.start()
    time.sleep(random.random())
    return t.stop()

def make_solutions(model: Model) -> tuple[list[Solution], list[Timing]]:
    timings = [make_timing(), make_timing(), make_timing()]
    solutions = [
            Solution(
                [
                    {"x": 0,"y": 0,"i": 2,"s": -1,"r": 0.7},
                    {"x": 1,"y": 0,"i": 0,"s": +1,"r": 5.2},
                ],
                counts=[2, 1],
                raw_energies=[0.2, 1.1],
                obj_values=[2.2, 3.1],
                feasible=[False, True],
                constraints=[{"c0": True, "c1": False}, {"c0": True, "c1": True}],
                variables_bounds={
                    "x": [True, True],
                    "y": [True, True],
                    "i": [True, True],
                    "s": [True, True],
                    "r": [True, True]
                },
                timing=timings[0],
                sense=Sense.MIN,
                env=model.environment
            ),
            Solution(
                [
                    {"x": 1,"y": 1,"i": 4,"s": +1,"r": 4.7},
                ],
                counts=[10],
                raw_energies=[3.2],
                obj_values=[5.2],
                feasible=[True],
                constraints=[{"c0": True, "c1": True}],
                variables_bounds={
                    "x": [True],
                    "y": [True],
                    "i": [True],
                    "s": [True],
                    "r": [True]
                },
                timing=timings[1],
                sense=Sense.MIN,
                env=model.environment
            ),
            Solution(
                [
                    {"x": 1,"y": 0,"i": 2,"s": +1,"r": 0},
                    {"x": 1,"y": 0,"i": 0,"s": +1,"r": 5.2},
                ],
                counts=[4, 2],
                raw_energies=[23, 1.1],
                obj_values=[25, 3.1],
                feasible=[True, True],
                constraints=[{"c0": True, "c1": True}, {"c0": True, "c1": True}],
                variables_bounds={
                    "x": [True, True],
                    "y": [True, True],
                    "i": [True, True],
                    "s": [True, True],
                    "r": [True, True]
                },
                timing=timings[2],
                sense=Sense.MIN,
                env=model.environment
            ),
        ]
    return solutions, timings

def test_merge_full(model: Model):
    solutions, timings = make_solutions(model)
    solution = Solution.from_many(solutions)
    assert sum([s.counts.sum() for s in solutions]) == solution.counts.sum()
    assert all([0.2, 1.1, 3.2, 23] == solution.raw_energies)
    assert all([2.2, 3.1, 5.2, 25] == solution.obj_values)
    assert solution.runtime is not None
    assert np.isclose(sum(t.total_seconds for t in timings), solution.runtime.total_seconds, atol=0.1)

    assert not solution[0].feasible
    assert solution[1].feasible
    assert solution[2].feasible
    assert solution[3].feasible

def test_merge_no_raw_in_one(model: Model):
    solutions, timings = make_solutions(model)
    solutions[1].raw_energies = None
    solution = Solution.from_many(solutions)
    assert sum([s.counts.sum() for s in solutions]) == solution.counts.sum()
    assert solution.raw_energies is None
    assert all([2.2, 3.1, 5.2, 25] == solution.obj_values)
    assert solution.runtime is not None
    assert np.isclose(sum(t.total_seconds for t in timings), solution.runtime.total_seconds, atol=0.1)

    assert not solution[0].feasible
    assert solution[1].feasible
    assert solution[2].feasible
    assert solution[3].feasible

def test_merge_no_rt_in_one(model: Model):
    solutions, _ = make_solutions(model)
    solutions.append(
            Solution(
                [
                    {"x": 0,"y": 0,"i": 2,"s": -1,"r": 0.7},
                    {"x": 1,"y": 0,"i": 0,"s": +1,"r": 5.2},
                ],
                counts=[2, 1],
                raw_energies=[0.2, 1.1],
                obj_values=[2.2, 3.1],
                feasible=[False, True],
                constraints=[{"c0": True, "c1": False}, {"c0": True, "c1": True}],
                variables_bounds={
                    "x": [True, True],
                    "y": [True, True],
                    "i": [True, True],
                    "s": [True, True],
                    "r": [True, True]
                },
                timing=None,
                sense=Sense.MIN,
                env=model.environment
            ),
    )
    solution = Solution.from_many(solutions)
    assert sum([s.counts.sum() for s in solutions]) == solution.counts.sum()
    assert all([0.2, 1.1, 3.2, 23] == solution.raw_energies)
    assert all([2.2, 3.1, 5.2, 25] == solution.obj_values)
    assert solution.runtime is None

    assert not solution[0].feasible
    assert solution[1].feasible
    assert solution[2].feasible
    assert solution[3].feasible

def test_merge_no_obj(model: Model):
    solutions, timings = make_solutions(model)
    solutions[0].obj_values = None
    solution = Solution.from_many(solutions)
    assert sum([s.counts.sum() for s in solutions]) == solution.counts.sum()
    assert all([0.2, 1.1, 3.2, 23] == solution.raw_energies)
    assert solution.obj_values is None
    assert solution.runtime is not None
    assert np.isclose(sum(t.total_seconds for t in timings), solution.runtime.total_seconds, atol=0.1)

    assert not solution[0].feasible
    assert solution[1].feasible
    assert solution[2].feasible
    assert solution[3].feasible

def test_merge_with_eval(model: Model):
    solutions, timings = make_solutions(model)
    solution = Solution.from_many(solutions, model)
    assert sum([s.counts.sum() for s in solutions]) == solution.counts.sum()
    assert all([0.2, 1.1, 3.2, 23] == solution.raw_energies)
    assert all([1.7, 7.2, 11.7, 4.0] == solution.obj_values)
    assert solution.runtime is not None
    assert np.isclose(sum(t.total_seconds for t in timings), solution.runtime.total_seconds, atol=0.1)

    assert not solution[0].feasible
    assert solution[1].feasible
    assert solution[2].feasible
    assert solution[3].feasible
