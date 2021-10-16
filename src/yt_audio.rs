use rustube::{Id, VideoFetcher};
use std::error::Error;

use crate::{ffmpeg, statics::SHORTS_CLIENT};

pub struct AudioFile {
    pub file: String,
    pub title: String,
    pub thumb: Option<String>,
}

pub async fn yt_audio(url: &str) -> Result<AudioFile, Box<dyn Error + Sync + Send>> {
    let new_url = url.trim_end().trim_start();
    let id = Id::from_raw(new_url)?;
    let descrambler = VideoFetcher::from_id(id.into_owned())?.fetch().await?;
    let video = descrambler.descramble()?;
    let stream = video.best_quality().ok_or(crate::error::Error::YtAudio)?;
    let url = stream.signature_cipher.url.as_str();
    let bytes = SHORTS_CLIENT.get(url).send().await?.bytes().await?;
    let title = video.title().to_owned();
    let file = ffmpeg::convert_to_mp3(&video.video_details().author, &bytes[..]).await?;
    let mut thumb = None;
    if !video.video_details().thumbnails.is_empty() {
        let resp = reqwest::get(video.video_details().thumbnails[0].url.as_str()).await?;
        let bytes = resp.bytes().await?;
        let title = ffmpeg::convert_to_jpeg(&bytes).await?;
        thumb = Some(title);
    }
    Ok(AudioFile { file, title, thumb })
}
