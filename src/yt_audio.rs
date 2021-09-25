use rustube::{Id, VideoFetcher};
use std::error::Error;

use crate::ffmpeg;

pub async fn yt_audio(
    url: &str,
) -> Result<(String, String, Option<String>), Box<dyn Error + Sync + Send>> {
    let new_url = url.trim_end().trim_start();
    let id = Id::from_raw(new_url)?;
    let descrambler = VideoFetcher::from_id(id.into_owned())?.fetch().await?;
    let video = descrambler.descramble()?;
    let stream = video.best_quality().ok_or(crate::error::Error::YtAudio)?;
    let url = stream.signature_cipher.url.as_str();
    let resp = reqwest::get(url).await?;
    let bytes = resp.bytes().await?;
    let title = video.title().to_owned();
    let file_name = ffmpeg::convert_to_mp3(&video.video_details().author, &bytes[..]).await?;
    let mut thumb = None;
    if video.video_details().thumbnails.len() != 0 {
        let resp = reqwest::get(video.video_details().thumbnails[0].url.as_str()).await?;
        let bytes = resp.bytes().await?;
        let title = ffmpeg::convert_to_jpeg(&bytes).await?;
        thumb = Some(title);
    }
    Ok((file_name, title, thumb))
}
