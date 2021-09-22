use once_cell::sync::Lazy;
use regex::Regex;
use reqwest::header::{HeaderValue, HeaderMap};
use reqwest::cookie::Jar;
use reqwest::{Url, Client};
use std::sync::Arc;
use rand::rngs::StdRng;
use tokio::sync::Mutex;
use rand::SeedableRng;

pub static SHORTS_CLIENT: Lazy<Client> = Lazy::new(|| {
    let mut headers = HeaderMap::new();
    headers.insert("Accept-Language", HeaderValue::from_static("en-US"));
    headers.insert("CONSENT", HeaderValue::from_static("YES+42"));
    reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (X11; Linux x86_64; rv:88.0) Gecko/20100101 Firefox/88.0")
        .default_headers(headers)
        .build()
        .unwrap()
});

pub static RE_SHORTS: Lazy<Regex> = Lazy::new(|| Regex::new(r#""content":\{"richGridRenderer":(.*?)},"tabIdentifier":"#).unwrap());

pub static RAND_GEN: Lazy<Mutex<StdRng>> = Lazy::new(|| Mutex::new(SeedableRng::from_entropy()));