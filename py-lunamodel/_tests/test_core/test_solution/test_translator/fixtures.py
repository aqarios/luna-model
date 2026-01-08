from dataclasses import dataclass
from dimod import SampleSet, as_samples
import numpy as np
import pytest
from luna_model import Model, Vtype, Environment, Variable, Bounds
from numpy.typing import NDArray
from random import Random, shuffle
from typing import Any, Callable

import pytest
from numpy import unique
from _tests.test_core.utils import random_bool, random_int


@pytest.fixture()
def aws_model() -> Model:
    m = Model(name="TestModel")
    x0 = m.add_variable("x0")
    m.objective = x0 * 1
    x1 = m.add_variable("x1", vtype=Vtype.Real)
    m.objective += x0 * x1 * -1
    x2 = m.add_variable("x2")
    x3 = m.add_variable("x3", vtype=Vtype.Integer, lower=0, upper=30)
    x4 = m.add_variable("x4")
    m.objective += (
        x0 * x1 * 12.213
        + x1 * x2 * 0.5
        + x0 * x2 * -3
        + 1
        + x0 * x3 * 1848482
        + x1 * x4
    )
    m.constraints.add_constraint(x0 + x2 <= 1)
    m.constraints.add_constraint(x0 + x2 <= 1, "my_constraint")
    return m


@pytest.fixture()
def aws_result() -> dict[str, NDArray]:
    return {
        "samples": np.array(
            [
                [0, 1, 1, 0, 0],
                [1, 0, 1, 0, 0],
                [0, 1, 1, 0, 0],
                [0, 0, 1, 0, 0],
            ]
        ),
        "energies": np.array([-2.0, -1.0, -2.0, -1.0]),
    }


@dataclass
class DwaveResult:
    sampleset: SampleSet
    counts: list[int]
    energy: list[float]
    samples: list[list]


@pytest.fixture()
def dwave_result() -> DwaveResult:
    samples_raw = [
        {"x0": 0, "x1": 1, "x2": 1},
        {"x0": 0, "x1": 0, "x2": 1},
        {"x0": 0, "x1": 1, "x2": 0},
    ]
    samples = [list(sample.values()) for sample in samples_raw]
    counts = [1, 2, 3]
    energy = [-1.0, 0.0, 1.0]
    sampleset = SampleSet.from_samples(
        as_samples(samples_raw),
        "BINARY",
        energy,
        num_occurrences=np.array(counts),
    )
    return DwaveResult(sampleset, counts, energy, samples)


def mock_env(n_variables: int, vtype: Vtype = Vtype.Binary) -> Environment:
    env = Environment()
    with env:
        for i in range(n_variables):
            _ = Variable(str(f"x{i}"), vtype=vtype)
    return env


@pytest.fixture()
def np_model() -> Model:
    m = Model(name="TestModel")
    with m.environment:
        x0 = Variable("x0")
        m.objective = x0 * 1
        x1 = Variable("x1", vtype=Vtype.Real)
        m.objective += x0 * x1 * -1
        x2 = Variable("x2")
        x3 = Variable("x3", vtype=Vtype.Integer, bounds=Bounds(0, 30))
        x4 = Variable("x4")
        m.objective += (
            x0 * x1 * 12.213
            + x1 * x2 * 0.5
            + x0 * x2 * -3
            + 1
            + x0 * x3 * 1848482
            + x1 * x4
        )
        m.constraints.add_constraint(x0 + x2 <= 1)
        m.constraints.add_constraint(x0 + x2 <= 1, "my_constraint")
    return m


@pytest.fixture()
def np_result() -> tuple[NDArray, NDArray]:
    return (
        np.array(
            [
                [0, 1, 1, 0, 0],
                [1, 0, 1, 0, 0],
                [0, 1, 1, 0, 0],
                [0, 0, 1, 0, 0],
            ]
        ),
        np.array([-2.0, -1.0, -2.0, -1.0]),
    )


