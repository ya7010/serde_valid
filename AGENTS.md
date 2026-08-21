# Agent instructions

## Release procedure

- Before creating a release tag, update the repository's package/workspace version and any related internal dependency versions to the release version.
- Merge the version update into the default branch first.
- Create and push the release tag only after the version update has been merged, so the tag points to the merged commit.
