use std::io::{self, Read, Write};

/// Protocol commands for P2P file transfer
#[derive(Debug, Clone, PartialEq)]
pub enum Command {
    Hello { filename: String, size: u64 }, // had to add filename because output is a directory, and there can be multiple senders for the same listener
    Ack,
    Nack,
    Send { size: u64 },
}

impl Command {
    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            Command::Hello { filename, size } => {
                format!("HELLO {} {}\n", filename, size).into_bytes()
            }
            Command::Ack => {
              format!("ACK\n").into_bytes()
            }
            Command::Nack => {
              format!("NACK\n").into_bytes()
            }
            Command::Send { size } => {
                format!("SEND {}\n", size).into_bytes()
            }
        }
    }

    pub fn parse(command_str: &str) -> Result<Command, String> {
        let s = command_str.trim();
        let parts: Vec<&str> = s.split_whitespace().collect();
        if parts.is_empty() {
            return Err("huh".to_string());
        }
        match parts[0] {
            "HELLO" => {
                if parts.len() != 3 {
                    return Err("HELLO requires filename and size".to_string());
                }
                let filename = parts[1].to_string();
                let size = parts[2]
                    .parse::<u64>()
                    .map_err(|_| "Invalid size in HELLO".to_string())?;
                Ok(Command::Hello { filename, size })
            }
            "ACK" => Ok(Command::Ack),
            "NACK" => Ok(Command::Nack),
            "SEND" => {
                if parts.len() != 2 {
                    return Err("SEND requires size".to_string());
                }
                let size = parts[1]
                    .parse::<u64>()
                    .map_err(|_| "Invalid size in SEND".to_string())?;
                Ok(Command::Send { size })
            }
            _ => Err(format!("Unknown command: {}", parts[0])),
        }
    }

    // utils

    pub fn write_to<W: Write>(&self, stream: &mut W) -> io::Result<()> {
        stream.write_all(&self.to_bytes())?;
        stream.flush()
    }

    pub fn read_from<R: Read>(stream: &mut R) -> io::Result<Command> {
        let mut buffer = Vec::new();
        let mut byte = [0u8; 1];
        // read until newline
        loop {
            stream.read_exact(&mut byte)?;
            if byte[0] == b'\n' {
                // end of command
                break;
            }
            buffer.push(byte[0]);
        }
        let command_str = String::from_utf8(buffer)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        Command::parse(&command_str)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
    }
}