# Contributing to LunaModel

Thanks for contributing to LunaModel. This repository contains the Rust workspace, the Python package, tests, documentation, release automation, and GitHub Pages build for LunaModel.

The most useful contributions are focused changes with clear motivation: bug fixes, documentation updates, tests, examples, translator improvements, Python API improvements, Rust API improvements, and small refactors that make an existing path easier to maintain.

## Communication

Use the channel that matches the work:

- [Bug report](https://github.com/aqarios/luna-model/issues/new?template=bug.yml): something is broken or behaves unexpectedly.
- [Proposal](https://github.com/aqarios/luna-model/issues/new?template=proposal.yml): a concrete feature or API change.
- [Use case](https://github.com/aqarios/luna-model/issues/new?template=usecase.yml): an optimization workflow is hard or impossible to express, but the implementation path is not clear yet.
- [Discussions](https://github.com/aqarios/luna-model/discussions): questions, examples, design discussion, and usage help.

Small fixes can go straight to a pull request. Larger API, translator, transformation, or release workflow changes should start with an issue or discussion before implementation.

## Branches

`main` is the only long-lived branch. Work happens on short-lived branches and is merged through pull requests.

Recommended branch names:

| Prefix | Use for |
| ------ | ------- |
| `f/<id>-<description>` | Features and larger improvements |
| `fix/<id>-<description>` | Bug fixes |
| `docs/<id>-<description>` | Documentation-only work |
| `ci/<id>-<description>` | CI and release workflow work |
| `hot/<id>-<description>` | Urgent fixes that must be released quickly |

Use the issue, ticket, or discussion id when one exists. For small maintenance work without an issue, use a short descriptive branch name.

## Development Setup

Clone with submodules:

```bash
git clone --recurse-submodules https://github.com/aqarios/luna-model.git
cd luna-model
```

Install Python development dependencies:

```bash
cd py-lunamodel
uv sync
```

The repository requires Rust and Cargo, `uv`, and Python 3.11 or newer.

Install the Git hooks from the repository root:

```bash
uv run --directory py-lunamodel pre-commit install
```

## Local Checks

Run the checks that match the files you changed.

For Python package changes from `py-lunamodel`:

```bash
uv run pytest
uv run ruff check .
uv run ruff format --check .
uv run mypy src
```

For Rust workspace changes from the repository root:

```bash
cargo test --workspace
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
```

To run the same formatting and lint checks used by Git hooks:

```bash
uv run --directory py-lunamodel pre-commit run --all-files
```

For documentation changes from the repository root:

```bash
ci/tools/build-docs-site.sh
```

If a full check is too expensive for the change, run the narrowest relevant test and mention what you did not run in the pull request.

## Pull Requests

Keep pull requests reviewable:

- Describe the user-facing or maintainer-facing problem being solved.
- Keep unrelated cleanup out of feature and bug-fix pull requests.
- Add or update tests when behavior changes.
- Update docs when public APIs, workflows, installation, or release behavior change.
- Include the exact checks you ran.
- Mention any compatibility, migration, or release-note concerns.

Use clear commit messages. A good default is:

```text
area: explain the change in imperative mood
```

Examples:

```text
python: add read_with helpers for expression access
docs: build combined Python and Rust documentation site
tests: package MPS fixtures outside language statistics
```

## Release Process

The project uses semantic versioning.

Version format:

- `vX.Y.Z` for stable releases.
- `vX.Y.ZrcN`, `vX.Y.ZbN`, or `vX.Y.ZaN` for release candidates, beta releases, or alpha releases.

Releases are prepared through the `Prepare Release` workflow:

1. Run the `release-prep.yml` workflow manually and choose the release type.
2. The workflow creates a `release/v<version>` pull request with the version bump.
3. Review and merge the release pull request.
4. The `tag-release.yml` workflow creates the Git tag.
5. The tagged release workflow builds and publishes release artifacts.

Do not create release tags manually unless the automation is unavailable and maintainers agree on the recovery path.

## Generated And Fixture Files

Large or generated fixtures should not dominate GitHub language statistics. Mark fixture formats as generated in `.gitattributes` when adding new solver data files.

The MPS test fixtures are stored as a zip archive and extracted by the tests when needed. Do not commit the extracted `.mps` files.
