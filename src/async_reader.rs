// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with
// this file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::process::ChildStdout;
use std::io::{BufReader, BufRead};
use std::sync::mpsc::Sender;

use error::Result;


/// An asynchronous reader, that reads from a spawned command stdout
/// and sends it to a channel
pub struct AsyncReader {
    stdout: BufReader<ChildStdout>,
    sender: Sender<Result<String>>,
}

impl AsyncReader {
    /// Create a new AsyncReader
    pub fn new(stdout: ChildStdout, sender: Sender<Result<String>>) -> AsyncReader {
        AsyncReader {
            stdout: BufReader::new(stdout),
            sender: sender,
        }
    }

    /// Reads the output from ispell and sends it over the channel
    pub fn read_loop(&mut self) {
        loop {
            let result = self.read();
            match self.sender.send(result) {
                Ok(_) => (),
                Err(_) => break, // main process was aborted
            }
        }
    }

    /// Reads a string
    fn read(&mut self) -> Result<String> {
        let mut output = String::new();
        loop {
            try!(self.stdout.read_line(&mut output));
            if output.ends_with("\n\n") || output == "\n" || output.starts_with("@") {
                break;
            }
        }
        Ok(output)
    }
}

