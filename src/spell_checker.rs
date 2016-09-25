// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with
// this file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::process::Child;
use std::io::Read;
use std::io::Write;

use error::{Result, Error};

const BUF_LEN: usize = 42;

/// Spell Checker
///
/// Checks the spelling of a list of words
pub struct SpellChecker {
    ispell: Child,
}

impl SpellChecker {
    /// Creates a new spell checker from a running process
    pub fn new(process: Child) -> Result<SpellChecker> {
        let mut checker = SpellChecker {
            ispell: process,
        };

        // Skip the introduction
        // Read the first line that displays Version
        try!(checker.write_str(""));
        try!(checker.read_str());

        Ok(checker)        
    }

    /// Reads the output from ispell
    fn read_str(&mut self) -> Result<String> {
        if let Some(ref mut stdout) = self.ispell.stdout {
            let mut buffer = [0; BUF_LEN];
            let mut output = vec!();
            loop {
                let n = try!(stdout.read(&mut buffer));
                output.extend_from_slice(&buffer[0..n]);
                if n < BUF_LEN {
                    break;
                } else {
                    continue;
                }
            }
            Ok(try!(String::from_utf8(output)))
        } else {
            Err(Error::process("process should have been opened with a stdout pipe"))
        }
    }

    /// Write to ispell stdinn
    fn write_str(&mut self, text: &str) -> Result<()> {
        if let Some(ref mut stdin) = self.ispell.stdin {
            try!(stdin.write_all(text.as_bytes()));
            try!(stdin.write_all("\n".as_bytes()));
            try!(stdin.flush());
        } else {
            return Err(Error::process("ispell's stdin and stdout were not properly piped"));
        }
        Ok(())
    }

    
    /// Checks the spelling of a string
    pub fn check(&mut self, text: &str) -> Result<()> {
        try!(self.write_str(text));

        let words = text.split_whitespace().count();
        let mut n_lines = 0;

        while n_lines < words {
            let s = try!(self.read_str());
            print!("{}", s);
            for line in s.lines() {
                n_lines += 1;
                if n_lines >= words {
                    break;
                }
                let first = line.chars().next().unwrap();
                match first {
                    '*' => println!("OK"),
                    '&' => println!("Error: {}", line),
                    _ => println!("???: {}", line),//return Err(Error::protocol(format!("expected '&' or '*', found '{}'", first)))
                }
            }
        }
        
        Ok(())
    }    
}

impl Drop for SpellChecker {
    fn drop(&mut self) {
        // We could do this more nicely 
        self.ispell.kill().unwrap();
    }
}
