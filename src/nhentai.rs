use crate::{error::Error, escape::escape, statics::BANNED_TAGS};
use hentai::{Hentai, Website};
pub async fn nhentai(
    s: String,
) -> Result<(String, String), Box<dyn std::error::Error + Sync + Send>> {
    let response = if s.is_empty() {
        let response = loop {
            let resp = match Hentai::random(Website::NET).await {
                Ok(response) => response,
                Err(_err) => continue,
            };
            let mut ok = false;
            for tag in &resp.tags {
                if BANNED_TAGS.contains(&tag.name.as_str()) {
                    ok = true;
                }
            }
            if ok {
                continue;
            }
            break resp;
        };
        response
    } else {
        match Hentai::new(s.parse::<u32>()?, Website::NET).await {
            Ok(response) => response,
            Err(err) => return Err(Box::new(Error::Hentai(err))),
        }
    };
    let mut caption = format!(
        "*{}*\nSauce: [{}]({})\nNumber of pages: {}\nTags:\n",
        escape(
            &response
                .title
                .pretty
                .ok_or_else(|| crate::error::Error::Hentai(
                    std::io::Error::new(std::io::ErrorKind::Unsupported, "sus").into()
                ))?
        ),
        response.id,
        &response.url,
        response.num_pages,
    );
    for tag in response.tags {
        caption += &format!("[{}](https://nhentai.net{}) ", escape(&tag.name), tag.url);
    }

    Ok((response.thumbnail_url, caption))
}
