use rand::RngCore;
use serde::Deserialize;

use crate::{escape::escape, statics::RAND_GEN};
const URL: &str = "https://api.urbandictionary.com/v0/define?term=";

#[derive(Debug, Deserialize)]
struct Definition {
    definition: String,
    example: String,
    permalink: String,
    thumbs_up: usize,
    thumbs_down: usize,
}

#[derive(Debug, Deserialize)]
struct Definitions {
    list: Vec<Definition>,
}

pub async fn urban(s: String) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    //let s = String::new();
    let url = format!("{}{}", URL, s);
    let definitions: Definitions = reqwest::get(url).await?.json().await?;
    let len = definitions.list.len();
    let rand_num = RAND_GEN.lock().await.next_u64() as usize;
    let index = rand_num % len;
    let item = &definitions.list[index];
    let caption = format!(
        "*Definition:*\n{}\n\n*Example:*\n{}\nScore: {}\\-{}\n[Permalink]({})",
        escape(&item.definition),
        escape(&item.example),
        item.thumbs_up,
        item.thumbs_down,
        item.permalink
    );
    Ok(caption)
}
