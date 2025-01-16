use std::error::Error as StdError;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug)]
pub enum Error {
    // sync errors
    SyncRecvError(std::sync::mpsc::RecvError),
    SyncTryRecvError(std::sync::mpsc::TryRecvError),
    SyncSendError(Box<dyn StdError>),
    SyncThreadError(std::io::Error),
    SyncError(String),
    // misc
    UnknownError(Box<dyn StdError>),
}

impl Error {
    pub fn as_sync_send_error<E>(err: E) -> Self
    where
        E: StdError + 'static,
    {
        Self::SyncSendError(Box::new(err))
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::SyncError(err) => write!(f, "sync error: {}", err),
            Error::SyncRecvError(err) => write!(f, "sync recv error: {}", err),
            Error::SyncTryRecvError(err) => write!(f, "sync try recv error: {}", err),
            Error::SyncSendError(err) => write!(f, "sync send error: {}", err),
            Error::SyncThreadError(err) => write!(f, "sync thread error: {}", err),
            Error::UnknownError(err) => write!(f, "unknown error: {}", err),
        }
    }
}

impl StdError for Error {}
