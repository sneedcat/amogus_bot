use std::error::Error;

use m3u8_rs::playlist::{Playlist, VariantStream};
use rand::RngCore;
use reqwest::Response;
use roux::Subreddit;

use crate::{
    escape::escape,
    ffmpeg::convert_audio_and_video_to_mp4,
    statics::{CLIENT, RAND_GEN},
};

async fn make_request(url: &str, file: &str) -> Result<Response, Box<dyn Error + Send + Sync>> {
    let download_url = format!("{}/{}", url, file);
    Ok(CLIENT.get(&download_url).send().await?)
}

async fn generate_buffer(url: &str, file: &str) -> Result<Vec<u8>, Box<dyn Error + Send + Sync>> {
    let resp = make_request(url, file).await?;
    let bytes = resp.bytes().await?;
    let data = match m3u8_rs::parse_playlist_res(&bytes[..]) {
        Ok(s) => s,
        Err(_) => return Err(Box::new(crate::error::Error::Reddit)),
    };
    let p = match data {
        Playlist::MediaPlaylist(p) => p,
        _ => return Err(Box::new(crate::error::Error::Reddit)),
    };

    let mut buf = Vec::new();
    for segment in p.segments {
        let resp = make_request(&url, &segment.uri).await?;
        let bytes = resp.bytes().await?;
        buf.extend_from_slice(&bytes[..]);
    }
    Ok(buf)
}

pub async fn reddit(s: &str) -> Result<(String, String), Box<dyn Error + Send + Sync>> {
    let subreddit = Subreddit::new(s);
    let hot = subreddit.hot(25, None).await?;
    let arr: Vec<_> = hot
        .data
        .children
        .into_iter()
        .filter(|post| post.data.url.is_some() && post.data.domain == "v.redd.it")
        .collect();
    if arr.is_empty() {
        return Err(Box::new(crate::error::Error::Reddit));
    }
    let n = RAND_GEN.lock().await.next_u64() as usize % arr.len();
    let post = &arr[n];
    let caption = format!("*{}*\n`Score: {}`\n[Number of comments:{}]({})", escape(&post.data.title), post.data.score, post.data.num_comments, format!("https://reddit.com{}", post.data.permalink));
    let url = post.data.url.as_ref().unwrap();
    let resp = make_request(&url, "HLSPlaylist.m3u8").await?;
    let bytes = resp.bytes().await?;
    let data = match m3u8_rs::parse_playlist_res(&bytes[..]) {
        Ok(s) => s,
        Err(_) => return Err(Box::new(crate::error::Error::Reddit)),
    };
    let p = match data {
        Playlist::MasterPlaylist(p) => p,
        _ => return Err(Box::new(crate::error::Error::Reddit)),
    };
    let var = p
        .variants
        .iter()
        .filter(|v| v.audio == Some("0".to_string()) && !v.alternatives.is_empty())
        .fold(None, |acc: Option<VariantStream>, var| match acc {
            Some(v) => {
                let a = v.bandwidth.parse::<u64>().unwrap();
                match var.bandwidth.parse::<u64>() {
                    Ok(b) => {
                        if a < b {
                            return Some(var.clone());
                        }
                        Some(v)
                    }
                    Err(_) => return Some(v),
                }
            }
            None => match var.bandwidth.parse::<u64>() {
                Ok(_) => Some(var.clone()),
                _ => None,
            },
        });
    if var.is_none() {
        return Err(Box::new(crate::error::Error::Reddit));
    }
    let var = var.unwrap();
    let folder = RAND_GEN.lock().await.next_u64().to_string();
    tokio::fs::create_dir(&folder).await?;
    let last = var.alternatives.last().ok_or(crate::error::Error::Reddit)?;
    let audio_buf = generate_buffer(&url, last.uri.as_ref().unwrap()).await?;
    tokio::fs::write(format!("{}/audio", folder), audio_buf).await?;
    let video_buf = generate_buffer(&url, &var.uri).await?;
    tokio::fs::write(format!("{}/video", folder), video_buf).await?;
    convert_audio_and_video_to_mp4(&folder).await?;
    Ok((folder, caption))
}
