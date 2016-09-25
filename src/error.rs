// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with
// this file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::error;
use std::result;
use std::fmt;

/// Internal ErrorType
#[derive(Debug, PartialEq)]
enum ErrorType {
    /// Default
    Default,
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


