use std::error::Error;

use m3u8_rs::playlist::{Playlist, VariantStream};
use rand::RngCore;
use reqwest::Response;
use roux::Subreddit;

pub enum Content {
    Image(String, String),
    Video(String, String),
    Text(String),
}

use crate::{
    escape::escape,
    ffmpeg::{convert_audio_and_video_to_mp4, convert_to_jpeg},
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
    if p.segments.is_empty() {
        return Err(Box::new(crate::error::Error::Reddit));
    }
    let uri = &p.segments[0].uri;
    let resp = make_request(url, uri).await?;
    let bytes = resp.bytes().await?;
    buf.extend_from_slice(&bytes[..]);
    Ok(buf)
}

pub async fn reddit(s: &str) -> Result<Content, Box<dyn Error + Send + Sync>> {
    let subreddit = Subreddit::new(s);
    let hot = subreddit.hot(35, None).await?;
    let arr: Vec<_> = hot
        .data
        .children
        .into_iter()
        .filter(|post| post.data.url.is_some())
        .collect();
    if arr.is_empty() {
        return Err(Box::new(crate::error::Error::Reddit));
    }
    let n = RAND_GEN.lock().await.next_u64() as usize % arr.len();
    let post = &arr[n];
    println!("{:?}", post.data.selftext);
    if !post.data.selftext.is_empty() {
        let caption = format!(
            "*{}*\n{}\nScore: {}\n[Number of comments:{}](reddit.com{})",
            escape(&post.data.title),
            escape(&post.data.selftext),
            post.data.score,
            post.data.num_comments,
            post.data.permalink
        );
        Ok(Content::Text(caption))
    } else if post.data.domain == "v.redd.it" {
        let caption = format!(
            "*{}*\nScore: {}\n[Number of comments: {}](https://reddit.com{})",
            escape(&post.data.title),
            post.data.score,
            post.data.num_comments,
            post.data.permalink
        );
        let url = post.data.url.as_ref().unwrap();
        let resp = make_request(url, "HLSPlaylist.m3u8").await?;
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
                        Err(_) => Some(v),
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
        let audio_buf = generate_buffer(url, last.uri.as_ref().unwrap()).await?;
        tokio::fs::write(format!("{}/audio", folder), audio_buf).await?;
        let video_buf = generate_buffer(url, &var.uri).await?;
        tokio::fs::write(format!("{}/video", folder), video_buf).await?;
        convert_audio_and_video_to_mp4(&folder).await?;
        Ok(Content::Video(folder, caption))
    } else if true {
        let caption = format!(
            "*{}*\nScore: {}\n[Number of comments: {}](https://reddit.com{})",
            escape(&post.data.title),
            post.data.score,
            post.data.num_comments,
            post.data.permalink
        );
        let resp = CLIENT.get(post.data.url.as_ref().unwrap()).send().await?;
        let file = resp.bytes().await?;
        let title = convert_to_jpeg(&file[..]).await?;
        Ok(Content::Image(title, caption))
    } else {
        let caption = format!(
            "*{}*\n[URL]({})\nScore: {}\n[Number of comments: {}](https://reddit.com{})",
            escape(&post.data.title),
            post.data.url.as_ref().unwrap(),
            post.data.score,
            post.data.num_comments,
            post.data.permalink
        );
        Ok(Content::Text(caption))
    }
}
