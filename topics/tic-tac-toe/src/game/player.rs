#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Player {
    Human,
    AI,
}

impl Player {
    #[allow(dead_code)]
    pub fn opposite(self) -> Self {
        match self {
            Player::Human => Player::AI,
            Player::AI => Player::Human,
        }
    }
    
    pub fn symbol(self) -> char {
        match self {
            Player::Human => 'X',
            Player::AI => 'O',
        }
    }
}
