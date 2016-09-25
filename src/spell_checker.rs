// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with
// this file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::process::Child;
use std::io::Read;
use std::io::Write;

/// Spell Checker
///
/// Checks the spelling of a list of words
pub struct SpellChecker {
    ispell: Child,
}

impl SpellChecker {
    /// Creates a new spell checker from a running process
    pub fn new(process: Child) -> SpellChecker {
        SpellChecker {
            ispell: process,
        }
    }

    /// Checks the spelling of a string
    pub fn check(&mut self, text: &str) {
        if let Some(ref mut stdout) = self.ispell.stdout {
            let mut buffer:Vec<u8> = vec!(0;100);
            let n = stdout.read(&mut buffer).unwrap();
            buffer.truncate(n);
            let s = String::from_utf8(buffer).unwrap();
            println!("{}", s);
        }
        if let Some(ref mut stdin) = self.ispell.stdin {
            stdin.write_all(text.as_bytes()).unwrap();
            stdin.write_all("\n".as_bytes()).unwrap();
            stdin.flush().unwrap();
        }
        if let Some(ref mut stdout) = self.ispell.stdout {
            let mut buffer:Vec<u8> = vec!(0;100);
            let n = stdout.read(&mut buffer).unwrap();
            buffer.truncate(n);
            let s = String::from_utf8(buffer).unwrap();
            println!("{}", s);
                        let mut buffer:Vec<u8> = vec!(0;100);
            let n = stdout.read(&mut buffer).unwrap();
            buffer.truncate(n);
            let s = String::from_utf8(buffer).unwrap();
            println!("{}", s);
        }



        //println!("{}", output);
    

        self.ispell.kill().unwrap();
    }    
}

