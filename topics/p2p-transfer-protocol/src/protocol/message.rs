use log::error;
use std::{fmt::Display, str::FromStr};

#[derive(Debug, PartialEq, Eq)]
pub enum Message {
    Hello { file_size: u64 },
    ACK,
    NACK,
    Send,
}

// Wire format
impl Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Message::Hello { file_size } => write!(f, "HELLO {file_size}\n"),
            Message::ACK => write!(f, "ACK\n"),
            Message::NACK => write!(f, "NACK\n"),
            Message::Send => write!(f, "SEND\n"),
        }
    }
}

impl FromStr for Message {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Normalize
        let s = s.trim();
        let upper = s.to_ascii_uppercase();

        if upper.starts_with("HELLO") {
            let parts: Vec<&str> = s.split_whitespace().collect();
            if parts.len() == 2 {
                if let Ok(file_size) = parts[1].parse::<u64>() {
                    return Ok(Message::Hello { file_size });
                }
            }
            error!("Invalid HELLO message");
            return Err(());
        };

        // No arguments msgs
        return match upper.as_str() {
            "ACK" => Ok(Message::ACK),
            "NACK" => Ok(Message::NACK),
            "SEND" => Ok(Message::Send),
            // Unknown message
            _ => Err(()),
        };
    }
}
