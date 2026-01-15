//! # README.md file generator

use crate::utils;
use crate::utils::read_file;
use std::fmt::Write;
use std::path::Path;

const TWO_SPACES: &str = "  ";

const LICENSE_COLOR: &str = "4169E1";
const HUMAN_COLOR: &str = "DC143C";
const ENGOS_COLOR: &str = "32CD32";

pub fn scaffold_readme(file_name: impl AsRef<Path>) -> String {
  let mut output = String::new();
  let body = read_file(file_name);
  let parsed_toml = utils::parse_toml("Cargo.toml");
  let package_name = utils::get_package_name(&parsed_toml);
  let repository_url = utils::get_repository(&parsed_toml)
    .strip_suffix(".git")
    .expect("repository name does not end with '.git' suffix");
  // Write the name of the package.
  _ = writeln!(&mut output, "### {}", package_name);
  _ = writeln!(&mut output);
  // Write badges.
  _ = writeln!(&mut output, "[![crates.io][crates-badge]][crates-url]");
  _ = writeln!(&mut output, "[![coverage][cov-badge]][cov-url]{TWO_SPACES}");
  _ = writeln!(&mut output, "![build Linux][build-badge-linux]");
  _ = writeln!(&mut output, "![build Windows][build-badge-windows]");
  _ = writeln!(&mut output, "![build macOs][build-badge-macos]");
  _ = writeln!(&mut output, "![build macOs arm64][build-badge-macos-arm64]{TWO_SPACES}");
  _ = writeln!(&mut output, "[![mit-license][mit-badge]][mit-license-url]");
  _ = writeln!(&mut output, "[![apache-license][apache-badge]][apache-license-url]");
  _ = writeln!(&mut output, "[![cc][cc-badge]][cc-url]{TWO_SPACES}");
  _ = writeln!(&mut output, "[![mbh][mbh-badge]][mbh-url]");
  _ = writeln!(&mut output, "[![es][es-badge]][es-url]\n");
  // Write links to badges and files.
  _ = writeln!(&mut output, "[crates-badge]: https://img.shields.io/crates/v/{package_name}.svg");
  _ = writeln!(&mut output, "[crates-url]: https://crates.io/crates/{package_name}");
  _ = writeln!(&mut output, "[cov-badge]: https://img.shields.io/badge/coverage-0%25-21b577.svg");
  _ = writeln!(&mut output, "[cov-url]: https://crates.io/crates/coverio");
  _ = writeln!(&mut output, "[build-badge-linux]: {repository_url}/actions/workflows/build-linux.yml/badge.svg");
  _ = writeln!(&mut output, "[build-badge-windows]: {repository_url}/actions/workflows/build-windows.yml/badge.svg");
  _ = writeln!(&mut output, "[build-badge-macos]: {repository_url}/actions/workflows/build-macos.yml/badge.svg");
  _ = writeln!(&mut output, "[build-badge-macos-arm64]: {repository_url}/actions/workflows/build-macos-arm64.yml/badge.svg");
  _ = writeln!(&mut output, "[mit-badge]: https://img.shields.io/badge/License-MIT-{LICENSE_COLOR}.svg");
  _ = writeln!(&mut output, "[mit-url]: https://opensource.org/licenses/MIT");
  _ = writeln!(&mut output, "[mit-license-url]: {repository_url}/blob/main/LICENSE-MIT");
  _ = writeln!(&mut output, "[apache-badge]: https://img.shields.io/badge/License-Apache%202.0-{LICENSE_COLOR}.svg");
  _ = writeln!(&mut output, "[apache-url]: https://www.apache.org/licenses/LICENSE-2.0");
  _ = writeln!(&mut output, "[apache-license-url]: {repository_url}/blob/main/LICENSE");
  _ = writeln!(&mut output, "[apache-notice-url]: {repository_url}/blob/main/NOTICE");
  _ = writeln!(&mut output, "[cc-badge]: https://img.shields.io/badge/Contributor%20Covenant-2.1-{LICENSE_COLOR}.svg");
  _ = writeln!(&mut output, "[cc-url]: {repository_url}/blob/main/CODE_OF_CONDUCT.md");
  _ = writeln!(&mut output, "[mbh-badge]: https://img.shields.io/badge/Made_by_a-HUMAN-{HUMAN_COLOR}.svg");
  _ = writeln!(&mut output, "[mbh-url]: https://github.com/DariuszDepta");
  _ = writeln!(&mut output, "[es-badge]: https://img.shields.io/badge/at-Engos_Software-{ENGOS_COLOR}.svg");
  _ = writeln!(&mut output, "[es-url]: https://engos.de");
  _ = writeln!(&mut output, "[repository-url]: {repository_url}");
  // Write the content.
  _ = writeln!(&mut output);
  _ = write!(&mut output, "{body}");
  _ = writeln!(&mut output);
  // Write license section.
  _ = writeln!(&mut output, "## License");
  _ = writeln!(&mut output);
  _ = writeln!(&mut output, "Licensed under either of");
  _ = writeln!(&mut output);
  _ = writeln!(&mut output, "- [MIT license][mit-url] (see [LICENSE-MIT][mit-license-url]) or");
  _ = writeln!(
    &mut output,
    "- [Apache License, Version 2.0][apache-url] (see [LICENSE][apache-license-url] and [NOTICE][apache-notice-url])"
  );
  _ = writeln!(&mut output);
  _ = writeln!(&mut output, "at your option.");
  _ = writeln!(&mut output);
  // Write contribution section.
  _ = writeln!(&mut output, "## Contribution");
  _ = writeln!(&mut output);
  _ = writeln!(&mut output, "Any contributions to [{package_name}][repository-url] are greatly appreciated.");
  _ = writeln!(&mut output, "All contributions intentionally submitted for inclusion in the work by you,");
  _ = writeln!(&mut output, "shall be dual licensed as above, without any additional terms or conditions.");
  _ = writeln!(&mut output);
  output
}
