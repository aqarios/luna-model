import datetime
import os
import platform
import re
import subprocess
import sys
from pathlib import Path
from typing import IO

from benchmarks.serialization.deserialize import (
    bench_deserialize_bqm,
)
from benchmarks.serialization.serialize import (
    bench_serialize_cqm,
    bench_serialize_cqm_size,
)
from benchmarks.serialization.serialize_extralarge import (
    bench_serialize_aqm_xl,
)
from benchmarks.translator.read_dense_qubo import bench_read_dense_qubo
from benchmarks.translator.read_dense_qubo_memory import bench_read_dense_qubo_memory

TASKS = [
    bench_read_dense_qubo,
    bench_serialize_cqm,
    bench_serialize_cqm_size,
    bench_deserialize_bqm,
    bench_serialize_aqm_xl,
    bench_read_dense_qubo_memory,
]


def get_processor_name():
    name = ""
    if platform.system() == "Windows":
        name = platform.processor()
    elif platform.system() == "Darwin":
        os.environ["PATH"] = os.environ["PATH"] + os.pathsep + "/usr/sbin"
        command = ["sysctl", "-n", "machdep.cpu.brand_string"]
        name = (
            str(subprocess.check_output(command))
            .strip()
            .replace("b", "")
            .replace("'", "")
        )
    elif platform.system() == "Linux":
        command = "cat /proc/cpuinfo"
        all_info = subprocess.check_output(command, shell=True).decode().strip()
        for line in all_info.split("\n"):
            if "model name" in line:
                name = re.sub(".*model name.*:", "", line, count=1)
    name = name.strip()
    name = name.replace(r"\n", "")
    return name


def header(dt) -> str:
    title = f"Benchmarks on {dt}"
    processor_name = get_processor_name()

    return f"## {title}\n\nCPU: {processor_name}\n\n"


def code_block(fun, file: IO | None = None):
    if file:
        file.write("```\n")

    fun(file)

    if file:
        file.write("```\n\n")


def add_report_entry(dt):
    report_collection = Path("__file__").parent.parent / "report.md"
    if report_collection.exists():
        with report_collection.open("a") as f:
            f.write(header(dt))
            f.write(
                f"Detailed benchmarks can be found [here](./bench_reports/bench_{dt}.md)\n\n"
            )


def main():
    # report_file_name = sys.argv[1] if len(sys.argv) == 2 else None
    now = datetime.datetime.now().strftime("%Y-%m-%dT%H-%M-%S")
    add_report_entry(now)
    detailed_file = Path(f"./bench_reports/bench_{now}.md")
    detailed_file.touch()

    with detailed_file.open("a") as f:
        for task in TASKS:
            code_block(task, f)


if __name__ == "__main__":
    sys.exit(main())
