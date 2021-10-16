use crate::{escape::escape, statics::CLIENT};
use serde::Deserialize;

#[derive(Deserialize)]
struct Autocomplete {
    text: String,
}

pub async fn gpt(text: String) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    if &text == "" {
        return Ok("Text shouldn't be empty".to_string());
    }
    let payload = r#"{"prompt":""#.to_string()
        + &text
        + r#"","temperature":1,"top_k":40,"top_p":0.9,"seed":0,"stream":false}"#;
    let autocomplete: Autocomplete = CLIENT
        .post("https://bellard.org/textsynth/api/v1/engines/gptj_6B/completions")
        .body(payload)
        .send()
        .await?
        .json()
        .await?;
    let t = autocomplete.text.replace('\n', " ");
    Ok(format!("_Text_:\n*{}* {}", escape(&text), escape(&t)))
}
