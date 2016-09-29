// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with
// this file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::process::ChildStdout;
use std::io::Read;
use std::sync::mpsc::Sender;

use error::Result;

const BUF_LEN: usize = 42;

/// An asynchronous reader, that reads from a spawned command stdout
/// and sends it to a channel
pub struct AsyncReader {
    stdout: ChildStdout,
    sender: Sender<Result<String>>,

}

impl AsyncReader {
    /// Create a new AsyncReader
    pub fn new(stdout: ChildStdout, sender: Sender<Result<String>>) -> AsyncReader {
        AsyncReader {
            stdout: stdout,
            sender: sender,
        }
    }
    
    /// Reads the output from ispell and sends it over the channel
    pub fn read_loop(&mut self)  {
        loop {
            let result = self.read();
            self.sender.send(result).expect("!!!");
        }
    }

    /// Reads a string
    fn read(&mut self) -> Result<String> {
        let mut buffer = [0; BUF_LEN];
        let mut output = vec!();
        loop {
            let n = try!(self.stdout.read(&mut buffer));
            output.extend_from_slice(&buffer[0..n]);
            if n < BUF_LEN {
                break;
            } else {
                continue;
            }
        }
        let s = try!(String::from_utf8(output));
        Ok(s)
    }
}
