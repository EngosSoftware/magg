# Publisher for crates in workspace

## Prerequisites

- [x] The Rust project this tool is executed in, is a **workspace**.
- [x] Workspace `Cargo.toml` (starts with `[workspace]`) contains a section `[workspace.dependencies]`.
- [x] Workspace dependencies have multiple crates that have `path` attribute set instead of `version`.
- [x] Dependencies with `path` are sorted in the order of publishing.
- [ ] No other workspace dependencies have the `path` attribute set.
- [x] Dependencies with `path` attribute MUST NOT have `version` attribute set.
- [x] Dependencies with `path` are defined in a single line with the `path` as a first attribute (for simpler replacements).

## Some ideas on validations

- [x] Validate if the `Cargo.toml` in a current directory is a workspace.
- [x] Validate if the workspace has defined dependencies, like `[workspace.dependencies]`.
- [x] Validate if the workspace has a version attribute defined, like `[workspace.package]\nversion = "3.0.3"`.
- [x] Validate if there are dependencies with `path` key (these will be published).
- [x] Validate if the `path` attribute is the first one.
- [x] Validate if in the workspace `Cargo.toml` the dependency with `path` is formatted exactly this way:
  ```text
  dependency_name = { path = "path_to_the_dependency" }
  ```
  It is crucial for text substitution after publishing.
- For each dependency with `path` check if in the specified directory:
  - [x] exists `Cargo.toml`,
  - [x] name of the package is the same as the dependency name,
  - [x] version in the package points to the version in workspace,
  - [x] all `[dependencies]` and `[dev-dependencies]` in the package that point to other packages
    in the workspace use the `{ workspace = true }` clause.
- Validate, if after publishing and text substitution, the updated workspace `Cargo.toml` contains
  dependency with the proper version set.

## Findings

- `toml` crate preserves the order of the keys in the attribute table.

## Publishing process

1. Run validations.
2. Ask if the version to be published is correct. 
3. Publish crates:
   - Change current directory to the one specified in `Cargo.toml`.
   - Execute `cargo publish --dry-run`.
   - Ask if proceed with publishing.
   - Execute `cargo publish` command.
   - Change back the directory to the workspace.
   - Replace `path` attribute with `version` attribute in workspace `Cargo.toml`.
   - Validate updated workspace `Cargo.toml`.
