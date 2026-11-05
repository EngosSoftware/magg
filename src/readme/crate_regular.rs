//! # `README.md` file generator for regular crates

use super::*;

/// Creates a `README.md` file for a regular crate.
pub fn get_readme_for_regular_crate(file_name: impl AsRef<Path>) -> Result<String> {
  let mut output = String::new();
  let body = utils::read_file(file_name)?;
  let parsed_toml = utils::parse_toml("Cargo.toml")?;
  let package_name = utils::get_package_name(&parsed_toml);
  let repository_url = utils::get_repository(&parsed_toml)
    .strip_suffix(".git")
    .expect("repository name does not end with '.git' suffix");
  let workflows = format!("{repository_url}/actions/workflows");
  // Write the name of the package.
  _ = writeln!(&mut output, "### {}", package_name);
  _ = writeln!(&mut output);
  // Write badges.
  _ = writeln!(&mut output, "[![crates.io][crates-badge]][crates-url]");
  _ = writeln!(&mut output, "![build][build-badge]");
  _ = writeln!(&mut output, "[![docs][docs-badge]][docs-url]");
  _ = writeln!(&mut output, "![coverage][cov-badge]");
  _ = writeln!(&mut output, "![license][license-badge]");
  _ = writeln!(&mut output, "![human][human-badge]");
  _ = writeln!(&mut output);
  // Write links to badges and files.
  _ = writeln!(&mut output, "[crates-badge]: https://img.shields.io/crates/v/{package_name}.svg");
  _ = writeln!(&mut output, "[crates-url]: https://crates.io/crates/{package_name}");
  _ = writeln!(&mut output, "[build-badge]: {workflows}/build.yml/badge.svg");
  _ = writeln!(&mut output, "[docs-badge]: https://docs.rs/{package_name}/badge.svg");
  _ = writeln!(&mut output, "[docs-url]: https://docs.rs/{package_name}");
  _ = writeln!(&mut output, "[cov-badge]: https://img.shields.io/badge/coverage-0%25-21B577.svg");
  _ = writeln!(&mut output, "[license-badge]: https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-{LICENSE_COLOR}.svg");
  _ = writeln!(&mut output, "[human-badge]: https://img.shields.io/badge/made_by-HUMAN-{HUMAN_COLOR}.svg");
  _ = writeln!(&mut output);
  // Write the body content.
  _ = write!(&mut output, "{body}");
  _ = writeln!(&mut output);
  // Write license section.
  _ = writeln!(&mut output, "## License");
  _ = writeln!(&mut output);
  _ = writeln!(&mut output, "Licensed under either of");
  _ = writeln!(&mut output);
  _ = writeln!(&mut output, "- [Apache License, Version 2.0](https://www.apache.org/licenses/LICENSE-2.0)");
  _ = writeln!(&mut output, "  ([LICENSE-APACHE](LICENSE-APACHE) and [NOTICE](NOTICE))");
  _ = writeln!(&mut output, "- [MIT license](https://opensource.org/licenses/mit)");
  _ = writeln!(&mut output, "  ([LICENSE-MIT](LICENSE-MIT))");
  _ = writeln!(&mut output);
  _ = writeln!(&mut output, "at your option.");
  _ = writeln!(&mut output);
  // Write contribution section.
  _ = writeln!(&mut output, "## Contribution");
  _ = writeln!(&mut output);
  _ = writeln!(&mut output, "Any contributions intentionally submitted for inclusion in the work by you,");
  _ = writeln!(&mut output, "shall be dual licensed as above, without any additional terms or conditions.");
  Ok(output)
}
