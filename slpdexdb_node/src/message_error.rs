use std::io;
use std::error::Error;
use std::fmt::{Display, Formatter};


#[derive(Debug)]
pub enum MessageError {
    WrongMagic,
    InvalidChecksum,
    IoError(io::Error),
}

impl From<io::Error> for MessageError {
    fn from(err: io::Error) -> Self {
        MessageError::IoError(err)
    }
}

impl From<MessageError> for io::Error {
    fn from(err: MessageError) -> Self {
        use self::MessageError::*;
        match err {
            IoError(err) => err,
            WrongMagic => io::Error::new(io::ErrorKind::InvalidData, "Wrong magic"),
            InvalidChecksum => io::Error::new(io::ErrorKind::InvalidData, "Invalid checksum"),
        }
    }
}

impl Display for MessageError {
    fn fmt<'a>(&self, f: &mut Formatter<'a>) -> Result<(), std::fmt::Error> {
        write!(f, "{:#?}", self)?;
        Ok(())
    }
}

impl Error for MessageError {

}
