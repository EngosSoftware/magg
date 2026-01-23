# Publisher for crates in workspace

## Prerequisites

- The Rust project this tool is executed in, is a **workspace**.
- Workspace `Cargo.toml` (starts with `[workspace]`) contains a section `[workspace.dependencies]`.
- Workspace dependencies have multiple crates that have `path` attribute set instead of `version`.
- Dependencies with `path` are sorted in the order of publishing.
- No other workspace dependencies have the `path` attribute set.
- Dependencies with `path` attribute MUST NOT have `version` attribute set.
- Dependencies with `path` are defined in a single line with the `path` as a first attribute (for simpler replacements).

## Some ideas on validations

- Validate if the `Cargo.toml` in a current directory is a workspace.
- Validate if the workspace has defined dependencies, like `[workspace.dependencies]`.
- Validate if the workspace has a version attribute defined.
- Validate if there are dependencies with `path` key (these will be published).
- Validate if the `path` attribute is the first one.
- Validate if in the workspace `Cargo.toml` the dependency with `path` is formatted exactly this way:
  ```text
  dependency_name = { path = "path_to_the_dependency" }
  ```
  It is crucial for text substitution after publishing.
- For each dependency with `path` check if in the specified directory:
  - exists `Cargo.toml`,
  - name of the package is the same as the dependency name,
  - version in the package points to the version in workspace.
- Validate, if after publishing and text substitution, the updated workspace `Cargo.toml` contains
  dependency with the proper version set.

## Findings

- `toml` crate preserves the order of the keys in the attribute table.

## Publishing process

1. Run validations.
2. Publish crates:
   - Change current directory to the one specified in `Cargo.toml`.
   - Execute `cargo publish --dry-run`.
   - Ask if proceed with publishing.
   - Execute `cargo publish` command.
   - Change back the directory to the workspace.
   - Replace `path` attribute with `version` attribute in workspace `Cargo.toml`.
   - Validate updated workspace `Cargo.toml`.
