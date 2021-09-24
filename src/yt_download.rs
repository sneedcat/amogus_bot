use std::error::Error;
use rustube::{Id, VideoFetcher};
use crate::escape::escape;

pub async fn yt_download(url: &str) -> Result<(String, String), Box<dyn Error + Sync + Send>> {
    let id = Id::from_raw(url)?;
    let descrambler = VideoFetcher::from_id(id.into_owned())?
        .fetch()
        .await?;
    let video = descrambler.descramble()?;
    let quality = video.best_quality().ok_or(crate::error::Error::YtDownload)?;
    let url =  quality.signature_cipher.url.as_str();
    let title = video.title();
    let view_count = video.video_details().view_count;
    let caption = format!(
        "*{}*\n`View count: {}`",
        escape(
            title
        ),
        view_count,
    );
    Ok((url.to_owned(), caption))
}
