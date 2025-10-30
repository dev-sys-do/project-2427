use std::{fmt::Display, str::FromStr};

#[cfg(test)]
use strum::EnumIter;

#[derive(Debug, PartialEq, Eq)]
#[cfg_attr(test,derive(EnumIter))]
pub enum Message {
    Hello,
    ACK,
    NACK,
    Send,
}

// Wire format
impl Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Message::Hello => "HELLO",
            Message::ACK => "ACK",
            Message::NACK => "NACK",
            Message::Send => "SEND",
        };
        write!(f, "{}", s)
    }
}

impl FromStr for Message {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_uppercase().as_str() {
            "HELLO" => Ok(Message::Hello),
            "ACK" => Ok(Message::ACK),
            "NACK" => Ok(Message::NACK),
            "SEND" => Ok(Message::Send),
            _ => Err(()),
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize() {
        assert_eq!(Message::Hello.to_string(), "HELLO");
        assert_eq!(Message::ACK.to_string(), "ACK");
        assert_eq!(Message::NACK.to_string(), "NACK");
        assert_eq!(Message::Send.to_string(), "SEND");

        assert_eq!("hello".parse::<Message>(), Ok(Message::Hello));
        assert_eq!("ACK".parse::<Message>(), Ok(Message::ACK));
        assert_eq!("nack".parse::<Message>(), Ok(Message::NACK));
        assert_eq!("SEND".parse::<Message>(), Ok(Message::Send));
        assert!("unknown".parse::<Message>().is_err());
    }


    #[test]
    fn test_deserialize() {
        assert_eq!("hello".parse::<Message>(), Ok(Message::Hello));
        assert_eq!("ACK".parse::<Message>(), Ok(Message::ACK));
        assert_eq!("nack".parse::<Message>(), Ok(Message::NACK));
        assert_eq!("SEND".parse::<Message>(), Ok(Message::Send));
        assert!("unknown".parse::<Message>().is_err());
    }
}