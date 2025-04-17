import sys
import ast
import re
import subprocess
from pathlib import Path

# Shared configuration and helpers for all generators

LIB_NAME = "aqmodels"
PROJECT_ROOT = Path(__file__).resolve().parent.parent
LIB_ROOT = None

if LIB_ROOT is None:
    for base in (PROJECT_ROOT / "src", PROJECT_ROOT):
        candidate = base / LIB_NAME
        if candidate.exists():
            LIB_ROOT = candidate
            break
    else:
        raise RuntimeError(f"Could not find library root for {LIB_NAME}")

AUTOGEN_HEADER = """# This file is auto-generated.
# Do not edit manually.
"""

EXPORT_RE = re.compile(r"#\s*@export(.*)")
EXPORT_OVERRIDE_RE = re.compile(r"#\s*@export\_override(.*)")
NORMALIZED_TARGETS = {
    "top": "__init__",
    "root": "__init__",
    "main": "__init__",
}


def collect_exports(lib_root: Path) -> list[dict]:
    exports = []
    for path in lib_root.rglob("_*.py"):
        exports += parse_exports(path, lib_root)
    return exports


def collect_override_exports(lib_root: Path) -> list[dict]:
    exports = []
    for path in lib_root.rglob("_*.py"):
        exports += parse_exports_override(path, lib_root)
    return exports


def parse_exports(py_path: Path, lib_root: Path) -> list[dict]:
    exports = []
    text = py_path.read_text(encoding="utf-8")
    tree = ast.parse(text, filename=str(py_path))
    lines = text.splitlines()

    for node in tree.body:
        if isinstance(node, ast.ClassDef):
            lineno = node.lineno - 1
            preceding = lines[lineno - 1].strip() if lineno > 0 else ""
            match = EXPORT_RE.search(preceding)
            targets = []

            if match:
                raw_target = match.group(1).strip()
                raw_targets = (
                    [t.strip() for t in raw_target.split(",") if t.strip()]
                    if raw_target
                    else ["__init__"]
                )
                targets = [NORMALIZED_TARGETS.get(t, t) for t in raw_targets]

            for deco in node.decorator_list:
                if isinstance(deco, ast.Name) and deco.id == "export":
                    targets.append("__init__")
                elif (
                    isinstance(deco, ast.Call)
                    and getattr(deco.func, "id", None) == "export"
                ):
                    for arg in deco.args:
                        if isinstance(arg, ast.Constant) and isinstance(arg.value, str):
                            val = arg.value.strip()
                            targets.append(NORMALIZED_TARGETS.get(val, val))

            if targets:
                module_path = py_path.relative_to(lib_root).with_suffix("")
                public_module = module_path.parts[0].removeprefix("_")
                exports.append(
                    {
                        "class_name": node.name,
                        "module_path": module_path,
                        "targets": sorted(set(targets)),
                        "public_module": public_module,
                    }
                )
    return exports


def parse_exports_override(py_path: Path, lib_root: Path) -> list[dict]:
    exports = []
    text = py_path.read_text(encoding="utf-8")
    tree = ast.parse(text, filename=str(py_path))
    lines = text.splitlines()

    for node in tree.body:
        if isinstance(node, ast.ClassDef):
            lineno = node.lineno - 1
            preceding = lines[lineno - 1].strip() if lineno > 0 else ""
            match = EXPORT_OVERRIDE_RE.search(preceding)
            targets = []

            if match:
                raw_target = match.group(1).strip()
                raw_targets = (
                    [t.strip() for t in raw_target.split(",") if t.strip()]
                    if raw_target
                    else ["__init__"]
                )
                targets = [NORMALIZED_TARGETS.get(t, t) for t in raw_targets]

            for deco in node.decorator_list:
                if isinstance(deco, ast.Name) and deco.id == "export_override":
                    targets.append("__init__")
                elif (
                    isinstance(deco, ast.Call)
                    and getattr(deco.func, "id", None) == "export_override"
                ):
                    for arg in deco.args:
                        if isinstance(arg, ast.Constant) and isinstance(arg.value, str):
                            val = arg.value.strip()
                            targets.append(NORMALIZED_TARGETS.get(val, val))

            if targets:
                module_path = py_path.relative_to(lib_root).with_suffix("")
                public_module = module_path.parts[0].removeprefix("_")
                exports.append(
                    {
                        "class_name": node.name,
                        "module_path": module_path,
                        "targets": sorted(set(targets)),
                        "public_module": public_module,
                    }
                )
    return exports


def format():
    try:
        subprocess.run(["ruff", "format"])
    except:  # noqa: E722
        print("ruff not found, can not format.", file=sys.stderr)
