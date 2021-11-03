use crate::{escape::escape, statics::RAND_GEN, statics::SHORTS_CLIENT};
use rand::RngCore;
use rustube::{Id, VideoFetcher};
use std::error::Error;

pub struct Video {
    pub file_name: String,
    pub caption: String,
}

pub async fn yt_download(url: &str) -> Result<Video, Box<dyn Error + Sync + Send>> {
    let new_url = url.trim_end().trim_start();
    let id = Id::from_raw(new_url)?;
    let descrambler = VideoFetcher::from_id(id.into_owned())?.fetch().await?;
    let video = descrambler.descramble()?;
    let quality = video
        .streams()
        .iter()
        .filter(|stream| stream.includes_video_track && stream.includes_audio_track)
        .filter(|stream| {
            stream.quality_label.is_some()
                && stream.quality_label.unwrap()
                    < rustube::video_info::player_response::streaming_data::QualityLabel::P480
        })
        .max_by_key(|stream| stream.quality_label)
        .ok_or(crate::error::Error::YtDownload)?;
    let url = quality.signature_cipher.url.as_str();
    let title = video.title();
    let view_count = video.video_details().view_count;
    let file_name = format!("{}.mp4", RAND_GEN.lock().await.next_u64());
    let bytes = SHORTS_CLIENT.get(url).send().await?.bytes().await?;
    tokio::fs::write(&file_name, bytes).await?;
    let caption = format!("*{}*\n`View count: {}`", escape(title), view_count);
    Ok(Video { file_name, caption })
}
