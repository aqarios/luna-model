from random import Random
from time import sleep

import pytest
from luna_model import Environment, Timer, Variable
from luna_model.translator import QctrlTranslator

from _tests.test_core.utils import make_seed, random, random_int

from .fixtures import fake_qctrl_result

REPS: int = 100
MAX_VARS: int = 200


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
        sol = QctrlTranslator.to_lm(res)
    print(sol)


def test_qctrl_translator_constructed():
    for _ in range(REPS):
        rand = Random(make_seed())
        sample_len = rand.randint(2, MAX_VARS)
        num_samples = rand.randint(1, max(sample_len // 2, 1))
        fake_result, reverser = fake_qctrl_result(
            rand, sample_len, random(random_int(rand)), num_samples
        )

        env = Environment()
        with env:
            _ = [Variable(f"x{i}") for i in range(sample_len)]
            sol = QctrlTranslator.to_lm(fake_result)

        samples = sol.samples.tolist()
        assert len(samples) == num_samples, "number of samples does not match"
        assert len(samples[0]) == sample_len, (
            "sample len (num variables) does not match"
        )
        assert sol.raw_energies is None
        assert len(sol.counts.tolist()) == num_samples
        assert sol.runtime is None

        for result in sol.results:
            assert result.constraints is None
            assert result.feasible is None

        results = list(sol.results)
        assert len(results) == num_samples


def test_qctrl_translator_constructed_explicit_env():
    for _ in range(REPS):
        rand = Random(make_seed())
        sample_len = rand.randint(2, MAX_VARS)
        num_samples = rand.randint(1, max(sample_len // 2, 1))
        fake_result, reverser = fake_qctrl_result(
            rand, sample_len, random(random_int(rand)), num_samples
        )

        env = Environment()
        with env:
            _ = [Variable(f"x{i}") for i in range(sample_len)]

        sol = QctrlTranslator.to_lm(fake_result, env=env)

        samples = sol.samples.tolist()
        assert len(samples) == num_samples, "number of samples does not match"
        assert len(samples[0]) == sample_len, (
            "sample len (num variables) does not match"
        )
        assert sol.raw_energies is None
        assert len(sol.counts.tolist()) == num_samples
        assert sol.runtime is None

        for result in sol.results:
            assert result.constraints is None
            assert result.feasible is None
            assert result.obj_value is None
            bs = reverser([e for e in result.sample])
            assert result.counts == fake_result["final_bitstring_distribution"][bs]

        results = list(sol.results)
        assert len(results) == num_samples


def test_qctrl_translator_constructed_with_time():
    for _ in range(REPS):
        rand = Random(make_seed())
        sample_len = rand.randint(2, MAX_VARS)
        num_samples = rand.randint(1, max(sample_len // 2, 1))
        fake_result, reverser = fake_qctrl_result(
            rand, sample_len, random(random_int(rand)), num_samples
        )
        timer = Timer.start()
        sleep(0.3)
        timing = timer.stop()

        env = Environment()
        with env:
            _ = [Variable(f"x{i}") for i in range(sample_len)]
            sol = QctrlTranslator.to_lm(fake_result, timing)

        samples = sol.samples.tolist()
        assert len(samples) == num_samples, "number of samples does not match"
        assert len(samples[0]) == sample_len, (
            "sample len (num variables) does not match"
        )
        assert sol.raw_energies is None
        assert len(sol.counts.tolist()) == num_samples
        assert sol.runtime is not None
        assert round(sol.runtime.total.total_seconds(), 1) == 0.3
        assert round(sol.runtime.total_seconds, 1) == 0.3
        assert sol.runtime.qpu is None

        for result in sol.results:
            assert result.constraints is None
            assert result.feasible is None

        results = list(sol.results)
        assert len(results) == num_samples


def test_qctrl_translator_constructed_vars():
    for _ in range(REPS):
        rand = Random(make_seed())
        sample_len = rand.randint(2, MAX_VARS)
        num_samples = rand.randint(1, max(sample_len // 2, 1))
        fake_result, reverser = fake_qctrl_result(
            rand, sample_len, random(random_int(rand)), num_samples
        )

        env = Environment()
        with env:
            _ = [Variable(f"x{i}") for i in range(sample_len)]
            sol = QctrlTranslator.to_lm(fake_result)

        samples = sol.samples.tolist()
        assert len(samples) == num_samples, "number of samples does not match"
        assert len(samples[0]) == sample_len, (
            "sample len (num variables) does not match"
        )
        assert sol.raw_energies is None
        assert len(sol.counts.tolist()) == num_samples
        assert sol.runtime is None

        for result in sol.results:
            assert result.constraints is None
            assert result.feasible is None
            assert result.obj_value is None
            bs = reverser([e for e in result.sample])
            assert result.counts == fake_result["final_bitstring_distribution"][bs]

        results = list(sol.results)
        assert len(results) == num_samples
