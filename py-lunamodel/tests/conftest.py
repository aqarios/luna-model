"""Session-wide random seed shared between the pytest-xdist controller and its workers.

Several test modules (see `test_core/utils.py::make_seed`) generate fuzz-style random test
data — including data embedded directly in parametrize IDs — using an unseeded random pick
per call. Under plain serial execution that's merely "different every run"; under
`pytest-xdist`, each worker is a separate process that collects the test suite independently,
so an unseeded pick makes every worker compute different parametrize IDs for "the same" test,
and xdist aborts with "Different tests were collected between gw0 and gw1".

The fix: pick exactly one seed for the whole run, in the controller (or the single process, if
not running under xdist at all), and hand it down to every worker via xdist's documented
`pytest_configure_node` hook before that worker starts collecting. Every process then exposes it
as `LUNA_TEST_SEED` in its own environment, so `make_seed()` can read it with no dependency on
pytest internals. All processes then derive the same *sequence* of per-call seeds from that one
shared value (see `test_core/utils.py`), since collection executes the same source in the same
order everywhere — fixing the xdist crash and, as a side effect, making a failing run actually
reproducible: rerun with `LUNA_TEST_SEED=<printed seed>` (serial or parallel) to get identical
data.
"""

import os
import random
import sys


def pytest_configure(config):
    if hasattr(config, "workerinput"):
        # Running inside an xdist worker: the controller already picked the seed and passed it
        # down via pytest_configure_node below.
        seed = config.workerinput["luna_test_seed"]
    else:
        # The controller process, or a plain serial run with no xdist involved at all.
        override = os.environ.get("LUNA_TEST_SEED")
        seed = int(override) if override is not None else random.randint(0, 2**32 - 1)
        print(
            f"\nLuna test data seed = {seed} "
            f"(rerun with LUNA_TEST_SEED={seed} to reproduce this run's generated data)\n",
            file=sys.stderr,
        )

    config.luna_test_seed = seed
    # Exposed as a plain env var so make_seed() doesn't need to depend on pytest's Config object.
    os.environ["LUNA_TEST_SEED"] = str(seed)


def pytest_configure_node(node):
    """xdist controller hook: runs once per worker, before it starts up."""
    node.workerinput["luna_test_seed"] = node.config.luna_test_seed
