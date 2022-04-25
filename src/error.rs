use std::{error, fmt, io, net, result};

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    NoBrokers,
    NotConnected,
    ParseAddress(net::AddrParseError),
    IO(io::Error),
}

impl From<net::AddrParseError> for Error {
    fn from(err: net::AddrParseError) -> Error {
        Error::ParseAddress(err)
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::IO(err)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::NoBrokers => write!(f, "please specify at least one broker"),
            Error::NotConnected => write!(f, "not connected"),
            Error::ParseAddress(e) => e.fmt(f),
            Error::IO(e) => e.fmt(f),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Error::NoBrokers |
            Error::NotConnected => None,
            Error::ParseAddress(e) => Some(e),
            Error::IO(e) => Some(e),
        }
    }
}
