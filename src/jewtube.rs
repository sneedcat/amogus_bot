use html_escape::decode_html_entities;
use rand::RngCore;

use crate::{
    escape::escape,
    statics::{RAND_GEN, RE_EXTRACT_JEWTUBE, RE_JEWTUBE},
};

pub struct BlogPost {
    pub thumb: String,
    pub content: String,
}

const URL: &str = "https://jewtube.com";

pub async fn jewtube() -> Result<BlogPost, Box<dyn std::error::Error + Send + Sync>> {
    let resp = reqwest::get(URL).await?.text().await?;
    //println!("{}", &resp);
    let len = RE_JEWTUBE
        .captures_iter(&resp)
        .filter(|cap| cap.get(0).is_some())
        .count();
    let n = RAND_GEN.lock().await.next_u64() as usize % len;
    let cap = RE_JEWTUBE.captures_iter(&resp).nth(n).unwrap();
    let m = cap.get(0).unwrap();
    let s = m.as_str();
    let capture = RE_EXTRACT_JEWTUBE
        .captures(s)
        .ok_or(crate::error::Error::Jewtube)?;
    let url = decode_html_entities(
        capture
            .name("url")
            .ok_or(crate::error::Error::Jewtube)?
            .as_str(),
    );
    let title = decode_html_entities(
        capture
            .name("title")
            .ok_or(crate::error::Error::Jewtube)?
            .as_str(),
    );
    let thumb = decode_html_entities(
        capture
            .name("thumb")
            .ok_or(crate::error::Error::Jewtube)?
            .as_str(),
    );
    let comments = decode_html_entities(
        capture
            .name("comments")
            .ok_or(crate::error::Error::Jewtube)?
            .as_str(),
    );
    let caption = decode_html_entities(
        capture
            .name("caption")
            .ok_or(crate::error::Error::Jewtube)?
            .as_str(),
    );
    let content = format!(
        "[{}]({})\n{}\n{}",
        escape(&title),
        url,
        escape(&caption),
        comments
    );
    let blogpost = BlogPost {
        thumb: thumb.to_string(),
        content,
    };
    Ok(blogpost)
}
