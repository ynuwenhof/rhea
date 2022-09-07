use std::{io, result};
use tokio::task::JoinError;

pub type Result<T> = result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error(transparent)]
    Join(#[from] JoinError),
    #[error(transparent)]
    Command(#[from] CommandError),
}

#[derive(thiserror::Error, Debug)]
pub enum CommandError {}
