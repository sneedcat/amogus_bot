use crate::escape::escape;
use crate::statics::{RAND_GEN, RE_SHORTS, SHORTS_CLIENT};
use rand::RngCore;
use rustube::{IdBuf, VideoFetcher};
use std::error::Error;

pub struct Short {
    pub file_name: String,
    pub caption: String,
}

pub async fn shorts() -> Result<Short, Box<dyn Error + Sync + Send>> {
    let resp = SHORTS_CLIENT
        .get("https://www.youtube.com/hashtag/shorts")
        .send()
        .await?;
    let text = resp.text().await?;
    let capture = RE_SHORTS
        .captures(&text)
        .ok_or(crate::error::Error::Shorts)?;
    let mat = capture.get(1).ok_or(crate::error::Error::Shorts)?;
    let content: serde_json::Value = serde_json::from_str(mat.as_str())?;
    let videos = &content["contents"];
    let random_number = RAND_GEN.lock().await.next_u64() as usize
        % videos.as_array().ok_or(crate::error::Error::Shorts)?.len();
    let random_video = &videos[random_number]["richItemRenderer"]["content"]["videoRenderer"];
    let id = random_video["videoId"]
        .as_str()
        .ok_or(crate::error::Error::Shorts)?
        .to_owned();
    let descrambler = VideoFetcher::from_id(IdBuf::from_string(id)?)?
        .fetch()
        .await?;
    let video = descrambler.descramble()?;
    let stream = video
        .streams()
        .iter()
        .filter(|stream| stream.includes_video_track && stream.includes_audio_track)
        .filter(|stream| {
            stream.quality_label.is_some()
                && stream.quality_label.unwrap()
                    <= rustube::video_info::player_response::streaming_data::QualityLabel::P480
        })
        .max_by_key(|stream| stream.quality_label)
        .ok_or(crate::error::Error::Shorts)?;
    let url = stream.signature_cipher.url.clone();
    let title = video.title();
    let channel = &video.video_details().author;
    let channel_url = format!(
        "https://www.youtube.com/channel/{}",
        video.video_details().channel_id
    );
    let view_count = video.video_details().view_count;
    let caption = format!(
        "*{}*\n`View count: {}`\nSource: [{}]({})\n",
        escape(title),
        view_count,
        escape(channel),
        channel_url
    );
    let file_name = format!("{}.mp4", RAND_GEN.lock().await.next_u64());
    let bytes = SHORTS_CLIENT.get(url).send().await?.bytes().await?;
    tokio::fs::write(&file_name, bytes).await?;
    Ok(Short { file_name, caption })
}
