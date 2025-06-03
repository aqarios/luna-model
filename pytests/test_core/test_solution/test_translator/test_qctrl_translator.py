from random import Random
from typing import Any

import pytest
from time import sleep

from aqmodels import (
    Timer,
    Variable,
    Environment,
)
from aqmodels.translator import QctrlTranslator
from pytests.test_core.utils import random, make_seed, random_bool, random_int

REPS: int = 100
MAX_VARS: int = 200


def fake_qctrl_result(
    rand: Random, len: int, cost: float, num_samples: int
) -> dict[str, Any]:
    def random_bitstring():
        return "".join([str(int(random_bool(rand))) for _ in range(len)])

    best = random_bitstring()
    best_dist = {best: random_int(rand)}
    more_dist = {random_bitstring(): random_int(rand) for _ in range(num_samples - 1)}
    return {
        "solution_bitstring": best,
        "solution_bitstring_cost": cost,
        "final_bitstring_distribution": {**best_dist, **more_dist},
        "variables_to_bitstring_index_map": {
            f"n[{i}]": len - 1 - i for i in range(len)
        },
    }


def actual_qctrl_result():
    return {
        "best_parameters": [-0.2578826696230263, 0.5623785079452714],
        "iteration_count": 11,
        "provider_job_ids": [
            "hasjlkdhflkjasdhf0",
            "hasjlkdhflkjasdhf1",
            "hasjlkdhflkjasdhf2",
            "hasjlkdhflkjasdhf3",
            "hasjlkdhflkjasdhf4",
            "hasjlkdhflkjasdhf5",
            "hasjlkdhflkjasdhf6",
            "hasjlkdhflkjasdhf7",
            "hasjlkdhflkjasdhf8",
            "hasjlkdhflkjasdhf9",
            "hasjlkdhflkjasdf10",
        ],
        "solution_bitstring": "11",
        "solution_bitstring_cost": -2.0,
        "final_bitstring_distribution": {"11": 8192, "01": 100},
        "variables_to_bitstring_index_map": {"n[0]": 1, "n[1]": 0},
    }


def test_qctrl_base():
    res = actual_qctrl_result()
    with Environment():
        _ = [Variable(f"x{i}") for i in range(2)]
        sol = QctrlTranslator.to_aq(res)
    print(sol)


