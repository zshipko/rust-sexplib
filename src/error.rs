use std;
use std::result;

#[derive(Debug)]
pub enum Error {
    NotImplemented,
    InvalidType,
    IO(std::io::Error),
    Message(String)
}

pub type Result<T> = result::Result<T, Error>;

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        Error::IO(err)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        f.write_fmt(format_args!("{:?}", self))
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        match self {
            &Error::NotImplemented => "not implemented",
            &Error::InvalidType => "invalid type",
            &Error::IO(ref e) => e.description(),
            &Error::Message(ref m) => m.as_str()
        }
    }
}

