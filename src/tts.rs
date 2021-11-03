use memchr::memchr;
use rand::RngCore;

use crate::statics::{CLIENT, RAND_GEN};

pub async fn tts(text: String) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let (lang, text) = match memchr(b' ', text.as_bytes()) {
        Some(index) => text.split_at(index),
        None => ("en", text.as_str()),
    };
    let url = format!("https://translate.google.com/translate_tts?ie=UTF-8&q={}&tl={}&tk=418730.60457&client=webapp", text, lang);
    let body = CLIENT.get(url).send().await?.bytes().await?;
    let name = format!("{}.mp3", RAND_GEN.lock().await.next_u64());
    tokio::fs::write(&name, &body).await?;
    Ok(name)
}
