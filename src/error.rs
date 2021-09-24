use std::fmt::{Display, Formatter, Debug};

pub enum Error {
    Shorts,
    YtDownload,
    YtAudio,
    Xkcd,
    Ffmpeg,
    Hentai(hentai::HentaiError),
}

impl Debug for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Shorts => write!(f, "shorts error"),
            Error::YtDownload => write!(f, "yt_download error"),
            Error::YtAudio => write!(f, "yt_audio error"),
            Error::Xkcd => write!(f, "xkcd error"),
            Error::Ffmpeg => write!(f, "ffmpeg error"),
            Error::Hentai(e) => write!(f, "hentai error: {}", e),
        }
    }
}

impl From<hentai::HentaiError> for Error {
    fn from(e: hentai::HentaiError) -> Self {
        Error::Hentai(e)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for Error {}