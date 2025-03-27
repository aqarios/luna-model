import re
from pathlib import Path
from collections import defaultdict
from gen_common import LIB_ROOT, PROJECT_ROOT, AUTOGEN_HEADER, collect_exports


def extract_class_block(text: str, name: str) -> str:
    lines = text.splitlines()
    class_lines = []
    inside = False

    for i, line in enumerate(lines):
        if re.match(rf"\s*(?:@.*)?\s*class\s+{re.escape(name)}\b", line):
            inside = True
        if inside:
            class_lines.append(line)
            if i + 1 < len(lines):
                next_line = lines[i + 1]
                if next_line.strip() and not next_line.startswith((" ", "\t", "@")):
                    break
    if not class_lines:
        raise ValueError(f"Class {name} not found")
    return "\n".join(class_lines).rstrip() + "\n"


def extract_imports(text: str, defined_symbols: set) -> list[str]:
    imports = []
    for line in text.splitlines():
        if line.strip().startswith("import ") or line.strip().startswith("from "):
            match = re.match(r"from\s+\S+\s+import\s+(.+)", line.strip())
            if match:
                imported = [
                    part.strip().split(" ")[0] for part in match.group(1).split(",")
                ]
                if not any(symbol in defined_symbols for symbol in imported):
                    imports.append(line.strip())
            else:
                imports.append(line.strip())
        elif line.strip() and not line.strip().startswith("#"):
            break  # Stop at first non-import, non-comment line
    return imports


def generate_stub_file(target_path: Path, symbols: list[dict]):
    lines = [AUTOGEN_HEADER]
    import_set = set()
    symbol_blocks = []
    exported_names = set()
    defined_symbols = {sym["class_name"] for sym in symbols}
    public_submodules = set()

    for sym in symbols:
        path = LIB_ROOT / f"{sym['module_path']}.pyi"
        text = path.read_text(encoding="utf-8")
        class_block = extract_class_block(text, sym["class_name"])
        symbol_blocks.append(class_block)
        exported_names.add(sym["class_name"])
        import_set.update(extract_imports(text, defined_symbols))

        if Path(LIB_ROOT / sym["module_path"].parts[0]).is_dir() and not sym[
            "module_path"
        ].parts[0].startswith("_"):
            public_submodules.add(sym["module_path"].parts[0])

    if import_set:
        lines.extend(sorted(import_set))
        lines.append("")

    for mod in sorted(public_submodules):
        lines.append(f"from . import {mod}")
    if public_submodules:
        lines.append("")

    lines.extend(symbol_blocks)
    lines.append("")
    lines.append("__all__ = [")
    for name in sorted(exported_names.union(public_submodules)):
        lines.append(f'    "{name}",')
    lines.append("]")

    target_path.parent.mkdir(parents=True, exist_ok=True)
    target_path.write_text("\n".join(lines), encoding="utf-8")
    print(f"✅ Wrote stub: {target_path.relative_to(PROJECT_ROOT)}")


def main():
    exports = collect_exports(LIB_ROOT)
    top_level_exports = [e for e in exports if "__init__" in e["targets"]]
    generate_stub_file(LIB_ROOT / "__init__.pyi", top_level_exports)
    generate_stub_file(LIB_ROOT / "_core.pyi", top_level_exports)

    sub_exports = defaultdict(list)
    for e in exports:
        for t in e["targets"]:
            if t != "__init__":
                sub_exports[e["module_path"].parts[0]].append(e)

    for mod, entries in sub_exports.items():
        stub_path = LIB_ROOT / mod / "__init__.pyi"
        generate_stub_file(stub_path, entries)


if __name__ == "__main__":
    main()
