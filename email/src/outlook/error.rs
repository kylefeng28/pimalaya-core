use std::{error, fmt, io, result};

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    MissingAccessToken,
    BuildAccessTokenError(Box<dyn error::Error + Send + Sync>),
    GraphApiError(graph_rs_sdk::GraphFailure),
    IoError(io::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::MissingAccessToken => write!(f, "missing access token"),
            Self::BuildAccessTokenError(err) => {
                write!(f, "cannot build access token: {err}")
            }
            Self::GraphApiError(err) => write!(f, "graph api error: {err}"),
            Self::IoError(err) => write!(f, "io error: {err}"),
        }
    }
}

impl error::Error for Error {}

impl crate::AnyError for Error {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl From<Error> for crate::AnyBoxedError {
    fn from(err: Error) -> Self {
        Box::new(err)
    }
}

impl From<graph_rs_sdk::GraphFailure> for Error {
    fn from(err: graph_rs_sdk::GraphFailure) -> Self {
        Self::GraphApiError(err)
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Self::IoError(err)
    }
}
