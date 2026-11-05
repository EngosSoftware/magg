//! # License generator

use time::OffsetDateTime;

const APACHE_LICENSE: &str = include_str!("apache-license.txt");
const APACHE_NOTICE: &str = include_str!("apache-notice.txt");
const MIT_LICENSE: &str = include_str!("mit-license.txt");
const COPYRIGHT_OWNER: &str = "Dariusz Depta";

pub fn get_apache_license() -> String {
  APACHE_LICENSE.to_string()
}

pub fn get_apache_notice() -> String {
  APACHE_NOTICE
    .replace("{{START-YEAR}}", start_year())
    .replace("{{END-YEAR}}", &end_year())
    .replace("{{COPYRIGHT-OWNER}}", COPYRIGHT_OWNER)
}

pub fn get_mit_license() -> String {
  MIT_LICENSE
    .replace("{{START-YEAR}}", start_year())
    .replace("{{END-YEAR}}", &end_year())
    .replace("{{COPYRIGHT-OWNER}}", COPYRIGHT_OWNER)
}

const fn start_year() -> &'static str {
  "2015"
}

fn end_year() -> String {
  format!("{}", OffsetDateTime::now_utc().year())
}
