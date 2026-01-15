//! Code of conduct generator

const CODE_OF_CONDUCT: &str = include_str!("CODE_OF_CONDUCT.md");
const MAIL: &str = "[depta@engos.de](mailto:depta@engos.de)";

pub fn get_code_of_conduct() -> String {
  CODE_OF_CONDUCT.replace("::MAIL::", MAIL)
}
