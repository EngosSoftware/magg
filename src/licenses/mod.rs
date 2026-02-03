//! License generator

use time::OffsetDateTime;

const APACHE_2: &str = include_str!("APACHE-2");
const APACHE_NOTICE: &str = include_str!("APACHE-NOTICE");
const MIT: &str = include_str!("MIT");
const START_YEAR: &str = "2015";
const COPYRIGHT_OWNER: &str = "Dariusz Depta";

pub fn get_apache_2() -> String {
  APACHE_2.to_string()
}

pub fn get_apache_notice() -> String {
  APACHE_NOTICE
    .replace("[START_YEAR]", START_YEAR)
    .replace("[END_YEAR]", &get_year())
    .replace("[COPYRIGHT_OWNER]", COPYRIGHT_OWNER)
}

pub fn get_mit() -> String {
  MIT
    .replace("[START_YEAR]", START_YEAR)
    .replace("[END_YEAR]", &get_year())
    .replace("[COPYRIGHT_OWNER]", COPYRIGHT_OWNER)
}

fn get_year() -> String {
  format!("{}", OffsetDateTime::now_utc().year())
}