@pytest.mark.solution_translation
def test_qctrl_translator_constructed():
    for _ in range(REPS):
        rand = Random(make_seed())
        sample_len = rand.randint(2, MAX_VARS)
        num_samples = rand.randint(1, sample_len // 2)
        print("NUM SAMPLES = ", num_samples)
        fake_result = fake_qctrl_result(
            rand, sample_len, random(random_int(rand)), num_samples
        )

        env = Environment()
        with env:
            _ = [Variable(f"x{i}") for i in range(sample_len)]
            sol = QctrlTranslator.to_aq(fake_result)

        samples = sol.samples.tolist()
        assert len(samples) == num_samples, "number of samples does not match"
        assert len(samples[0]) == sample_len, "sample len (num variables) does not match"
        min_energy_index = None
        min_energy = None
        for i, item in enumerate(sol.raw_energies):
            if item is None:
                continue
            min_energy_index = i if (min_energy is None) or (min_energy > item) else min_energy_index
        assert min_energy_index is not None
        assert (
            "".join([str(e) for e in reversed(samples[min_energy_index])])
            == fake_result["solution_bitstring"]
        )
        assert len(sol.counts.tolist()) == num_samples
        assert fake_result["solution_bitstring_cost"] in sol.raw_energies.tolist()
        assert sol.runtime is None

        for result in sol.results:
            assert result.constraints is None
            assert result.feasible is None

        results = list(sol.results)
        assert len(results) == num_samples


@pytest.mark.solution_translation
def test_qctrl_translator_constructed_explicit_env():
    for _ in range(REPS):
        rand = Random(make_seed())
        sample_len = rand.randint(2, MAX_VARS)
        num_samples = rand.randint(1, sample_len // 2)
        fake_result = fake_qctrl_result(
            rand, sample_len, random(random_int(rand)), num_samples
        )

        env = Environment()
        with env:
            _ = [Variable(f"x{i}") for i in range(sample_len)]

        sol = QctrlTranslator.to_aq(fake_result, env=env)

        samples = sol.samples.tolist()
        assert len(samples) == num_samples, "number of samples does not match"
        assert len(samples[0]) == sample_len, "sample len (num variables) does not match"
        min_energy_index = None
        min_energy = None
        for i, item in enumerate(sol.raw_energies):
            if item is None:
                continue
            min_energy_index = i if (min_energy is None) or (min_energy > item) else min_energy_index
        assert min_energy_index is not None
        assert (
            "".join([str(e) for e in reversed(samples[min_energy_index])])
            == fake_result["solution_bitstring"]
        )
        assert len(sol.counts.tolist()) == num_samples
        assert fake_result["solution_bitstring_cost"] in sol.raw_energies.tolist()
        assert sol.runtime is None

        for result in sol.results:
            assert result.constraints is None
            assert result.feasible is None
            assert result.obj_value is None
            bs = "".join([str(e) for e in reversed([e for e in result.sample])])
            assert result.counts == fake_result["final_bitstring_distribution"][bs]
            if bs == fake_result["solution_bitstring"]:
                assert result.raw_energy == fake_result["solution_bitstring_cost"]

        results = list(sol.results)
        assert len(results) == num_samples


@pytest.mark.solution_translation
def test_qctrl_translator_constructed_with_time():
    for _ in range(REPS):
        rand = Random(make_seed())
        sample_len = rand.randint(2, MAX_VARS)
        num_samples = rand.randint(1, sample_len // 2)
        fake_result = fake_qctrl_result(
            rand, sample_len, random(random_int(rand)), num_samples
        )
        timer = Timer.start()
        sleep(0.3)
        timing = timer.stop()

        env = Environment()
        with env:
            _ = [Variable(f"x{i}") for i in range(sample_len)]
            sol = QctrlTranslator.to_aq(fake_result, timing)

        samples = sol.samples.tolist()
        assert len(samples) == num_samples, "number of samples does not match"
        assert len(samples[0]) == sample_len, "sample len (num variables) does not match"
        min_energy_index = None
        min_energy = None
        for i, item in enumerate(sol.raw_energies):
            if item is None:
                continue
            min_energy_index = i if (min_energy is None) or (min_energy > item) else min_energy_index
        assert min_energy_index is not None
        assert (
            "".join([str(e) for e in reversed(samples[min_energy_index])])
            == fake_result["solution_bitstring"]
        )
        assert len(sol.counts.tolist()) == num_samples
        assert fake_result["solution_bitstring_cost"] in sol.raw_energies.tolist()
        assert sol.runtime is not None
        assert round(sol.runtime.total.total_seconds(), 1) == 0.3
        assert round(sol.runtime.total_seconds, 1) == 0.3
        assert sol.runtime.qpu is None

        for result in sol.results:
            assert result.constraints is None
            assert result.feasible is None

        results = list(sol.results)
        assert len(results) == num_samples


@pytest.mark.solution_translation
def test_qctrl_translator_constructed_vars():
    for _ in range(REPS):
        rand = Random(make_seed())
        sample_len = rand.randint(2, MAX_VARS)
        num_samples = rand.randint(1, sample_len // 2)
        fake_result = fake_qctrl_result(
            rand, sample_len, random(random_int(rand)), num_samples
        )

        env = Environment()
        with env:
            _ = [Variable(f"x{i}") for i in range(sample_len)]
            sol = QctrlTranslator.to_aq(fake_result)

        samples = sol.samples.tolist()
        assert len(samples) == num_samples, "number of samples does not match"
        assert len(samples[0]) == sample_len, "sample len (num variables) does not match"
        min_energy_index = None
        min_energy = None
        for i, item in enumerate(sol.raw_energies):
            if item is None:
                continue
            min_energy_index = i if (min_energy is None) or (min_energy > item) else min_energy_index
        assert min_energy_index is not None
        assert (
            "".join([str(e) for e in reversed(samples[min_energy_index])])
            == fake_result["solution_bitstring"]
        )
        assert len(sol.counts.tolist()) == num_samples
        assert fake_result["solution_bitstring_cost"] in sol.raw_energies.tolist()
        assert sol.runtime is None

        for result in sol.results:
            assert result.constraints is None
            assert result.feasible is None
            assert result.obj_value is None
            bs = "".join([str(e) for e in reversed([e for e in result.sample])])
            assert result.counts == fake_result["final_bitstring_distribution"][bs]
            if bs == fake_result["solution_bitstring"]:
                assert result.raw_energy == fake_result["solution_bitstring_cost"]

        results = list(sol.results)
        assert len(results) == num_samples
