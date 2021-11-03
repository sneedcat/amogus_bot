use crate::statics::CLIENT;
use reqwest::header::{HeaderValue, CONTENT_TYPE};
use serde::Deserialize;
use std::error::Error;

#[derive(Deserialize)]
struct ShortenResult {
    result_url: String,
}

pub async fn shorten(url: &str) -> Result<String, Box<dyn Error + Send + Sync>> {
    let data = format!("url={}", url_escape::encode_userinfo(url));
    let result: ShortenResult = CLIENT
        .post("https://cleanuri.com/api/v1/shorten")
        .body(data)
        .header(
            CONTENT_TYPE,
            HeaderValue::from_static("application/x-www-form-urlencoded"),
        )
        .send()
        .await?
        .json()
        .await?;
    let text = result.result_url.replace("\\/", "/");
    Ok(text)
}
