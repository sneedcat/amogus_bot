use std::error::Error;

use crate::{
    escape::escape,
    statics::{CLIENT, RE_XKCD},
};

pub struct Comic {
    pub comic_url: String,
    pub title: String,
}

pub async fn xkcd(s: String) -> Result<Comic, Box<dyn Error + Sync + Send>> {
    let url = if !s.is_empty() {
        format!("https://xkcd.com/{}/", s)
    } else {
        "https://xkcd.com/".to_owned()
    };
    let resp = CLIENT.get(&url).send().await?;
    let s = resp.text().await?;
    let captures = RE_XKCD.captures(&s).ok_or(crate::error::Error::Xkcd)?;
    if captures.len() != 3 {
        return Err(crate::error::Error::Xkcd.into());
    }
    let url = captures.get(1).ok_or(crate::error::Error::Xkcd)?.as_str();
    let title = captures.get(2).ok_or(crate::error::Error::Xkcd)?.as_str();
    Ok(Comic {
        comic_url: format!("https:{}", url.to_owned()),
        title: format!("*{}*\n", escape(title)),
    })
}
