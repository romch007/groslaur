use std::time::Duration;

use anyhow::{Context, Result};
use serde::Deserialize;

const API_URL: &str = "https://aur.archlinux.org/rpc/v5/";

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(dead_code)]
pub struct PackageInfo {
    #[serde(default)]
    pub co_maintainers: Vec<String>,
    #[serde(default)]
    pub depends: Vec<String>,
    pub description: Option<String>,
    pub first_submitted: u32,
    #[serde(rename = "ID")]
    pub id: u32,
    #[serde(default)]
    pub keywords: Vec<String>,
    pub last_modified: u32,
    #[serde(default)]
    pub license: Vec<String>,
    pub maintainer: Option<String>,
    pub name: String,
    pub num_votes: u32,
    #[serde(default)]
    pub opt_depends: Vec<String>,
    pub submitter: Option<String>,
    pub version: String,
    pub popularity: f32,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct ApiResponse {
    pub resultcount: u32,
    pub results: Vec<PackageInfo>,
    #[serde(rename = "type")]
    pub response_type: String,
    pub version: u16,
    pub error: Option<String>,
}

pub fn get_packages_info(packages: &[String]) -> Result<ApiResponse> {
    let mut path = API_URL.to_owned();
    path.push_str("info?");

    for package in packages {
        path.push_str("arg[]=");
        path.push_str(package);
        path.push('&');
    }
    path.pop().unwrap();

    let client = reqwest::blocking::Client::new();
    let response = client
        .get(path)
        .timeout(Duration::from_secs(5))
        .send()
        .with_context(|| "request failed")?;

    let body = response.json().with_context(|| "parsing body failed")?;

    Ok(body)
}

pub fn search_package(term: &str) -> Result<ApiResponse> {
    let mut path = API_URL.to_owned();
    path.push_str("search/");
    path.push_str(term);

    let client = reqwest::blocking::Client::new();
    let response = client
        .get(path)
        .timeout(Duration::from_secs(5))
        .send()
        .with_context(|| "request failed")?;

    let body = response.json().with_context(|| "parsing body failed")?;

    Ok(body)
}
