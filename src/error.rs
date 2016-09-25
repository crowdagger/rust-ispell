// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with
// this file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::error;
use std::result;
use std::fmt;
use std::string::FromUtf8Error;
use std::io;

/// Internal ErrorType
#[derive(Debug, PartialEq)]
enum ErrorType {
    /// Default
    Default,
    Process,
    Utf8,
    Protocol,
}

/// Ispell Result type
pub type Result<T> = result::Result<T, Error>;

#[derive(Debug, PartialEq)]
/// Ispell Error type
pub struct Error {
    msg: String,
    variant: ErrorType
}

impl Error {
    /// Creates a new default error
    pub fn new<S: Into<String>>(msg: S) -> Error {
        Error {
            msg: msg.into(),
            variant: ErrorType::Default,
        }
    }

    /// Create a new process error
    ///
    /// (for errors launching Ispell)
    pub fn process<S: Into<String>>(msg: S) -> Error {
        Error {
            msg: msg.into(),
            variant: ErrorType::Process
        }
    }

    /// Create a new UTF8 error
    ///
    /// (for errors converting to UTF8)
    pub fn utf8<S: Into<String>>(msg: S) -> Error {
        Error {
            msg: msg.into(),
            variant: ErrorType::Utf8,
        }
    }

    /// Creates a new protocol error
    ///
    /// (when we didn't understand ispell output)
    pub fn protocol<S: Into<String>>(msg: S) -> Error {
        Error {
            msg: msg.into(),
            variant: ErrorType::Protocol,
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        &self.msg
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl From<FromUtf8Error> for Error {
    fn from(err: FromUtf8Error) -> Error {
        Error::utf8(format!("error decoding ispell output to utf8: {}", err))
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::process(format!("error while reading/writing to ispell: {}", err))
    }
}
