# Versioning strategy

We are using semantic versioning.

**vX.Y.Z**
- X is the major version
- Y is the minor version
- Z is the patch version

For release candidates we use:

**vX.Y.ZrcA**
- A is the number of the release candidate.
- vX.Y.Z is the targeted release number. It's a version not yet released.

# Branches Overview

This git branching strategy follows the [One Flow](https://www.endoflineblog.com/oneflow-a-git-branching-model-and-workflow) concept.

There is only one long-lived branch, `main`.
Our main difference to the One Flow approach is, that we don’t keep old release branches, since we have to use tags for the versions.

| Branch | Description |
| --- | --- |
| main | Main branch |
| f/y_xxxx | Feature branch. Implement here new features. |
| ci/y_xxxx | CI branch. Implement changes to the ci system here. |
| fix/y_xxxx | Bug fixes (not hotfixes) |
| hot/y_xxxx | Hotfixes. These fixes need to be published as a minor version update |
| release | Bump the version here. To release the SW create a Tag after the version was bumped. After the release this branch can be deleted (and for the next recreated). |
| pre-release | Tag pre-releases here. Also bump the version here (only minor version increases). After the release this branch can be deleted (and for the next recreated) |
- y is always an issue/ticket/bug number
- x can be any description
- All releases have to be tagged with a version number (use the same number as used inside the `Cargo.toml`).
- Since all versions are tagged, hotfixes and releases can be merged at any point into individual ongoing feature branches.

/heading

## Feature/Bug fix/CI branch

Feature and bug fix branches are handled 1:1 the same. The main difference is the naming. With this it’s a bit easier for us to keep track which branch does what.

```mermaid
gitGraph
commit
branch fix/y_xxxx
branch f/y_xxxx
branch ci/y_xxxx
checkout fix/y_xxxx
commit
checkout main
merge fix/y_xxxx
checkout f/y_xxxx
commit
checkout main
merge f/y_xxxx
checkout ci/y_xxxx
commit
checkout main
merge ci/y_xxxx
```

### Start

```bash
git checkout -b f/y_xxxx  main
```

### Finish

Create a PR to merge it into main. Delete the branch after its merged. → We use option two of the blog post.

## Release/Pre-release branch

Creating pre-release and regular releases only differ in the version/tag you publish. Both are based on main (It can also be an older commit, if you don’t want to include the latest features).

Our main difference to the One Flow approach is, that we don’t keep old release branches, since we have to use tags for the versions. If you at some point want to implement/change something based of a specific release create a branch from this tag.

```mermaid
gitGraph
commit
branch release
checkout main
commit
checkout release
commit id: "bump v1.1.0" tag: "v1.1.0"
checkout main
merge release
```

### Start

If a release branch already exists, delete it. Then create release branch based on the desired commit.

```bash
git checkout -b release  main
```

### Release

When you are ready to release the SW you have to do these steps:
1. Ensure, that the version used in the `Cargo.toml` is the same as the version you want to create (also run an `uv sync` to update everything).
2. Create a tag for the latest commit in the release branch

### Finish

1. Merge the release branch back into the main branch (this updates e.g., the version number in the main branch).
2. Delete the release branch

## Hotfix branch

For bugs, we need to fix asap.

```mermaid
gitGraph
commit
branch hot/y_xxxx
checkout main
commit
checkout hot/y_xxxx
commit id: "fix something"
commit id: "bump v1.1.1" tag: "v1.1.1"
checkout main
merge hot/y_xxxx
```

### Start

Create Hotfix branch like this

```bash
git checkout -b hot/y_xxxx vX.Y.Z
```

### Release

When you are ready to release the hotfix you have to do these steps:
1. Bump the version to vX.Y.Z+1
2. Ensure, that the version used in the `Cargo.toml` is the same as the version you want to create (also run an `uv sync` to update everything).
3. Create a tag for the latest commit in the hotfix branch

### Finish

1. Merge the hotfix branch back into the main branch (this updates e.g., the version number in the main branch).
2. Delete the release branch
