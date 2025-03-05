from pathlib import Path
import sys
from typing import IO
from benchmarks.translator.read_dense_qubo import bench_read_dense_qubo
from benchmarks.serialization.serialize import (
    bench_serialize_bqm,
    bench_serialize_bqm_size,
)
from benchmarks.serialization.deserialize import (
    bench_deserialize_bqm,
)
from benchmarks.serialization.serialize_extralarge import (
    bench_serialize_aqm_xl,
)

from benchmarks.translator.read_dense_qubo_memory import bench_read_dense_qubo_memory

report_file_name = sys.argv[1] if len(sys.argv) == 2 else None

report_file_write: IO | None = None
report_file_append: IO | None = None
if report_file_name:
    report_file = Path(report_file_name)
    report_file.touch()
    report_file_write = report_file.open("w")
    report_file_append = report_file.open("a")


def header():
    if report_file_write:
        report_file_write.write("```")


def footer(n: int = 1):
    if report_file_append:
        report_file_append.write("```")
        rem: int = n - 1
        for _ in range(rem):
            report_file_append.write("\n")


header()
bench_read_dense_qubo(report_file_append)
footer(2)
bench_serialize_bqm(report_file_append)
footer(2)
bench_deserialize_bqm(report_file_append)
footer(2)
bench_serialize_bqm_size(report_file_append)
footer(2)
bench_serialize_aqm_xl(report_file_append)
footer(2)
bench_read_dense_qubo_memory(report_file_append)
footer()
