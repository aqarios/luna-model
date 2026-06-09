# Contributing to LunaModel

Thanks for taking the time to help improve LunaModel. This document explains how the
project is run in practice: how we use issues and discussions, how to propose and submit
changes, and how to get a development environment going.

Please read the first section before opening anything — it is the part that most often
catches people off guard.

## How issues and discussions work

LunaModel keeps a small, deliberately curated issue tracker. **Every open issue is work the
maintainers have already accepted and that is ready to be picked up.** For that reason,
issues are opened and edited by maintainers — not filed directly by the public.

That is not a way of brushing feedback aside; it is the opposite. The front door for
everyone is **[GitHub Discussions](https://github.com/aqarios/luna-model/discussions)**, and
nearly anything belongs there: suspected bugs, feature ideas, questions, modeling help, and
half-formed thoughts you have not fully worked out yet.

The path looks like this:

1. **Open a discussion** describing what you hit or what you would like to see.
2. **A maintainer triages it** — confirming behavior, asking for missing detail, and talking
   through the idea with you.
3. **Accepted work becomes an issue.** When something is confirmed and actionable, a
   maintainer turns the discussion into a tracked issue, and from there it is fair game for a
   pull request.

### Why it works this way

A large share of reports that look like bugs turn out to be configuration mistakes,
environment differences, or an API being used in a way it was not meant to be — not actual
defects. Starting in discussions lets us untangle those quickly, keeps the tracker free of
noise, and means that anything labeled an issue genuinely needs code written for it. The
small amount of friction up front saves a lot of time for everyone afterward.

### Telling bugs and features apart

It helps to know which one you have:

- A **bug** is existing behavior that is broken or that contradicts what the documentation
  promises. Bugs are usually quick to confirm, so they tend to become issues fast.
- A **feature** is behavior that does not exist yet. Features almost always need a short
  conversation about scope and fit first, so expect some back-and-forth in a discussion
  before a feature turns into an issue or a pull request.

## Pull requests

Small, self-evident fixes — a typo, an obvious one-line bug, a documentation clarification —
are welcome as direct pull requests. For anything larger, please make sure there is an
accepted issue or an agreed direction in a discussion before you invest real effort. Even
genuinely good work is hard to merge when the direction was never agreed on, so check first;
it protects your time as much as ours.

When you open a pull request:

- State the user-facing or maintainer-facing problem it solves.
- Keep it focused — leave unrelated cleanup out of feature and bug-fix branches.
- Add or update tests when behavior changes.
- Update the docs when public APIs, installation, or release behavior change.
- List the exact checks you ran (and call out anything you deliberately skipped).
- Note any compatibility, migration, or release-note implications.

Write commit messages in the imperative mood, prefixed with the area they touch:

```text
python: add read_with helpers for expression access
translate: support fixed bounds when emitting LP
docs: clarify environment context in constraint examples
```

## Development setup

The repository is a Rust workspace with the Python package in `py-lunamodel`. You will need
Rust and Cargo, [uv](https://docs.astral.sh/uv/), and Python 3.11 or newer.

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

Releases are handled by the maintainers. LunaModel follows semantic versioning — `vX.Y.Z` for
stable releases, with `rcN`, `bN`, or `aN` suffixes for pre-releases. Please do not bump the
version or create release tags in a pull request unless a maintainer has explicitly asked you
to.

## License

By contributing, you agree that your contributions are licensed under the Apache License 2.0,
the same license that covers the project. See [LICENSE](./LICENSE).
