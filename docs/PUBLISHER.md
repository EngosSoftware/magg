# Publisher for crates in workspace

## Prerequisites

- Projects this tool is dedicated for, are Rust **workspaces**.
- Workspace manifest file (`Cargo.toml`) MUST start with `[workspace]` section and MUST contain `[workspace.dependencies]` section.
- Workspace dependencies to be published MUST have `path` attribute set instead of `version`.
- Workspace dependencies with `path` are published in the order of appearing in the `[workspace.dependencies]` section.
- Other workspace dependencies SHOUD NOT have the `path` attribute set.
- Workspace dependencies with `path` attribute set MUST NOT have `version` attribute set.
- Workspace dependencies with `path` attribute set are defined in a single line with the `path` as a first attribute (for simpler replacements).

## Validations

- Validate if the `Cargo.toml` in a current directory is a workspace manifest.
- Validate if the workspace has defined dependencies in section `[workspace.dependencies]`.
- Validate if the workspace has a version attribute defined, like this:
  ```toml
  [workspace.package]
  version = "1.0.0"
  ```
- Validate if there are workspace dependencies with `path` attribute (only these will be published).
- Validate if the `path` attribute is the first one.
- Validate if in the workspace `Cargo.toml` the dependency with `path` is formatted exactly this way:
  ```text
  dependency_name = { path = "path_to_the_dependency" }
  ```
  It is crucial for effective text substitution after publishing.
- For each dependency with `path` check if in the directory specified by `path` attribute:
  - exists `Cargo.toml`,
  - name of the package is the same as the dependency name in workspace manifest,
  - version in the package points to the version in workspace,
  - all `[dependencies]` and `[dev-dependencies]` in the package that point to other packages
    in the workspace use the `{ workspace = true }` clause.
- Validate, if after publishing and text substitution, the updated workspace `Cargo.toml` contains
  dependency with the proper version set.

## Publishing process

1. Run validations.
2. Ask if the version to be published is correct. 
3. Publish crates:
   - Change current directory to the one specified in `Cargo.toml`.
   - Ask if proceed with dry-run.
   - Execute `cargo publish --dry-run`.
   - Ask if proceed with publishing.
   - Execute `cargo publish` command.
   - Change back the directory to the workspace.
   - Replace `path` attribute with `version` attribute in workspace `Cargo.toml`.
4. Validate updated workspace `Cargo.toml`.
