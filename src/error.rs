use std::fmt::{Display, Formatter, Debug};

pub enum Error {
    Shorts,
    YtDownload,
    Xkcd,
}

impl Debug for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Shorts => write!(f, "shorts error"),
            Error::YtDownload => write!(f, "yt_download error"),
            Error::Xkcd => write!(f, "xkcd error"),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for Error {}