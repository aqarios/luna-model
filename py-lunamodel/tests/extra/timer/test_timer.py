import threading

import pytest

from luna_model.timer import SubTimer, Timer, Timing

def test_init_returns_timer():
    t = Timer()
    assert isinstance(t, Timer)


def test_start_returns_timer():
    t = Timer.start()
    assert isinstance(t, Timer)


def test_stop_returns_timing():
    t = Timer.start()
    timing = t.stop()
    assert isinstance(timing, Timing)
    assert timing.total >= 0.0


def test_record_returns_sub_timer():
    t = Timer.start()
    sub = t.record("preprocessing")
    assert isinstance(sub, SubTimer)
    sub.stop()
    timing = t.stop()
    assert timing.preprocessing is not None


def test_sub_timer_records_under_its_key():
    t = Timer.start()

    preprocessing = t.record("preprocessing")
    preprocessing.stop()

    timing = t.stop()
    recorded = timing.get("preprocessing")
    assert len(recorded) == 1
    assert recorded[0] >= 0.0


def test_sub_timer_stop_returns_elapsed_seconds():
    t = Timer.start()
    sub = t.record("qpu")
    elapsed = sub.stop()
    assert isinstance(elapsed, float)
    assert elapsed >= 0.0


def test_sub_timer_stop_twice_raises():
    t = Timer.start()
    sub = t.record("qpu")
    sub.stop()

    with pytest.raises(Exception):  # noqa: B017,PT011
        sub.stop()


def test_repeated_sub_timers_accumulate_under_same_key():
    t = Timer.start()

    for _ in range(3):
        sub = t.record("qpu")
        sub.stop()

    timing = t.stop()
    assert len(timing.get("qpu")) == 3


def test_multiple_named_sub_timers_are_independent():
    t = Timer.start()

    preprocessing = t.record("preprocessing")
    preprocessing.stop()

    qpu = t.record("qpu")
    qpu.stop()

    timing = t.stop()
    assert len(timing.get("preprocessing")) == 1
    assert len(timing.get("qpu")) == 1


def test_sub_timers_started_concurrently_all_record():
    t = Timer.start()

    def run():
        sub = t.record("parallel")
        sub.stop()

    threads = [threading.Thread(target=run) for _ in range(8)]
    for thread in threads:
        thread.start()
    for thread in threads:
        thread.join()

    timing = t.stop()
    assert len(timing.get("parallel")) == 8
