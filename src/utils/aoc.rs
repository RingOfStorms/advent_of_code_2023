use crate::prelude::*;
use reqwest::{
    header::{self, COOKIE},
    Client,
};
use reqwest_middleware::{ClientBuilder as MiddlewareClientBuilder, ClientWithMiddleware};
use reqwest_retry::{policies::ExponentialBackoff, RetryTransientMiddleware};
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::PathBuf;

static AOC_PUZZLE_INPUT_CACHE: &str = "aoc_puzzle_cache";

pub async fn get_puzzle_input(day: u8) -> Result<String> {
    let file_name = format!("day_{:02}", day);
    let cache_path = PathBuf::from(AOC_PUZZLE_INPUT_CACHE).join(file_name);
    if cache_path.exists() {
        // Read from the cache file
        let mut cache_file = File::open(cache_path)?;
        let mut contents = String::new();
        cache_file.read_to_string(&mut contents)?;
        Ok(contents)
    } else {
        // Fetch the content from the URL
        let response = aoc_client()
            .await?
            .get(format!("https://adventofcode.com/2023/day/{}/input", day))
            .send()
            .await?
            .text()
            .await?;

        // Cache the content to a file Ensure the cache directory exists
        fs::create_dir_all(AOC_PUZZLE_INPUT_CACHE)?;
        let mut cache_file = File::create(cache_path)?;
        cache_file.write_all(response.as_bytes())?;
        Ok(response)
    }
}

pub async fn aoc_client() -> Result<ClientWithMiddleware> {
    let session = &get_config().aoc_session;
    let mut session_cookie = header::HeaderValue::from_str(&format!("session={}", session))
        .expect("failed to create header with api_key.");
    session_cookie.set_sensitive(true);
    let mut headers = header::HeaderMap::new();
    headers.insert(COOKIE, session_cookie);
    let client = Client::builder()
        .default_headers(headers)
        .gzip(true)
        .brotli(true)
        .deflate(true)
        .https_only(false)
        .build()?;
    let client = MiddlewareClientBuilder::new(client)
        .with(RetryTransientMiddleware::new_with_policy(
            ExponentialBackoff::builder().build_with_max_retries(2),
        ))
        .build();
    Ok(client)
}
