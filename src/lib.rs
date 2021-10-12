pub mod error;
pub mod escape;
pub mod ffmpeg;
pub mod gpt;
pub mod nhentai;
pub mod reddit;
pub mod shorts;
pub mod statics;
pub mod urban;
pub mod xkcd;
pub mod yt_audio;
pub mod yt_download;

use teloxide::utils::command::BotCommand;

#[derive(BotCommand)]
#[command(rename = "lowercase", description = "These commands are supported:")]
pub enum Command {
    #[command(description = "displays this text.")]
    Help,
    #[command(description = "returns a youtube shorts video.")]
    Shorts,
    #[command(description = "returns a video with audio of a youtube url")]
    YtDownload(String),
    #[command(description = "returns only audio of a youtube url")]
    YtAudio(String),
    #[command(description = "returns a xkcd comic")]
    Xkcd(String),
    #[command(description = "returns a random nhentai")]
    Nhentai(String),
    #[command(description = "returns a random post from some subreddit")]
    Reddit(String),
    #[command(description = "returns a random definition of a word")]
    Urban(String),
    #[command(description = "returns autogenerated text from a sentence")]
    Gpt(String),
}
