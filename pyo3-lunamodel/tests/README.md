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

Uses an editable install of `../../py-lunamodel`:

```bash
uv run --group local pytest integration_ext/pytests
```

## Run tests against the published `lunamodel` on PyPI

```bash
uv run --group pypi pytest integration_ext/pytests
```

The minimum version is pinned in `pyproject.toml` under
`[dependency-groups] pypi`. Bump it there when a new release should be
exercised.

## Pinning the Python version

```bash
uv run -p 3.14 --group local pytest integration_ext/pytests
```

## Forcing a rebuild of the extension

After Rust changes in `pyo3-lunamodel/` or `integration_ext/`, uv may reuse
the cached wheel. Force a rebuild with:

```bash
uv sync --group local --reinstall-package pyo3-lunamodel-integration-ext
```

(Swap `--group pypi` as needed.)

## How it's wired

- `[tool.uv] package = false` — this is a uv workspace env, not a
  publishable distribution.
- `conflicts = [[{group = "local"}, {group = "pypi"}]]` — uv refuses to
  resolve both groups together, keeping the lockfile coherent.
- `[tool.uv.sources]` maps `lunamodel` to the local path **only** under
  `--group local`; otherwise it resolves from PyPI.
- `pyo3-lunamodel-integration-ext` is a path source built via PEP 517
  (maturin) on `uv sync` / `uv run`.
