use crate::escape::escape;
use crate::statics::{RAND_GEN, RE_SHORTS, SHORTS_CLIENT};
use rand::RngCore;
use rustube::{Id, IdBuf, VideoFetcher};
use std::error::Error;

pub async fn shorts() -> Result<(String, String), Box<dyn Error + Sync + Send>> {
    let resp = SHORTS_CLIENT
        .get("https://www.youtube.com/hashtag/shorts")
        .send()
        .await?;
    let s = resp.text().await?;
    let c = RE_SHORTS.captures(&s).ok_or(crate::error::Error::Shorts)?;
    let m = c.get(1).ok_or(crate::error::Error::Shorts)?;
    let v: serde_json::Value = serde_json::from_str(m.as_str())?;
    let videos = &v["contents"];
    let r = RAND_GEN.lock().await.next_u64() as usize
        % videos.as_array().ok_or(crate::error::Error::Shorts)?.len();
    let random_video = &videos[r]["richItemRenderer"]["content"]["videoRenderer"];
    let id = random_video["videoId"]
        .as_str()
        .ok_or(crate::error::Error::Shorts)?
        .to_owned();
    let descrambler = VideoFetcher::from_id(IdBuf::from_string(id)?)?
        .fetch()
        .await?;
    let video = descrambler.descramble()?;
    let path_to_video = video.best_quality().ok_or(crate::error::Error::Shorts)?;
    let url = &path_to_video.signature_cipher.url;
    let title = video.title();
    let channel = &random_video["ownerText"]["runs"][0]["text"];
    let channel_url = format!(
        "https://www.youtube.com{}",
        random_video["ownerText"]["runs"][0]["navigationEndpoint"]["commandMetadata"]
            ["webCommandMetadata"]["url"]
            .as_str()
            .ok_or(crate::error::Error::Shorts)?
    );
    let view_count = video.video_details().view_count;
    let caption = format!(
        "*{}*\n`View count: {}`\nSource: [{}]({})\n",
        escape(
            title
        ),
        view_count,
        escape(
            channel
                .as_str()
                .ok_or(crate::error::Error::Shorts)?
        ),
        channel_url
    );
    Ok((url.as_str().to_owned(), caption))
}
