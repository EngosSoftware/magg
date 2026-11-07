//! # Lists owned crates

use crate::utils;
use serde::Deserialize;

/// API URL for listing crates.
const URL_CRATES: &str = "https://crates.io/api/v1/crates";

#[derive(Deserialize)]
struct CratesResponse {
    crates: Vec<Crate>,
    meta: Meta,
}

#[derive(Deserialize)]
pub struct Crate {
    pub name: String,
    pub max_version: String,
    pub description: String,
    pub repository: String,
}

#[derive(Deserialize)]
struct Meta {
    next_page: Option<String>,
}

pub fn get_crates(id: &str) -> Vec<Crate> {
    let user_agent = format!(
        "{0}/{1} (https://crates.io/{0})",
        utils::CREATE_NAME,
        utils::CREATE_VERSION
    );
    let client = reqwest::blocking::Client::builder()
        .user_agent(user_agent)
        .build()
        .unwrap();
    let page_size = 100;
    let mut url = format!("{URL_CRATES}?user_id={id}&per_page={page_size}");
    let mut crates = vec![];
    loop {
        let resp: CratesResponse = client.get(url).send().unwrap().json().unwrap();
        crates.extend(resp.crates);
        match resp.meta.next_page {
            Some(next) => url = format!("{URL_CRATES}{next}"),
            None => break,
        }
    }
    crates
}
