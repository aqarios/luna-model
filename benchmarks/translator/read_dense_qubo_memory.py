import os
from collections import defaultdict
from typing import IO, DefaultDict, Dict, Iterable, Optional, Set
from dataclasses import dataclass
from numpy.typing import NDArray
from tqdm import tqdm  # type: ignore[import-untyped]
import memray
import uuid

from rich import print as rprint
from rich.table import Column
from rich.table import Table

from aqmodels import MatrixTranslator, Vtype
import dimod

from benchmarks.setting import DENSITIES, SIZES
from benchmarks.utils import BenchResult, make_qubo


def memit(f):
    def inner(arg) -> Iterable[memray.AllocationRecord]:
        file = "memory_records" + uuid.uuid4().hex + ".bin"
        with memray.Tracker(destination=memray.FileDestination(file, overwrite=True)):
            f(arg)
        alloc_records = list(
            memray.FileReader(file).get_high_watermark_allocation_records()
        )
        os.unlink(file)
        return alloc_records

    return inner


# return decorator


@memit
def _aqm(qubo: NDArray):
    _ = MatrixTranslator.to_model(qubo, vtype=Vtype.Binary)


@memit
def _dmd(qubo: NDArray):
    _ = dimod.BinaryQuadraticModel(qubo, "BINARY")


def bench_read_dense_qubo_memory(file: IO | None):
    result = BenchResult("Read Dense Qubo to Model (Memory Usage)")

    for size in tqdm(SIZES, desc="Num. Variables", leave=False):
        aqm_for_size = []
        dmd_for_size = []
        for density in tqdm(DENSITIES, desc="Density", leave=False):
            qubo = make_qubo(size, density)
            aqm_for_size.append(_aqm(qubo))
            dmd_for_size.append(_dmd(qubo))

        result.aqmodels.append(aqm_for_size)  # type: ignore
        result.dimod.append(dmd_for_size)  # type: ignore

    # result.meta.extend([SIZES, DENSITIES])
    # result.meta_labels.extend(["Size", "Density"])

    format_peek_memory_summary(result, file=file)


# Memray stuff
# Source: https://github.com/bloomberg/memray/blob/main/src/memray/reporters/tui.py#L197

DEFAULT_TERMINAL_LINES = 24


def size_fmt(num, suffix="B", target: str | None = None) -> str:
    if not target:
        for unit in ["", "K", "M", "G", "T", "P", "E", "Z"]:
            if abs(num) < 1024.0:
                return f"{num:5.3f}{unit}{suffix}"
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
    return f"{num:5.3f}{target}{suffix}"


def float_fmt(num) -> str:
    return f"{num:5.3f}"


MAX_MEMORY_RATIO: float = 0.95


KEY_TO_COLUMN_NAME = {
    1: "total_memory",
    2: "total_memory",
    3: "own_memory",
    4: "own_memory",
    5: "n_allocations",
}


def format_peek_memory_summary(result: BenchResult, file: IO | None = None) -> None:
    table = Table(
        Column("Size", ratio=1),
        Column("Density", ratio=1),
        Column("AQM Peek Total Memory", ratio=1, justify="right"),
        Column("Dimod Peek Total Memory", ratio=1, justify="right"),
        Column("Improvement Factor", ratio=1, justify="right"),
        Column("AQM Peek Own Memory", ratio=1, justify="right"),
        Column("Dimod Peek Own Memory", ratio=1, justify="right"),
        Column("Improvement Factor", ratio=1, justify="right"),
        title=result.title,
    )
    if result.caption:
        table.caption = result.caption
        table.caption_justify = "left"

    for size, aqm_res, dmd_res in zip(SIZES, result.aqmodels, result.dimod):
        for density, aqm_data, dmd_data in zip(DENSITIES, aqm_res, dmd_res):
            aqm_peeks = get_peek_memory(aqm_data)
            dmd_peeks = get_peek_memory(dmd_data)

            table.add_row(
                *format_row_entries(size, density),
                *format_mem_row_entries(aqm_peeks, dmd_peeks),
            )

    rprint(table, file=file)


