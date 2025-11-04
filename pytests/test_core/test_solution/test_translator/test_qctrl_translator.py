from random import Random, shuffle
from time import sleep
from typing import Any, Callable

import pytest
from luna_model import Environment, Timer, Variable
from luna_model.translator import QctrlTranslator
from numpy import unique

from pytests.test_core.utils import make_seed, random, random_bool, random_int

REPS: int = 100
MAX_VARS: int = 200


def fake_qctrl_result(
    rand: Random, length: int, cost: float, num_samples: int
) -> tuple[dict[str, Any], Callable[[list], str]]:
    def random_bitstring():
        return "".join([str(int(random_bool(rand))) for _ in range(length)])

    assignment = [length - 1 - i for i in range(length)]
    shuffle(assignment)
    map = {f"n[{i}]": a for i, a in enumerate(assignment)}
    # key position in sample actual, a position in solution false
    forward_assignment = {i: a for i, a in enumerate(assignment)}
    reverse_assignment = {a: i for i, a in enumerate(assignment)}

    def adjust_ordering(actual: list | str) -> str:
        # need to move each value to the A value of the mapping.
        # the actual is the correct bitstring in the expected order.
        out = "".join([str(actual[forward_assignment[i]]) for i in range(len(actual))])
        return out

    def reverse_adjust(other: list) -> str:
        out = "".join([str(other[reverse_assignment[i]]) for i in range(len(other))])
        return out

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
        "variables_to_bitstring_index_map": map,
    }, reverse_adjust


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


@pytest.mark.solution_translation()
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
            sol = QctrlTranslator.to_aq(fake_result)

        samples = sol.samples.tolist()
        assert len(samples) == num_samples, "number of samples does not match"
        assert len(samples[0]) == sample_len, (
            "sample len (num variables) does not match"
        )
        assert sol.raw_energies == None
        assert len(sol.counts.tolist()) == num_samples
        assert sol.runtime is None

        for result in sol.results:
            assert result.constraints is None
            assert result.feasible is None

        results = list(sol.results)
        assert len(results) == num_samples


@pytest.mark.solution_translation()
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

        sol = QctrlTranslator.to_aq(fake_result, env=env)

        samples = sol.samples.tolist()
        assert len(samples) == num_samples, "number of samples does not match"
        assert len(samples[0]) == sample_len, (
            "sample len (num variables) does not match"
        )
        assert sol.raw_energies == None
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


@pytest.mark.solution_translation()
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
            sol = QctrlTranslator.to_aq(fake_result, timing)

        samples = sol.samples.tolist()
        assert len(samples) == num_samples, "number of samples does not match"
        assert len(samples[0]) == sample_len, (
            "sample len (num variables) does not match"
        )
        assert sol.raw_energies == None
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


@pytest.mark.solution_translation()
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
            sol = QctrlTranslator.to_aq(fake_result)

        samples = sol.samples.tolist()
        assert len(samples) == num_samples, "number of samples does not match"
        assert len(samples[0]) == sample_len, (
            "sample len (num variables) does not match"
        )
        assert sol.raw_energies == None
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
