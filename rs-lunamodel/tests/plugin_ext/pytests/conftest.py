"""Generates the test's protobuf message (typed) from its `.proto` before the tests import it.

Runs at conftest import — i.e. before test modules are collected — via `grpcio-tools` (which bundles
`protoc`, so no system install is needed). Produces `variable_count_pb2.py` + `variable_count_pb2.pyi`
(the `--pyi_out` stubs give static typing), mirroring how a real plugin author generates a typed
message class from the shared `.proto`.
"""

import subprocess
import sys
from pathlib import Path

_HERE = Path(__file__).parent
_PROTO_DIR = _HERE / "protos"
_GEN_DIR = _HERE / "_generated"


def _generate_protos() -> None:
    _GEN_DIR.mkdir(exist_ok=True)
    subprocess.run(
        [
            sys.executable,
            "-m",
            "grpc_tools.protoc",
            f"-I{_PROTO_DIR}",
            f"--python_out={_GEN_DIR}",
            f"--pyi_out={_GEN_DIR}",
            str(_PROTO_DIR / "variable_count.proto"),
        ],
        check=True,
    )


_generate_protos()
sys.path.insert(0, str(_GEN_DIR))
