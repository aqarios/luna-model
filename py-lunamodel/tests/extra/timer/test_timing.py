from luna_model.timer import Timing


def test_default_total_is_zero():
    timing = Timing()
    assert timing.total == 0.0


def test_construct_with_total():
    timing = Timing(4.5)
    assert timing.total == 4.5


def test_get_missing_key_returns_empty_list():
    timing = Timing()
    assert timing.get("missing") == []


def test_setitem_then_get_returns_single_value():
    timing = Timing()
    timing["a"] = 2.0
    assert timing.get("a") == [2.0]


def test_setitem_replaces_previous_values():
    timing = Timing()
    timing["a"] = 1.0
    timing["a"] = 5.0
    assert timing.get("a") == [5.0]


def test_getitem_missing_key_returns_zero():
    timing = Timing()
    assert timing["missing"] == 0.0


def test_getitem_returns_total_for_key():
    timing = Timing()
    timing["a"] = 1.0
    assert timing["a"] == 1.0


def test_getitem_matches_total_for():
    timing = Timing()
    timing["a"] = 1.0
    assert timing["a"] == timing.total_for("a")


def test_subscript_augmented_assignment_accumulates_running_total():
    """`timing[key] += value` reads the current total via `__getitem__`
    (0.0 if unset) and replaces `key`'s recorded values with that single
    new total via `__setitem__` — it does not keep a history of every
    individual addend, unlike `Timer.record()`'s sub-timers."""
    timing = Timing()
    timing["a"] += 1.0
    assert timing["a"] == 1.0
    assert timing.get("a") == [1.0]

    timing["a"] += 2.0
    assert timing["a"] == 3.0
    assert timing.get("a") == [3.0]


def test_total_for_missing_key_is_none():
    timing = Timing()
    assert timing.total_for("missing") is None


def test_total_for_sums_recorded_values():
    timing = Timing()
    timing["a"] += 1.0
    timing["a"] += 2.0
    assert timing.total_for("a") == 3.0


def test_eq_compares_total_and_entries():
    a = Timing(1.0)
    b = Timing(1.0)
    assert a == b

    a["x"] = 1.0
    assert a != b


def test_eq_against_other_type_is_not_equal():
    timing = Timing()
    assert timing != object()


def test_str_and_repr_do_not_raise():
    timing = Timing(1.0)
    timing["a"] = 2.0
    assert isinstance(str(timing), str)
    assert isinstance(repr(timing), str)

def test_timing_to_str():
    timing = Timing(1.0)
    timing.qpu = 2.2
    timing.preprocess = 3.3
    timing.postprocess = 4.4213
    timing_str = str(timing)
    assert "Timing(total=1.0, qpu=2.2, preprocess=3.3, postprocess=4.4213)" == timing_str
