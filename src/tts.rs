use memchr::memchr;
use rand::RngCore;

use crate::statics::{CLIENT, RAND_GEN};

pub async fn tts(text: String) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let index = memchr(b' ', text.as_bytes()).ok_or(crate::error::Error::Tts)?;
    let (lang, text) = text.split_at(index);
    let url = format!("https://translate.google.com/translate_tts?ie=UTF-8&q={}&tl={}&tk=418730.60457&client=webapp", text, lang);
    let body = CLIENT.get(url).send().await?.bytes().await?;
    let name = format!("{}.mp3", RAND_GEN.lock().await.next_u64());
    tokio::fs::write(&name, &body).await?;
    Ok(name)
}