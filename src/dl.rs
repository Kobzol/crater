use errors::*;
use reqwest;
use util;

const MAX_REDIRECTS: usize = 4;

pub fn download(url: &str) -> Result<reqwest::Response> {
    util::try_hard(|| download_no_retry(url))
}

pub fn download_limit(url: &str, ms: usize) -> Result<reqwest::Response> {
    util::try_hard_limit(ms, || download_no_retry(url))
}

pub fn download_no_retry(url: &str) -> Result<reqwest::Response> {
    debug!{"Downloading {}", url};
    let client = reqwest::ClientBuilder::new()
        .redirect(reqwest::RedirectPolicy::limited(MAX_REDIRECTS))
        .build()
        .expect("could not setup https client");
    client.get(url).send().map_err(|e| e.into())
}
