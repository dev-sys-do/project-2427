use thiserror::Error;

pub type Position = u8;

pub type Grid = [Option<PlayerID>; 9];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlayerID {
    Player1,
    Player2,
}

pub type Result<T> = std::result::Result<T, Error>;
#[derive(Debug, Error)]
pub enum Error {
    #[error("{0}")]
    Other(String),
    #[error("Invalid input")]
    InvalidInput,
}
