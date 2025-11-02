pub type Position = u8;

pub type Grid = [Option<PlayerID>; 9];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlayerID {
    Player1,
    Player2,
}

pub type Result<T> = std::result::Result<T, Error>;
pub enum Error {
    Other(String),
    InvalidInput,
}