def get_peek_memory(
    data: Iterable[memray.AllocationRecord],
) -> tuple[float, float]:
    snapshot = tuple(data)
    current_memory_size = sum(record.size for record in snapshot)

    snapshot_data = aggregate_allocations(
        snapshot, MAX_MEMORY_RATIO * current_memory_size, True
    )

    peek_total_memory: float = -float("inf")
    peek_own_memory: float = -float("inf")

    for _, result in snapshot_data.items():
        if result.total_memory > peek_total_memory:
            peek_total_memory = result.total_memory
        if result.own_memory > peek_own_memory:
            peek_own_memory = result.own_memory

    return peek_total_memory, peek_own_memory


def format_row_entries(size: int, density: float) -> tuple[str, str]:
    return (
        f"{size:4d}",
        f"{int(density * 100):3d}%",
    )


def format_mem_row_entries(
    aqm_peeks: tuple[float, float], dmd_peeks: tuple[float, float]
) -> tuple[str, str, str, str, str, str]:
    aqm_peek_total, aqm_peek_own = aqm_peeks
    dmd_peek_total, dmd_peek_own = dmd_peeks

    aqm_peek_total_color = "green" if aqm_peek_total < dmd_peek_total else None
    dmd_peek_total_color = "green" if dmd_peek_total < aqm_peek_total else None

    aqm_peek_own_color = "green" if aqm_peek_own < dmd_peek_own else None
    dmd_peek_own_color = "green" if dmd_peek_own < aqm_peek_own else None

    def size_fmt_target(n):
        size_fmt(n, target="M")

    aqm_peek_total_str_content = size_fmt_target(aqm_peek_total)
    aqm_peek_own_str_content = size_fmt_target(aqm_peek_own)
    dmd_peek_total_str_content = size_fmt_target(dmd_peek_total)
    dmd_peek_own_str_content = size_fmt_target(dmd_peek_own)

    improvement_total = dmd_peek_total / aqm_peek_total
    improvement_total_str = float_fmt(improvement_total)
    improvement_total_col = "cyan" if improvement_total >= 1.0 else None
    improvement_own = dmd_peek_own / aqm_peek_own
    improvement_own_str = float_fmt(improvement_own)
    improvement_own_col = "cyan" if improvement_own >= 1.0 else None

    return (
        colorize(aqm_peek_total_str_content, aqm_peek_total_color),
        colorize(dmd_peek_total_str_content, dmd_peek_total_color),
        colorize(improvement_total_str, improvement_total_col),
        colorize(aqm_peek_own_str_content, aqm_peek_own_color),
        colorize(dmd_peek_own_str_content, dmd_peek_own_color),
        colorize(improvement_own_str, improvement_own_col),
    )


def colorize(content, color: str | None = None) -> str:
    if color:
        return f"[{color}]{content}[/{color}]"
    else:
        return content


@dataclass(frozen=True)
class Location:
    function: str
    file: str


@dataclass
class AllocationEntry:
    own_memory: int
    total_memory: int
    n_allocations: int
    thread_ids: Set[int]


def aggregate_allocations(
    allocations: Iterable[memray.AllocationRecord],
    memory_threshold: float = float("inf"),
    native_traces: Optional[bool] = False,
) -> Dict[Location, AllocationEntry]:
    """Take allocation records and for each frame contained, record "own"
    allocations which happened on the frame, and sum up allocations on
    all of the child frames to calculate "total" allocations."""

    processed_allocations: DefaultDict[Location, AllocationEntry] = defaultdict(
        lambda: AllocationEntry(
            own_memory=0, total_memory=0, n_allocations=0, thread_ids=set()
        )
    )

    current_total = 0
    for allocation in allocations:
        if current_total >= memory_threshold:
            break
        current_total += allocation.size

        stack_trace = list(
            allocation.hybrid_stack_trace()
            if native_traces
            else allocation.stack_trace()
        )
        if not stack_trace:
            frame = processed_allocations[Location(function="???", file="???")]
            frame.total_memory += allocation.size
            frame.own_memory += allocation.size
            frame.n_allocations += allocation.n_allocations
            frame.thread_ids.add(allocation.tid)
            continue

        # Walk upwards and sum totals
        visited = set()
        for i, (function, file_name, _) in enumerate(stack_trace):
            location = Location(function=function, file=file_name)
            frame = processed_allocations[location]
            if location in visited:
                continue
            visited.add(location)
            if i == 0:
                frame.own_memory += allocation.size
            frame.total_memory += allocation.size
            frame.n_allocations += allocation.n_allocations
            frame.thread_ids.add(allocation.tid)
    return processed_allocations