def fake_qctrl_result(
    rand: Random, length: int, cost: float, num_samples: int
) -> tuple[dict[str, Any], Callable[[list], str]]:
    def random_bitstring() -> str:
        return "".join([str(int(random_bool(rand))) for _ in range(length)])

    assignment = [length - 1 - i for i in range(length)]
    shuffle(assignment)
    mapped = {f"n[{i}]": a for i, a in enumerate(assignment)}
    # key position in sample actual, a position in solution false
    forward_assignment = dict(enumerate(assignment))
    reverse_assignment = {a: i for i, a in enumerate(assignment)}

    def adjust_ordering(actual: list | str) -> str:
        # need to move each value to the A value of the mapping.
        # the actual is the correct bitstring in the expected order.
        return "".join([str(actual[forward_assignment[i]]) for i in range(len(actual))])

    def reverse_adjust(other: list) -> str:
        return "".join([str(other[reverse_assignment[i]]) for i in range(len(other))])

    best = adjust_ordering(random_bitstring())
    best_dist = {best: random_int(rand)}

    base_samples = [adjust_ordering(random_bitstring()) for _ in range(num_samples - 1)]
    all_samples = [best, *base_samples]
    if unique(all_samples).shape[0] != num_samples:
        return fake_qctrl_result(rand, length, cost, num_samples)

    more_dist = {sample: random_int(rand) for sample in base_samples}

    return {
        "solution_bitstring": best,
        "solution_bitstring_cost": cost,
        "final_bitstring_distribution": {**best_dist, **more_dist},
        "variables_to_bitstring_index_map": mapped,
    }, reverse_adjust


@pytest.fixture()
def zib_model() -> Model:
    m = Model(name="TestModel")
    with m.environment:
        pennies = Variable("Pennies", vtype=Vtype.Integer, bounds=Bounds(lower=1))
        nickels = Variable("Nickels", vtype=Vtype.Integer, bounds=Bounds(lower=1))
        dimes = Variable("Dimes", vtype=Vtype.Integer, bounds=Bounds(lower=1))
        quarters = Variable("Quarters", vtype=Vtype.Integer, bounds=Bounds(lower=1))
        dollars = Variable("Dollars", vtype=Vtype.Integer, bounds=Bounds(lower=1))
        cu = Variable("Cu", vtype=Vtype.Real, bounds=Bounds(upper=1000))
        ni = Variable("Ni", vtype=Vtype.Real, bounds=Bounds(upper=50))
        zi = Variable("Zi", vtype=Vtype.Real, bounds=Bounds(upper=50))
        mn = Variable("Mn", vtype=Vtype.Real, bounds=Bounds(upper=50))
    m.objective = (
        0.01 * pennies + 0.05 * nickels + 0.1 * dimes + 0.25 * quarters + 1 * dollars
    )
    m.constraints += (
        0.06 * pennies
        + 3.8 * nickels
        + 2.1 * dimes
        + 5.2 * quarters
        + 7.2 * dollars
        - cu
        == 0,
        "Copper",
    )
    m.constraints += (
        1.2 * nickels + 0.2 * dimes + 0.5 * quarters + 0.2 * dollars - ni == 0,
        "Nickel",
    )
    m.constraints += 2.4 * pennies + 0.5 * dollars - zi == 0, "Zinc"
    m.constraints += 0.3 * dollars - mn == 0, "Manganese"
    return m


@pytest.fixture()
def zib_model_quadratic() -> Model:
    m = Model(name="TestModel")
    with m.environment:
        pennies = Variable("Pennies", vtype=Vtype.Integer, bounds=Bounds(lower=1))
        nickels = Variable("Nickels", vtype=Vtype.Integer, bounds=Bounds(lower=1))
        dimes = Variable("Dimes", vtype=Vtype.Integer, bounds=Bounds(lower=1))
        quarters = Variable("Quarters", vtype=Vtype.Integer, bounds=Bounds(lower=1))
        dollars = Variable("Dollars", vtype=Vtype.Integer, bounds=Bounds(lower=1))
        cu = Variable("Cu", vtype=Vtype.Real, bounds=Bounds(upper=1000))
        ni = Variable("Ni", vtype=Vtype.Real, bounds=Bounds(upper=50))
        zi = Variable("Zi", vtype=Vtype.Real, bounds=Bounds(upper=50))
        mn = Variable("Mn", vtype=Vtype.Real, bounds=Bounds(upper=50))
    m.objective = (
        0.01 * pennies * nickels
        + 0.05 * nickels * dollars
        + 0.1 * dimes
        + 0.25 * quarters
        + 1 * dollars
        + 5 * nickels * dollars
    )
    m.constraints += (
        0.06 * pennies
        + 3.8 * nickels
        + 2.1 * dimes
        + 5.2 * quarters
        + 7.2 * dollars
        - cu
        == 0,
        "Copper",
    )
    m.constraints += (
        1.2 * nickels + 0.2 * dimes + 0.5 * quarters + 0.2 * dollars - ni == 0,
        "Nickel",
    )
    m.constraints += 2.4 * pennies + 0.5 * dollars - zi == 0, "Zinc"
    m.constraints += 0.3 * dollars - mn == 0, "Manganese"
    return m
