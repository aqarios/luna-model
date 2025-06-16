import time
from pathlib import Path

import numpy as np
import pytest

from aqmodels import Variable, Vtype, Model, Solution, Environment, Sense, Timer

path = Path("./pytests/test_core/test_serialization/test_versions/data/v0/sol/")


@pytest.fixture
def model_samples():
    model = Model("MyModel")

    with model.environment:
        b1 = Variable("b1", vtype=Vtype.Binary)
        b2 = Variable("b2", vtype=Vtype.Binary)
        s1 = Variable("s1", vtype=Vtype.Spin)
        s2 = Variable("s2", vtype=Vtype.Spin)
        i1 = Variable("i1", vtype=Vtype.Integer)
        i2 = Variable("i2", vtype=Vtype.Integer)
        r1 = Variable("r1", vtype=Vtype.Real)
        r2 = Variable("r2", vtype=Vtype.Real)

    model.objective = (
            1 * b1 - 2 * b2 + 3 * s1 - 4 * s2 + 5 * i1 - 6 * i2 + 7 * r1 - 8 * r2
    )

    samples = [
        {b1: 0, b2: 1, s1: -1, s2: 1, i1: 3, i2: 42, r1: 0, r2: 12.34},
        {b1: 0, b2: 0, s1: -1, s2: -1, i1: 4, i2: 38, r1: 0.34, r2: 11.7923},
        {b1: 1, b2: 1, s1: -1, s2: 1, i1: 0, i2: 12, r1: 0.001, r2: 23.4}
    ]

    return model, samples


def test_v0_v1_equality_empty_sol():
    with Environment():
        sol = Solution.from_dict({})
    with open(path / "sol_empty", "rb") as f:
        reconstructed = Solution.decode(f.read())
        assert reconstructed == sol


def test_v0_v1_equality_sol_no_eval(
        model_samples: tuple[Model, list[dict[Variable, int | float]]],
):
    model, samples = model_samples
    sol = Solution.from_dicts(samples, env=model.environment)
    with open(path / "sol_no_eval", "rb") as f:
        reconstructed = Solution.decode(f.read())
        assert reconstructed == sol


def test_v0_v1_equality_sol_eval(
        model_samples: tuple[Model, list[dict[Variable, int | float]]],
):
    model, samples = model_samples
    sol = Solution.from_dicts(samples, model=model)
    with open(path / "sol_eval", "rb") as f:
        reconstructed = Solution.decode(f.read())
        assert reconstructed == sol


def test_v0_v1_equality_sol_sense_max(
        model_samples: tuple[Model, list[dict[Variable, int | float]]],
):
    model, samples = model_samples
    model.set_sense(Sense.Max)
    sol = Solution.from_dicts(samples, model=model)
    with open(path / "sol_eval_sense_max", "rb") as f:
        reconstructed = Solution.decode(f.read())
        # print()
        # print(reconstructed)
        # print("---------")
        # print(sol)
        # print("---------")
        # print(reconstructed.sense)
        # print(sol.sense)
        assert reconstructed == sol


def test_v0_v1_equality_sol_eval_timing(
        model_samples: tuple[Model, list[dict[Variable, int | float]]],
):
    model, samples = model_samples
    timer = Timer.start()
    time.sleep(1.23)
    timing = timer.stop()
    timing.qpu = 0.0142
    sol = Solution.from_dicts(samples, model=model, counts=[2, 1, 1], timing=timing)
    with open(path / "sol_eval_timing", "rb") as f:
        reconstructed = Solution.decode(f.read())
        assert reconstructed.variable_names == sol.variable_names
        assert reconstructed.best_sample_idx == sol.best_sample_idx
        assert np.all(reconstructed.obj_values == sol.obj_values)
        assert np.all(reconstructed.counts == sol.counts)
        assert np.all(reconstructed.raw_energies == sol.raw_energies)
        assert reconstructed.runtime.qpu == sol.runtime.qpu
        total_seconds = reconstructed.runtime.total_seconds
        assert total_seconds > 1.23 and total_seconds < 1.231
