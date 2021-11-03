use std::error::Error;

use crate::{
    escape::escape,
    statics::{CLIENT, RAND_GEN, RE_EXTRACT_XKCD, RE_XKCD},
};

use rand::RngCore;

pub struct Comic {
    pub comic_url: String,
    pub title: String,
}

pub async fn xkcd(s: String) -> Result<Comic, Box<dyn Error + Sync + Send>> {
    let url = if !s.is_empty() {
        format!("https://xkcd.com/{}/", s)
    } else {
        let s = CLIENT.get("https://xkcd.com/").send().await?.text().await?;
        let captures = RE_EXTRACT_XKCD
            .captures(&s)
            .ok_or(crate::error::Error::Xkcd)?;
        if captures.len() != 2 {
            return Err(Box::new(crate::error::Error::Xkcd));
        }
        let number = captures.get(1).ok_or(crate::error::Error::Xkcd)?;
        let n = number.as_str().parse::<u64>()?;
        let num = RAND_GEN.lock().await.next_u64();
        let mut n = num % n;
        if n == 404 {
            n = 400;
        }
        format!("https://xkcd.com/{}/", n)
    };
    let s = CLIENT.get(&url).send().await?.text().await?;
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
