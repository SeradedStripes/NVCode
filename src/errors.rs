use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum NvError {
    NotImplemented(String),
    Io(std::io::Error),
}

impl fmt::Display for NvError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NvError::NotImplemented(feature) => {
                write!(f, "feature not implemented: {feature}")
            }
            NvError::Io(err) => write!(f, "io error: {err}"),
        }
    }
}

impl Error for NvError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            NvError::NotImplemented(_) => None,
            NvError::Io(err) => Some(err),
        }
    }
}

impl From<std::io::Error> for NvError {
    fn from(value: std::io::Error) -> Self {
        NvError::Io(value)
    }
}
