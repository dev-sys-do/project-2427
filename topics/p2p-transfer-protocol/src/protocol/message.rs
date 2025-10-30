use log::error;
use std::{fmt::Display, str::FromStr};

#[derive(Debug, PartialEq, Eq)]
pub enum Message {
    Hello { file_size: u64 },
    Ack,
    Nack,
    Send,
}

// Wire format
impl Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Message::Hello { file_size } => writeln!(f, "HELLO {file_size}"),
            Message::Ack => writeln!(f, "ACK"),
            Message::Nack => writeln!(f, "NACK"),
            Message::Send => writeln!(f, "SEND"),
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
            if parts.len() == 2
                && let Ok(file_size) = parts[1].parse::<u64>()
            {
                return Ok(Message::Hello { file_size });
            }
            error!("Invalid HELLO message");
            return Err(());
        };

        // No arguments msgs
        match upper.as_str() {
            "ACK" => Ok(Message::Ack),
            "NACK" => Ok(Message::Nack),
            "SEND" => Ok(Message::Send),
            // Unknown message
            _ => Err(()),
        }
    }
}
