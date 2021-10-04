use once_cell::sync::Lazy;
use rand::rngs::StdRng;
use rand::SeedableRng;
use regex::Regex;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::Client;
use tokio::sync::Mutex;

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

pub static CLIENT: Lazy<Client> = Lazy::new(|| {
    let mut headers = HeaderMap::new();
    headers.insert("Accept-Language", HeaderValue::from_static("en-US"));
    reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (X11; Linux x86_64; rv:88.0) Gecko/20100101 Firefox/88.0")
        .default_headers(headers)
        .build()
        .unwrap()
});

pub static RE_SHORTS: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#""content":\{"richGridRenderer":(.*?)},"tabIdentifier":"#).unwrap());
pub static RE_XKCD: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"<div id="comic">\n<img src="(.*?)" title=".*?" alt="(.*?)""#).unwrap()
});

pub static RAND_GEN: Lazy<Mutex<StdRng>> = Lazy::new(|| Mutex::new(SeedableRng::from_entropy()));

pub static BANNED_TAGS: &[&str] = &["lolicon", "yaoi"];
