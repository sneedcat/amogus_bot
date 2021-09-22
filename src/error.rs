use std::fmt::{Display, Formatter, Debug};

pub enum Error {
    Shorts,
    YtDownload
}

impl Debug for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Shorts => write!(f, "shorts error"),
            YtDownload => write!(f, "ytdownload error"),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for Error {}