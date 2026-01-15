use crate::utils::get_file;

mod readme;
mod utils;

fn main() {
  let parsed_toml = utils::parse_toml("Cargo.toml");
  let package_name = utils::get_package_name(&parsed_toml);
  let repository_url = utils::get_repository(&parsed_toml)
    .strip_suffix(".git")
    .expect("repository name does not end with '.git' suffix");
  let body = get_file("manual/src/README.md");
  readme::scaffold_readme(package_name, repository_url, &body);
}
