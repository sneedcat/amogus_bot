use crate::{error::Error, escape::escape};
use hentai::{Hentai, Website};
use serde_json::Value;

pub struct PrintHentai {
    pub thumb: String,
    pub caption: String,
}

pub async fn nhentai(s: String) -> Result<PrintHentai, Box<dyn std::error::Error + Sync + Send>> {
    let filters = tokio::fs::read_to_string("filters.json").await?;
    let v: Value = serde_json::from_str(&filters)?;
    let filters = v["filters"].as_array().ok_or(crate::error::Error::Json)?;
    let response = if s.is_empty() {
        'l: loop {
            let mut filters: Vec<String> = filters
                .iter()
                .map(|s| s.as_str().ok_or(crate::error::Error::Json))
                .filter(|s| s.is_ok())
                .map(|s| s.unwrap().to_string())
                .collect();
            filters.sort();
            let resp = match Hentai::random(Website::NET).await {
                Ok(response) => response,
                Err(_) => continue,
            };
            for tag in &resp.tags {
                if filters.binary_search(&tag.name).is_ok() {
                    continue 'l;
                }
            }
            break resp;
        }
    } else if let Ok(num) = s.parse::<u32>() {
        match Hentai::new(num, Website::NET).await {
            Ok(response) => response,
            Err(err) => return Err(Box::new(Error::Hentai(err))),
        }
    } else {
        let mut tries = 5;
        loop {
            let resp = match Hentai::random(Website::NET).await {
                Ok(response) => response,
                Err(_) => continue,
            };
            let mut ok = false;
            for tag in &resp.tags {
                if tag.name == s {
                    ok = true;
                }
            }
            if ok {
                break resp;
            }
            if tries <= 0 {
                return Err(Box::new(crate::error::Error::Generic));
            }
            tries -= 1;
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

    Ok(PrintHentai {
        thumb: response.thumbnail_url,
        caption,
    })
}
