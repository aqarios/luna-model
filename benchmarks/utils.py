import time
from dataclasses import dataclass, field
from typing import Any, Callable

import numpy as np
import scipy.sparse as sp  # type: ignore[import-untyped]
from numpy.typing import NDArray
from tabulate import tabulate  # type: ignore[import-untyped]
from colorama import Fore, Back, Style  # type: ignore[import-untyped]

from benchmarks.setting import DENSITIES, SIZES


@dataclass
class BenchResult:
    title: str

    aqmodels: list = field(default_factory=list)  # type: ignore
    dimod: list = field(default_factory=list)  # type: ignore
    meta: list = field(default_factory=list)
    meta_labels: list = field(default_factory=list)

    others: list[list] | None = None  # type: ignore
    others_labels: list[str] | None = None  # type: ignore

    dimod_alt: str | None = None  # type: ignore
    aqm_alt: str | None = None  # type: ignore

    # metadata: dict[str, Any] = field(default_factory=dict)


def timeit(repeats: int):
    def decorator(f) -> Callable[[Any], float]:
        def inner(arg) -> float:
            timings = []
            for _ in range(repeats):
                start = time.perf_counter()
                f(arg)
                end = time.perf_counter()
                timings.append(end - start)
            return float(np.mean(timings))

        return inner

    return decorator


def make_qubo(size: int, density: float) -> NDArray:
    return sp.random(size, size, density).todense().A


def format_result(result: BenchResult):

    data_flat: list[list] = [[] for _ in range((len(SIZES) * len(DENSITIES)))]
    headers: list[str] = ["Size", "Density"]

    aqm_header = result.aqm_alt if result.aqm_alt else "AQM (in sec)"
    headers.append(f"{Fore.CYAN}{aqm_header}{Style.RESET_ALL}")
    for si, ds in enumerate(result.aqmodels):
        for di, dd in enumerate(ds):
            i = (si * len(ds)) + di
            data_flat[i].append(dd)

    headers.append(result.dimod_alt if result.dimod_alt else "Dimod (in sec)")
    for si, ds in enumerate(result.dimod):
        for di, dd in enumerate(ds):
            i = (si * len(ds)) + di
            data_flat[i].append(dd)

    if result.others is not None:
        assert result.others_labels is not None
        for other, label in zip(result.others, result.others_labels):
            headers.append(label)
            for si, ds in enumerate(other):
                for di, dd in enumerate(ds):
                    i = (si * len(ds)) + di
                    data_flat[i].append(dd)

    # Also add the speedup if only AQM vs Dimod and no others.
    if result.others is None and len(result.dimod) > 0:
        for row in data_flat:
            aqm_val = row[0]
            dmd_val = row[1]
            factor = dmd_val / aqm_val
            row.append(factor)
        headers.append("AQM is ... times better than Dimod")

    # Search minimum per row and highlight
    for row in data_flat:
        # min_val = np.array(row).min()
        # minima_indices = np.where(np.isclose(row, min_val))
        if result.others is None and len(result.dimod) > 0:
            argmin = np.argmin(row[:-1])
        else:
            argmin = np.argmin(row)

        for i, item in enumerate(row):
            if i == argmin:
                # do color
                row[i] = f"{Fore.GREEN}{format_float(row[i])}{Style.RESET_ALL}"
            else:
                row[i] = format_float(item)

    for i, size in enumerate(SIZES):
        for j, density in enumerate(DENSITIES):
            idx = (i * len(DENSITIES)) + j
            data_flat[idx].insert(0, format_density(density))
            data_flat[idx].insert(0, format_size(size))

    table = tabulate(
        data_flat,
        headers=headers,
        tablefmt="github",
        colalign=tuple(["right"] * len(headers)),
    )

    # terminal_width = os.get_terminal_size().columns
    width = len(table.split("\n")[0])
    title_length = len(result.title)
    lhs_length = (width - title_length) // 2
    rhs_length = width - title_length - lhs_length

    limiter = "#" * width
    lhs = "###" + (lhs_length - 3) * " "
    rhs = " " * (rhs_length - 3) + "###"
    print(limiter)
    print(f"{lhs}{result.title}{rhs}")
    print(limiter)

    print(table)


def format_float(rt: float) -> str:
    return f"{rt:7.4f}"


def format_density(d: float) -> str:
    return f"{int(d*100):3d}%"


def format_size(s: float) -> str:
    return f"{s:4d}"


def format_result_aqm(result: BenchResult, sizes, densities):

    data_flat: list[list] = [[] for _ in range((len(sizes) * len(densities)))]
    headers: list[str] = ["Size", "Density"]

    aqm_header = result.aqm_alt if result.aqm_alt else "AQM (in sec)"
    headers.append(f"{Fore.CYAN}{aqm_header}{Style.RESET_ALL}")
    for si, ds in enumerate(result.aqmodels):
        for di, dd in enumerate(ds):
            i = (si * len(ds)) + di
            data_flat[i].append(format_float(dd))

    for i, size in enumerate(sizes):
        for j, density in enumerate(densities):
            idx = (i * len(densities)) + j
            data_flat[idx].insert(0, format_density(density))
            data_flat[idx].insert(0, format_size(size))

    table = tabulate(
        data_flat,
        headers=headers,
        tablefmt="github",
        colalign=tuple(["right"] * len(headers)),
    )

    # terminal_width = os.get_terminal_size().columns
    width = len(table.split("\n")[0])
    title_length = len(result.title)
    lhs_length = (width - title_length) // 2
    rhs_length = width - title_length - lhs_length

    limiter = "#" * width
    lhs = "###" + (lhs_length - 3) * " "
    rhs = " " * (rhs_length - 3) + "###"
    print(limiter)
    print(f"{lhs}{result.title}{rhs}")
    print(limiter)

    print(table)
