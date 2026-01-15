//! # Composer for README.md files

const TWO_SPACES: &str = "  ";

pub fn scaffold_readme(package_name: &str, repository_url: &str, body: &str) {
  println!("### {}", package_name);
  println!();
  println!("[![crates.io][crates-badge]][crates-url]");
  println!("[![coverage][cov-badge]][cov-url]{TWO_SPACES}");
  println!("![build Linux][build-badge-linux]");
  println!("![build Windows][build-badge-windows]");
  println!("![build macOs][build-badge-macos]");
  println!("![build macOs arm64][build-badge-macos-arm64]{TWO_SPACES}");
  println!("[![mit-license][mit-badge]][mit-license-url]");
  println!("[![apache-license][apache-badge]][apache-license-url]");
  println!("[![cc][cc-badge]][cc-url]{TWO_SPACES}");
  println!("[![mbh][mbh-badge]][mbh-url]");
  println!("[![es][es-badge]][es-url]");
  println!();
  println!("[crates-badge]: https://img.shields.io/crates/v/{package_name}.svg");
  println!("[crates-url]: https://crates.io/crates/{package_name}");
  println!("[cov-badge]: https://img.shields.io/badge/coverage-0%25-21b577.svg");
  println!("[cov-url]: https://crates.io/crates/coverio");
  println!("[build-badge-linux]: {repository_url}/actions/workflows/build-linux.yml/badge.svg");
  println!("[build-badge-windows]: {repository_url}/actions/workflows/build-windows.yml/badge.svg");
  println!("[build-badge-macos]: {repository_url}/actions/workflows/build-macos.yml/badge.svg");
  println!("[build-badge-macos-arm64]: {repository_url}/actions/workflows/build-macos-arm64.yml/badge.svg");
  println!("[mit-badge]: https://img.shields.io/badge/License-MIT-blue.svg");
  println!("[mit-url]: https://opensource.org/licenses/MIT");
  println!("[mit-license-url]: {repository_url}/blob/main/LICENSE-MIT");
  println!("[apache-badge]: https://img.shields.io/badge/License-Apache%202.0-blue.svg");
  println!("[apache-url]: https://www.apache.org/licenses/LICENSE-2.0");
  println!("[apache-license-url]: {repository_url}/blob/main/LICENSE");
  println!("[apache-notice-url]: {repository_url}/blob/main/NOTICE");
  println!("[cc-badge]: https://img.shields.io/badge/Contributor%20Covenant-2.1-blue.svg");
  println!("[cc-url]: {repository_url}/blob/main/CODE_OF_CONDUCT.md");
  println!("[mbh-badge]: https://img.shields.io/badge/Made_by-HUMAN-D81B60.svg");
  println!("[mbh-url]: https://github.com/DariuszDepta");
  println!("[es-badge]: https://img.shields.io/badge/Brought_to_you_by-Engos_Software-43A047.svg");
  println!("[es-url]: https://engos.de");
  println!("[repository-url]: {repository_url}");
  println!();
  print!("{body}");
  println!();
  println!("## License\n");
  println!("Licensed under either of\n");
  println!("- [MIT license][mit-url] (see [LICENSE-MIT][mit-license-url]) or");
  println!("- [Apache License, Version 2.0][apache-url] (see [LICENSE][apache-license-url] and [NOTICE][apache-notice-url])\n");
  println!("at your option.\n");
  println!("## Contribution\n");
  println!("Any contributions to [{package_name}][repository-url] are greatly appreciated.");
  println!("All contributions intentionally submitted for inclusion in the work by you,");
  println!("shall be dual licensed as above, without any additional terms or conditions.");
}
