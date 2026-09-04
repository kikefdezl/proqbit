use std::result::Result as StdResult;

use thiserror::Error;

use crate::client::ClientError;

#[derive(Debug, Error)]
pub enum ProqbitError {
    #[error("Client error: {0}")]
    Client(#[from] ClientError),

    #[error("Notify error: {0}")]
    Notify(#[from] notify::Error),

    #[error("NotifyRust error: {0}")]
    NotifyRust(#[from] notify_rust::error::Error),

    #[error("Figment error: {0}")]
    Figment(#[from] Box<figment::Error>),
}

impl From<figment::Error> for ProqbitError {
    fn from(e: figment::Error) -> Self {
        ProqbitError::Figment(Box::new(e))
    }
}

pub type Result<T> = StdResult<T, ProqbitError>;
