from itertools import product

import numpy as np
import scipy.sparse as sp  # type: ignore[import-untyped]
from luna_model import Model, Timer
from luna_model.translator import QuboTranslator
from numpy.typing import NDArray
from rich.console import Console
from rich.table import Table

from tests.test_core.utils import make_seed

REPS = 10


def qubo(config) -> NDArray:
    size, density = config
    np.random.seed(make_seed())
    out = sp.random(size, size, density).todense()
    out += out.T
    return out


def model(config) -> Model:
    return QuboTranslator.to_lm(qubo(config))


def test_large_model_hash():
    table = Table(title="Hash Timings", caption="Default = hash(c=F, v=F)")

    table.add_column("N.Variables", justify="right")
    table.add_column("Density", justify="right")
    table.add_column("Default\n(__hash__)", justify="center")
    # table.add_column("hash\n(c=T, v=T)", justify="center")
    # table.add_column("hash\n(c=F, v=T)", justify="center")
    # table.add_column("hash\n(c=T, v=F)", justify="center")
    # table.add_column("hash\n(c=F, v=F)", justify="center")

    configs = list(product([100, 200, 400, 800, 1000, 1200], [0.1, 0.5, 1.0]))
    for size, density in configs:
        m = model((size, density))
        # warmup
        _ = hash(m)

        t0s = []
        for _ in range(REPS):
            timer = Timer.start()
            hash(m)
            t0 = timer.stop()
            t0s.append(t0.total_seconds)

        t0 = np.mean(t0s)
        # t0std = np.std(t0s)

        # t1s = []
        # for _ in range(REPS):
        #     timer = Timer.start()
        #     m.hash(compress=True, version=True)  # type: ignore
        #     t1 = timer.stop()
        #     t1s.append(t1.total_seconds)
        # t1 = np.mean(t1s)
        # # t1std = np.std(t1s)

        # t2s = []
        # for _ in range(REPS):
        #     timer = Timer.start()
        #     m.hash(compress=False, version=True)  # type: ignore
        #     t2 = timer.stop()
        #     t2s.append(t2.total_seconds)
        # t2 = np.mean(t2s)
        # # t2std = np.std(t2s)

        # t3s = []
        # for _ in range(REPS):
        #     timer = Timer.start()
        #     m.hash(compress=True, version=False)  # type: ignore
        #     t3 = timer.stop()
        #     t3s.append(t3.total_seconds)
        # t3 = np.mean(t3s)
        # # t3std = np.std(t3s)

        # t4s = []
        # for _ in range(REPS):
        #     timer = Timer.start()
        #     m.hash(compress=False, version=False)  # type: ignore
        #     t4 = timer.stop()
        #     t4s.append(t4.total_seconds)
        # t4 = np.mean(t4s)
        # # t4std = np.std(t4s)

        all = [t0]  # , t1, t2, t3, t4]
        all_txt = [
            f"{t0:.7f}",  #  default
            # f"{t1:.7f}",  #  c=T, v=T
            # f"{t2:.7f}",  #  c=F, v=T
            # f"{t3:.7f}",  #  c=T, v=F
            # f"{t4:.7f}",  #  c=F, v=F
        ]

        argmin = np.argmin(all)
        all_txt[argmin] = f"[bold magenta]{all_txt[argmin]}"

        table.add_row(str(size), str(density), *all_txt)

    console = Console()
    console.print(table)
