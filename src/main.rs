use std::error::Error;
use teloxide::payloads::{SendAudioSetters, SendMessageSetters, SendPhotoSetters};
use teloxide::types::{InputFile, ParseMode};
use teloxide::{prelude::*, utils::command::BotCommand};

use amogus_bot::Command;

async fn answer(
    cx: UpdateWithCx<AutoSend<Bot>, Message>,
    command: Command,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    match command {
        Command::Help => cx.answer(Command::descriptions()).await?,
        Command::YtDownload(s) => {
            let (d_url, caption) = amogus_bot::yt_download::yt_download(&s).await?;
            let input_file = InputFile::Url(d_url);
            cx.requester
                .send_video(cx.update.chat.id, input_file)
                .caption(caption)
                .parse_mode(ParseMode::MarkdownV2)
                .await?
        }
        Command::YtAudio(s) => {
            let (file_name, title, thumb) = amogus_bot::yt_audio::yt_audio(&s).await?;
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
            let (d_url, caption) = amogus_bot::shorts::shorts().await?;
            let input_file = InputFile::Url(d_url);
            cx.requester
                .send_video(cx.update.chat.id, input_file)
                .caption(caption)
                .parse_mode(ParseMode::MarkdownV2)
                .await?
        }
        Command::Xkcd(s) => {
            let (d_url, caption) = amogus_bot::xkcd::xkcd(s).await?;
            let input_file = InputFile::Url(d_url);
            cx.requester
                .send_photo(cx.update.chat.id, input_file)
                .caption(caption)
                .parse_mode(ParseMode::MarkdownV2)
                .await?
        }
        Command::Nhentai(s) => {
            let (d_url, caption) = amogus_bot::nhentai::nhentai(s).await?;
            let input_file = InputFile::Url(d_url);
            cx.requester
                .send_photo(cx.update.chat.id, input_file)
                .caption(caption)
                .parse_mode(ParseMode::MarkdownV2)
                .await?
        }
        Command::Reddit(s) => {
            let content = amogus_bot::reddit::reddit(&s).await?;
            use amogus_bot::reddit::Content;
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
        Command::Urban(text) => {
            let caption = amogus_bot::urban::urban(text).await?;
            cx.requester
                .send_message(cx.update.chat_id(), caption)
                .parse_mode(ParseMode::MarkdownV2)
                .await?
        }
        Command::Gpt(text) => {
            let caption = amogus_bot::gpt::gpt(text).await?;
            cx.requester
                .send_message(cx.update.chat_id(), caption)
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
