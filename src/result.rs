/*! result type
 *
 */
use jsonwebtoken::errors::Error as JwtError;
use serde_yaml;
use std::convert::From;
use std::fmt;
use std::io;
use std::str::Utf8Error;

/// 错误种类
#[derive(Debug)]
pub enum ErrorKind {
    /// No data available
    NoDataAvailable,
    /// A UTF-8 error
    Utf8Error(Utf8Error),
    /// An input/output error
    IoError(io::Error),
    /// serde yaml file error
    SerdeYamlError(serde_yaml::Error),
    // jwt token
    JwtError(JwtError),
    /// Other error from higher-level crate, for downcasting
    Other(Box<dyn std::error::Error + Send + Sync + 'static>),
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str("ErrorKind: ")?;
        match self {
            ErrorKind::NoDataAvailable => fmt.write_str("No data available"),
            ErrorKind::IoError(_) => fmt.write_str("I/O failure"),
            ErrorKind::Utf8Error(_) => fmt.write_str("UTF-8 failure"),
            ErrorKind::SerdeYamlError(x) => fmt.write_str(format!("{:?}", x).as_str()),
            ErrorKind::JwtError(x) => fmt.write_str(format!("{:?}", x).as_str()),
            ErrorKind::Other(x) => x.fmt(fmt),
        }
    }
}

impl std::error::Error for ErrorKind {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            ErrorKind::IoError(ref error) => Some(error),
            ErrorKind::Utf8Error(ref error) => Some(error),
            ErrorKind::SerdeYamlError(ref error) => Some(error),
            ErrorKind::Other(ref error) => error.source(),
            _ => None,
        }
    }
}

impl From<serde_yaml::Error> for ErrorKind {
    fn from(err: serde_yaml::Error) -> ErrorKind {
        ErrorKind::SerdeYamlError(err)
    }
}

impl From<JwtError> for ErrorKind {
    fn from(err: JwtError) -> ErrorKind {
        ErrorKind::JwtError(err)
    }
}

impl From<io::Error> for ErrorKind {
    fn from(err: io::Error) -> ErrorKind {
        if err.kind() == io::ErrorKind::UnexpectedEof {
            return ErrorKind::NoDataAvailable;
        }
        ErrorKind::IoError(err)
    }
}

impl From<Utf8Error> for ErrorKind {
    fn from(err: Utf8Error) -> ErrorKind {
        ErrorKind::Utf8Error(err)
    }
}
