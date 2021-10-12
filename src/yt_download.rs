use crate::escape::escape;
use rustube::{Id, VideoFetcher};
use std::error::Error;

pub async fn yt_download(url: &str) -> Result<(String, String), Box<dyn Error + Sync + Send>> {
    let new_url = url.trim_end().trim_start();
    let id = Id::from_raw(new_url)?;

    let descrambler = VideoFetcher::from_id(id.into_owned())?.fetch().await?;
    let video = descrambler.descramble()?;

    let quality = video
        .best_quality()
        .ok_or(crate::error::Error::YtDownload)?;
    let url = quality.signature_cipher.url.as_str();
    let title = video.title();
    let view_count = video.video_details().view_count;
    let caption = format!("*{}*\n`View count: {}`", escape(title), view_count,);
    Ok((url.to_owned(), caption))
}
