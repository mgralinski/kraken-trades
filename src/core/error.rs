use super::requests::ReqError;

#[derive(Debug)]
pub enum Error {
    NoCredentials,
    Request(ReqError),
    SerdeParse(serde_json::error::Error),
    ChronoParse(chrono::ParseError),
    IoError(std::io::Error),
    Csv(csv::Error),
}

impl From<ReqError> for Error {
    fn from(src: ReqError) -> Self {
        Self::Request(src)
    }
}

impl From<serde_json::error::Error> for Error {
    fn from(src: serde_json::error::Error) -> Self {
        Self::SerdeParse(src)
    }
}

impl From<chrono::ParseError> for Error {
    fn from(src: chrono::ParseError) -> Self {
        Self::ChronoParse(src)
    }
}

impl From<std::io::Error> for Error {
    fn from(src: std::io::Error) -> Self {
        Self::IoError(src)
    }
}

impl From<csv::Error> for Error {
    fn from(src: csv::Error) -> Self {
        Self::Csv(src)
    }
}
