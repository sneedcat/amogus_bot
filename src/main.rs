mod error;
mod escape;
mod shorts;
mod statics;
mod xkcd;
mod yt_download;

use std::error::Error;
use teloxide::payloads::SendPhotoSetters;
use teloxide::types::{InputFile, ParseMode};
use teloxide::{prelude::*, utils::command::BotCommand};

#[derive(BotCommand)]
#[command(rename = "lowercase", description = "These commands are supported:")]
enum Command {
    #[command(description = "displays this text.")]
    Help,
    #[command(description = "returns a youtube shorts video.")]
    Shorts,
    #[command(description = "downloads a video")]
    YtDownload(String),
    #[command(description = "grabs a xkcd image")]
    Xkcd(String),
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