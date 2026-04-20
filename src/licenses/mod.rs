//! License generator

use time::OffsetDateTime;

const APACHE_2: &str = include_str!("apache-2.txt");
const APACHE_NOTICE: &str = include_str!("apache-notice.txt");
const MIT: &str = include_str!("mit.txt");
const START_YEAR: &str = "2015";
const COPYRIGHT_OWNER: &str = "Dariusz Depta";

pub fn get_apache_2() -> String {
  APACHE_2.to_string()
}

pub fn get_apache_notice() -> String {
  APACHE_NOTICE
    .replace("{{START-YEAR}}", START_YEAR)
    .replace("{{END-YEAR}}", &end_year())
    .replace("{{COPYRIGHT-OWNER}}", COPYRIGHT_OWNER)
}

pub fn get_mit() -> String {
  MIT
    .replace("{{START-YEAR}}", START_YEAR)
    .replace("{{END-YEAR}}", &end_year())
    .replace("{{COPYRIGHT-OWNER}}", COPYRIGHT_OWNER)
}

fn end_year() -> String {
  format!("{}", OffsetDateTime::now_utc().year())
}
