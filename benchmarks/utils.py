import time
from dataclasses import dataclass, field
from typing import IO, Any, Callable

import numpy as np
import scipy.sparse as sp  # type: ignore[import-untyped]
from numpy.typing import NDArray
from rich import print as rprint
from rich.table import Column, Table

from benchmarks.setting import DENSITIES, SIZES


@dataclass
class BenchResult:
    title: str
    caption: str | None = None
    suffix: str = "seconds"

    aqmodels: list = field(default_factory=list)  # type: ignore
    dimod: list = field(default_factory=list)  # type: ignore
    # meta: list = field(default_factory=list)
    # meta_labels: list = field(default_factory=list)

    # others: list[list] | None = None  # type: ignore
    # others_labels: list[str] | None = None  # type: ignore

    # dimod_alt: str | None = None  # type: ignore
    # aqm_alt: str | None = None  # type: ignore

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


def format_row_entries(size: int, density: float) -> tuple[str, str]:
    return (
        f"{size:4d}",
        f"{int(density * 100):3d}%",
    )


def float_fmt(num) -> str:
    return f"{num:5.4f}"


def size_fmt(num, suffix="B", target: str | None = None) -> str:
    if not target:
        for unit in ["", "K", "M", "G", "T", "P", "E", "Z"]:
            if abs(num) < 1024.0:
                return f"{num:5.4f}{unit}{suffix}"
            num /= 1024.0
        return f"{num:.1f}Y{suffix}"

    # reduce to MB in all cases.
    mapper = {
        "B": 0,
        "K": 1,
        "M": 2,
        "G": 3,
        "T": 4,
        "P": 5,
        "E": 6,
        "Z": 7,
    }
    num /= 1024 ** mapper[target]
    return f"{num:5.4f}"  # {target}{suffix}"


def colorize(content, color: str | None = None) -> str:
    if color:
        return f"[{color}]{content}[/{color}]"
    else:
        return content


def format_result(result: BenchResult, file: IO | None = None, comp: str = "le"):
    table = Table(
        Column("Size", ratio=1, justify="right"),
        Column("Density", ratio=1, justify="right"),
        Column("AQM", ratio=1, justify="right"),
        Column("Dimod", ratio=1, justify="right"),
        Column("Improvement Factor", ratio=1, justify="right"),
        title=f"{result.title} (in {result.suffix})",
        highlight=True,
    )
    if result.caption:
        table.caption = result.caption
        table.caption_justify = "left"

    for size, aqm_res, dmd_res in zip(SIZES, result.aqmodels, result.dimod):
        for density, aqm_value, dmd_value in zip(DENSITIES, aqm_res, dmd_res):
            aqm_value_col: str | None
            dmd_value_col: str | None
            if comp == "le":
                aqm_value_col = "green" if aqm_value < dmd_value else None
                dmd_value_col = "green" if dmd_value < aqm_value else None
            elif comp == "ge":
                aqm_value_col = "green" if aqm_value > dmd_value else None
                dmd_value_col = "green" if dmd_value > aqm_value else None
            else:
                raise RuntimeError("unkown comparator")

            improvement = dmd_value / aqm_value
            improvement_str = float_fmt(improvement)
            improvement_col = "cyan" if improvement >= 1.0 else "red"

            if result.suffix == "MB":
                aqm_value_str = size_fmt(aqm_value, target="M")
                dmd_value_str = size_fmt(dmd_value, target="M")
            else:
                aqm_value_str = float_fmt(aqm_value)
                dmd_value_str = float_fmt(dmd_value)

            table.add_row(
                *format_row_entries(size, density),
                colorize(aqm_value_str, aqm_value_col),
                colorize(dmd_value_str, dmd_value_col),
                colorize(improvement_str, improvement_col),
            )

    if file:
        rprint(table, file=file)
    else:
        rprint(table)


def format_float(rt: float) -> str:
    return f"{rt:7.4f}"


def format_density(d: float) -> str:
    return f"{int(d * 100):3d}%"


def format_size(s: float) -> str:
    return f"{s:4d}"


def format_result_aqm(result: BenchResult, sizes, densities, file: IO | None):
    table = Table(
        Column("Size", ratio=1, justify="right"),
        Column("Density", ratio=1, justify="right"),
        Column("AQM", ratio=1, justify="right"),
        title=f"{result.title} (in {result.suffix})",
    )
    if result.caption:
        table.caption = result.caption
        table.caption_justify = "left"

    for size, aqm_res in zip(sizes, result.aqmodels):
        for density, aqm_value in zip(densities, aqm_res):
            if result.suffix == "MB":
                aqm_value_str = size_fmt(aqm_value, target="M")
            else:
                aqm_value_str = float_fmt(aqm_value)

            table.add_row(
                *format_row_entries(size, density),
                colorize(aqm_value_str, None),
            )

    if file:
        rprint(table, file=file)
    else:
        rprint(table)
