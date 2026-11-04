# Contributing to LunaModel

This document describes how issues, discussions, and pull requests are handled, and how to set
up a development environment.

## Issues and discussions

Issues track accepted work that is ready to be picked up. Maintainers open and edit them.

Bug reports, feature requests, and questions start in
[GitHub Discussions](https://github.com/aqarios/luna-model/discussions):

1. A discussion describes the problem or the proposal.
2. A maintainer triages it: confirms the behavior, asks for missing details, or clarifies the
   scope.
3. Confirmed, actionable items become issues and are open for pull requests.

Many reports that look like bugs are configuration issues, environment differences, or an API
used in an unsupported way. Triaging them in discussions keeps the issue tracker limited to
work that needs code changes.

- A **bug** is existing behavior that is broken or contradicts the documentation.
- A **feature** is behavior that does not exist yet. Its scope and fit are agreed in the
  discussion before it becomes an issue.

## Pull requests

Typo fixes, obvious one-line bug fixes, and documentation clarifications can be opened
directly as pull requests. Larger changes need an accepted issue or a direction agreed in a
discussion.

A pull request:

- states the problem it solves,
- contains only changes related to that problem,
- adds or updates tests when behavior changes,
- updates the documentation when public APIs, installation, or release behavior change,
- lists the checks that were run and any that were skipped,
- notes compatibility, migration, and release-note implications.

Commit messages and pull request titles follow
[Conventional Commits](https://www.conventionalcommits.org/), with the affected area as the
scope:

```text
fix(transform): keep variable order in BinarySpinPass
feat(expression): add filter method to filter expression terms by condition
docs: clarify environment context in constraint examples
```

## Development setup

The repository is a Rust workspace with the Python package in `py-lunamodel`, the C++
wrapper in `cxx-lunamodel`, the Rust SDK in `luna-model-ffi`, and the JavaScript package in
`js-lunamodel`. You will need Rust and Cargo, [uv](https://docs.astral.sh/uv/), and Python
3.11 or newer. The C, C++, and extension test suites additionally need
[just](https://github.com/casey/just), CMake, a C++20 compiler, and protobuf (`protoc`), and
the JavaScript tests need [Bun](https://bun.sh). `just doctor` checks for the native
toolchain.

```bash
git clone https://github.com/aqarios/luna-model.git
cd luna-model

# Python development environment
cd py-lunamodel
uv sync
```

Install the Git hooks so formatting and linting run before each commit:

```bash
uv run --directory py-lunamodel pre-commit install
```

## Local checks

Run the checks that match what you changed.

Python package changes (from `py-lunamodel`):

```bash
uv run pytest
uv run ruff check .
uv run ruff format --check .
uv run mypy src
```

Rust workspace changes (from the repository root):

```bash
cargo test --workspace
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
```

C API, C++, and extension changes (from the repository root):

```bash
just ctest            # C test suite
just cpp-test         # C++ test suite
just rs-ext-test      # Rust extension against the local wheel
just rs-plugin-test   # Rust plugin passes driven from Python
just cpp-plugin-test  # C++ plugin passes driven from Python
just cdrift wheelheadersdrift protodrift  # committed header and .proto copies are current
```

`just test` runs every suite (Rust, C, C++, Python, the extensions, and JavaScript) in
parallel, and `just --list` shows all recipes.

To run the same hooks that fire on commit:

```bash
uv run --directory py-lunamodel pre-commit run --all-files
```

If a full run is too expensive for a small change, run the narrowest relevant test and say in
the pull request what you did not run.

## Branches

`main` is the only long-lived branch. Everything else is a short-lived branch merged through a
pull request. When an issue or discussion exists, include its id in the branch name.

| Prefix                  | Use for                                       |
| ----------------------- | --------------------------------------------- |
| `f/<id>-<description>`   | Features and larger improvements              |
| `fix/<id>-<description>` | Bug fixes                                      |
| `docs/<id>-<description>`| Documentation-only work                        |
| `hot/<id>-<description>` | Urgent fixes that need to ship quickly         |

## Releases

Maintainers prepare releases with the `release-prep` workflow. LunaModel follows semantic
versioning: `vX.Y.Z` for stable releases and `vX.Y.Z-alpha.N`, `vX.Y.Z-beta.N`, or
`vX.Y.Z-rc.N` for pre-releases. Pull requests do not bump the version or create release tags
unless a maintainer asks for it.

## License

By contributing, you agree that your contributions are licensed under the Apache License 2.0,
the same license that covers the project. See [LICENSE](./LICENSE).
