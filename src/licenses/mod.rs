//! # License generator

use time::OffsetDateTime;

const APACHE_LICENSE: &str = include_str!("apache-license.txt");
const APACHE_NOTICE: &str = include_str!("apache-notice.txt");
const MIT_LICENSE: &str = include_str!("mit-license.txt");

pub fn get_apache_license() -> String {
    APACHE_LICENSE.to_string()
}

pub fn get_apache_notice(copyright_owner: &str) -> String {
    APACHE_NOTICE
        .replace("{{START-YEAR}}", start_year())
        .replace("{{END-YEAR}}", &end_year())
        .replace("{{COPYRIGHT-OWNER}}", copyright_owner)
}

pub fn get_mit_license(copyright_owner: &str) -> String {
    MIT_LICENSE
        .replace("{{START-YEAR}}", start_year())
        .replace("{{END-YEAR}}", &end_year())
        .replace("{{COPYRIGHT-OWNER}}", copyright_owner)
}

const fn start_year() -> &'static str {
    "2015"
}

fn end_year() -> String {
    format!("{}", OffsetDateTime::now_utc().year())
}
