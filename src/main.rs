mod error;
mod escape;
mod ffmpeg;
mod nhentai;
mod reddit;
mod shorts;
mod statics;
mod xkcd;
mod yt_audio;
mod yt_download;

use std::error::Error;
use teloxide::payloads::{SendAudioSetters, SendPhotoSetters};
use teloxide::types::{InputFile, ParseMode};
use teloxide::{prelude::*, utils::command::BotCommand};

use crate::reddit::Content;

#[derive(BotCommand)]
#[command(rename = "lowercase", description = "These commands are supported:")]
enum Command {
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
}

async fn answer(
    cx: UpdateWithCx<AutoSend<Bot>, Message>,
    command: Command,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    match command {
        Command::Help => cx.answer(Command::descriptions()).await?,
        Command::YtDownload(s) => {
            let (d_url, caption) = yt_download::yt_download(&s).await?;
            let input_file = InputFile::Url(d_url);
            cx.requester
                .send_video(cx.update.chat.id, input_file)
                .caption(caption)
                .parse_mode(ParseMode::MarkdownV2)
                .await?
        }
        Command::YtAudio(s) => {
            let (file_name, title, thumb) = yt_audio::yt_audio(&s).await?;
            let input_file = InputFile::File((&file_name).into());
            if let Some(thumb) = thumb {
                let file = InputFile::File((&thumb).into());
                cx.requester
                    .send_audio(cx.update.chat.id, input_file)
                    .title(title)
                    .thumb(file)
                    .await?;
                tokio::fs::remove_file(thumb).await?;
            } else {
                cx.requester
                    .send_audio(cx.update.chat.id, input_file)
                    .title(title)
                    .await?;
            }
            tokio::fs::remove_file(file_name).await?;
            cx.update
        }
        Command::Shorts => {
            let (d_url, caption) = shorts::shorts().await?;
            let input_file = InputFile::Url(d_url);
            cx.requester
                .send_video(cx.update.chat.id, input_file)
                .caption(caption)
                .parse_mode(ParseMode::MarkdownV2)
                .await?
        }
        Command::Xkcd(s) => {
            let (d_url, caption) = xkcd::xkcd(s).await?;
            let input_file = InputFile::Url(d_url);
            cx.requester
                .send_photo(cx.update.chat.id, input_file)
                .caption(caption)
                .parse_mode(ParseMode::MarkdownV2)
                .await?
        }
        Command::Nhentai(s) => {
            let (d_url, caption) = nhentai::nhentai(s).await?;
            let input_file = InputFile::Url(d_url);
            cx.requester
                .send_photo(cx.update.chat.id, input_file)
                .caption(caption)
                .parse_mode(ParseMode::MarkdownV2)
                .await?
        }
        Command::Reddit(s) => {
            let content = reddit::reddit(&s).await?;
            match content {
                Content::Image(image, caption) => {
                    let input_file = InputFile::File(image.clone().into());
                    cx.requester
                        .send_photo(cx.update.chat_id(), input_file)
                        .caption(caption)
                        .parse_mode(ParseMode::MarkdownV2)
                        .await?;
                    tokio::fs::remove_file(&image).await?;
                }
                Content::Video(folder, caption) => {
                    let file_name = format!("{}/output.mp4", folder);
                    let input_file = InputFile::File(file_name.into());
                    cx.requester
                        .send_video(cx.update.chat_id(), input_file)
                        .caption(caption)
                        .parse_mode(ParseMode::MarkdownV2)
                        .await?;
                    tokio::fs::remove_dir_all(&folder).await?;
                }
                Content::Text(text) => {
                    cx.requester
                        .send_message(cx.update.chat_id(), text)
                        .parse_mode(ParseMode::MarkdownV2)
                        .await?;
                }
            }
            cx.update
        }
    };

    Ok(())
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    teloxide::enable_logging!();
    log::info!("Starting simple_commands_bot...");
    let bot = Bot::from_env().auto_send();
    let bot_name = bot.get_me().await.unwrap().user.username.unwrap();
    teloxide::commands_repl(bot, bot_name, answer).await;
}
