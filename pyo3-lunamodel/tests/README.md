# Testing pyo3-lunamodel

Tests live in this directory: a small maturin-built extension crate
(`pyo3-lunamodel-integration-ext` under `integration_ext/`) plus pytest
suites in `integration_ext/pytests/`. They exercise the pyo3 bindings
against the `lunamodel` Python package.

The test environment is managed by [`uv`](https://docs.astral.sh/uv/) via
the `pyproject.toml` in this directory. There is no `nox` / `tox` — the
local-vs-published `lunamodel` switch is expressed as two mutually exclusive
uv dependency groups.

## Prerequisites

- `uv` (>= 0.5)
- A Rust toolchain (maturin builds the extension on first sync)

Run all commands from `pyo3-lunamodel/tests/`.

## Run tests against the local `py-lunamodel`

This is the default (`default-groups = ["local"]`). It builds and installs
`../../py-lunamodel`:

```bash
uv run pytest integration_ext/pytests
```

## Run tests against the published `lunamodel` on PyPI

```bash
uv run --no-default-groups --group pypi pytest integration_ext/pytests
```

`--no-default-groups` is required: `local` is enabled by default and conflicts
with `pypi`.

The extension itself is always compiled from the local Rust crates
(`integration_ext/Cargo.toml` uses path dependencies); the group only selects
which `luna_model` host it runs against. That is what makes this a check of
whether the published host is compatible with the current extension code.

The minimum version is pinned in `pyproject.toml` under
`[dependency-groups] pypi`. Bump it there when a new release should be
exercised.

## Pinning the Python version

```bash
uv run -p 3.14 pytest integration_ext/pytests
```

## Forcing a rebuild of the extension

After Rust changes in `pyo3-lunamodel/` or `integration_ext/`, uv may reuse
the cached wheel. Force a rebuild with:

```bash
uv sync --reinstall-package pyo3-lunamodel-integration-ext
```

`py-lunamodel` is installed non-editable, so after changes there add
`--reinstall-package luna-model` as well. (Use `--no-default-groups --group pypi`
for the PyPI variant.)

## How it's wired

- `[tool.uv] package = false` — this is a uv workspace env, not a
  publishable distribution.
- `conflicts = [[{group = "local"}, {group = "pypi"}]]` — uv refuses to
  resolve both groups together, keeping the lockfile coherent.
- `default-groups = ["local"]` makes the local host the default.
- `[tool.uv.sources]` maps `luna-model` to the local path **only** under
  the `local` group; otherwise it resolves from PyPI. `integration_ext` must
  not declare its own source for `luna-model`, or it would override the group
  selection.
- `pytests/test_layout.py` reads and computes on host objects in Rust. The
  `roundtrip_*` tests only pass the `Arc` back and cannot detect layout
  mismatches.
- `pyo3-lunamodel-integration-ext` is a path source built via PEP 517
  (maturin) on `uv sync` / `uv run`.
