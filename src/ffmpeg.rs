use std::error::Error;

use crate::statics::RAND_GEN;
use rand::RngCore;

pub async fn convert_to_mp3(
    artist: &str,
    buffer: &[u8],
) -> Result<String, Box<dyn Error + Send + Sync>> {
    let name = RAND_GEN.lock().await.next_u64();
    let title = format!("{}", name);
    let new_title = format!("{}.mp3", name);
    tokio::fs::write(&title, buffer).await?;
    let mut child = tokio::process::Command::new("ffmpeg")
        .arg("-i")
        .arg(&title)
        .arg("-metadata")
        .arg(format!("artist={}", artist))
        .arg(&new_title)
        .spawn()?;
    let status = child.wait().await?;
    tokio::fs::remove_file(&title).await?;
    if !status.success() {
        return Err(crate::error::Error::Ffmpeg.into());
    }
    Ok(new_title)
}

pub async fn convert_to_jpeg(buffer: &[u8]) -> Result<String, Box<dyn Error + Send + Sync>> {
    let name = RAND_GEN.lock().await.next_u64();
    let title = format!("{}", name);
    let new_title = format!("{}.jpeg", name);
    tokio::fs::write(&title, buffer).await?;
    let mut child = tokio::process::Command::new("ffmpeg")
        .arg("-i")
        .arg(&title)
        .arg("-vf")
        .arg("scale=min'(320,iw)':'min(320,ih)'")
        .arg(&new_title)
        .spawn()?;
    let status = child.wait().await?;
    tokio::fs::remove_file(&title).await?;
    if !status.success() {
        return Err(crate::error::Error::Ffmpeg.into());
    }
    Ok(new_title)
}

pub async fn convert_audio_and_video_to_mp4(
    folder: &str,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut child = tokio::process::Command::new("ffmpeg")
        .arg("-i")
        .arg(format!("{}/video", folder))
        .arg("-i")
        .arg(format!("{}/audio", folder))
        .arg(format!("{}/output.mp4", folder))
        .spawn()?;
    let status = child.wait().await?;
    if !status.success() {
        return Err(crate::error::Error::Ffmpeg.into());
    }
    Ok(())
}
