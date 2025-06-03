from random import Random
from time import sleep
from typing import Any

import pytest

from aqmodels import Environment, Timer, Variable
from aqmodels.translator import QctrlTranslator
from pytests.test_core.utils import make_seed, random, random_bool, random_int

REPS: int = 20
MAX_VARS: int = 20


def fake_qctrl_result(rand: Random, len: int, cost: float) -> dict[str, Any]:
    return {
        "solution_bitstring": [int(random_bool(rand)) for _ in range(len)],
        "final_aggregate_cost": cost,
    }


@pytest.mark.solution_translation
def test_qctrl_translator_constructed():
    for _ in range(REPS):
        rand = Random(make_seed())
        sample_len = rand.randint(2, MAX_VARS)
        fake_result = fake_qctrl_result(rand, sample_len, random(random_int(rand)))

        env = Environment()
        with env:
            _ = [Variable(f"x{i}") for i in range(sample_len)]
            sol = QctrlTranslator.to_aq(fake_result)

        samples = sol.samples.tolist()
        assert len(samples) == 1
        assert len(samples[0]) == sample_len
        assert samples[0] == fake_result["solution_bitstring"]
        assert sol.counts.tolist() == [1]
        assert sol.raw_energies.tolist() == [fake_result["final_aggregate_cost"]]
        assert sol.runtime is None

        for result in sol.results:
            assert result.constraints is None
            assert result.feasible is None

        results = list(sol.results)
        assert len(results) == 1


@pytest.mark.solution_translation
def test_qctrl_translator_constructed_explicit_env():
    for _ in range(REPS):
        rand = Random(make_seed())
        sample_len = rand.randint(2, MAX_VARS)
        fake_result = fake_qctrl_result(rand, sample_len, random(random_int(rand)))

        env = Environment()
        with env:
            _ = [Variable(f"x{i}") for i in range(sample_len)]

        sol = QctrlTranslator.to_aq(fake_result, env=env)

        samples = sol.samples.tolist()
        assert len(samples) == 1
        assert len(samples[0]) == sample_len
        assert samples[0] == fake_result["solution_bitstring"]
        assert sol.counts.tolist() == [1]
        assert sol.raw_energies.tolist() == [fake_result["final_aggregate_cost"]]
        assert sol.runtime is None

        for result in sol.results:
            assert result.constraints is None
            assert result.feasible is None
            assert result.obj_value is None
            assert result.counts == 1
            assert result.raw_energy == fake_result["final_aggregate_cost"]

        results = list(sol.results)
        assert len(results) == 1


@pytest.mark.solution_translation
def test_qctrl_translator_constructed_with_time():
    for _ in range(REPS):
        rand = Random(make_seed())
        sample_len = rand.randint(2, MAX_VARS)
        fake_result = fake_qctrl_result(rand, sample_len, random(random_int(rand)))
        timer = Timer.start()
        sleep(0.3)
        timing = timer.stop()

        env = Environment()
        with env:
            _ = [Variable(f"x{i}") for i in range(sample_len)]
            sol = QctrlTranslator.to_aq(fake_result, timing)

        samples = sol.samples.tolist()
        assert len(samples) == 1
        assert len(samples[0]) == sample_len
        assert samples[0] == fake_result["solution_bitstring"]
        assert sol.counts.tolist() == [1]
        assert sol.raw_energies.tolist() == [fake_result["final_aggregate_cost"]]
        assert sol.runtime is not None
        assert round(sol.runtime.total.total_seconds(), 1) == 0.3
        assert round(sol.runtime.total_seconds, 1) == 0.3
        assert sol.runtime.qpu is None

        for result in sol.results:
            assert result.constraints is None
            assert result.feasible is None

        results = list(sol.results)
        assert len(results) == 1


@pytest.mark.solution_translation
def test_qctrl_translator_constructed_vars():
    for _ in range(REPS):
        rand = Random(make_seed())
        sample_len = rand.randint(2, MAX_VARS)
        fake_result = fake_qctrl_result(rand, sample_len, random(random_int(rand)))

        env = Environment()
        with env:
            _ = [Variable(f"x{i}") for i in range(sample_len)]
            sol = QctrlTranslator.to_aq(fake_result)

        samples = sol.samples.tolist()
        assert len(samples) == 1
        assert len(samples[0]) == sample_len
        assert samples[0] == fake_result["solution_bitstring"]
        assert sol.counts.tolist() == [1]
        assert sol.raw_energies.tolist() == [fake_result["final_aggregate_cost"]]
        assert sol.runtime is None

        for result in sol.results:
            assert result.constraints is None
            assert result.feasible is None
            assert result.obj_value is None
            assert result.counts == 1
            assert result.raw_energy == fake_result["final_aggregate_cost"]

        results = list(sol.results)
        assert len(results) == 1
