// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with
// this file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::process::Child;
use std::io::Read;
use std::io::Write;

use error::{Result, Error};
use ispell_result::{IspellResult, IspellError};

const BUF_LEN: usize = 42;

/// Spell Checker
///
/// Checks the spelling of a list of words
///
/// # Example
///
/// ```
/// use ispell::SpellLauncher;
/// let mut checker = SpellLauncher::new().launch().unwrap();
/// let errors = checker.check("This should not contain any error").unwrap();
/// assert!(errors.is_empty());
/// ```
pub struct SpellChecker {
    ispell: Child,
}

impl SpellChecker {
    /// Creates a new spell checker from a running process
    #[doc(hidden)]
    pub fn new(process: Child) -> Result<SpellChecker> {
        let mut checker = SpellChecker {
            ispell: process,
        };

        // Skip the introduction
        // Read the first line that displays Version
        //        try!(checker.write_str(""));

        
        
        let s = try!(checker.read_str());
        match s.chars().next() {
            Some('@') => Ok(checker),
            _ => Err(Error::protocol(format!("First line of ispell output doesn't start with '@', aborting")))
        }
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
    ///
    /// This method only returns the error that ispell detects.
    pub fn check(&mut self, text: &str) -> Result<Vec<IspellError>> {
        let results = try!(self.check_raw(text));
        let mut errors = vec!();

        for elem in results.into_iter() {
            match elem {
                IspellResult::Miss(error)
                    | IspellResult::Guess(error)
                    | IspellResult::None(error)
                    => errors.push(error),

                _ => (),
            }
        }
        Ok(errors)
    }

    
    /// Checks the spelling of a string
    ///
    /// This method returns a vector of all `ispell` answers, even when
    /// there is no errors. Usually, the `check` method, which only returns
    /// errors, will be more useful.
    pub fn check_raw(&mut self, text: &str) -> Result<Vec<IspellResult>> {
        try!(self.write_str(text));

    
        let n_words = text.split_whitespace().count();
        let mut output = Vec::with_capacity(n_words);
        let mut n_lines = 0;

        
        while n_lines < n_words {
            let s = try!(self.read_str());
            for line in s.lines() {
                if n_lines >= n_words {
                    break;
                }
                n_lines += 1;
                if line.is_empty() {
                    continue;
                }
                let first = line.chars().next().unwrap();
                match first {
                    '*' => output.push(IspellResult::Ok),
                    '-' => output.push(IspellResult::Compound),
                    '+' => {
                        let words:Vec<_> = line.split_whitespace().collect();
                        if words.len() != 2 {
                            return Err(Error::protocol(format!("'root' line ill-formatted: {}", line)));
                        }
                        output.push(IspellResult::Root(words[1].to_owned()));
                    },
                    '#' => {
                        let error = try!(get_ispell_error(line, 3));
                        output.push(IspellResult::None(error));
                    },
                    '&' | '?' => {
                        let parts: Vec<_> = line.split(':').collect();
                        if parts.len() != 2 {
                            return Err(Error::protocol(format!("unexpected output from ispell: {}", line)));
                        }
                        let mut error = try!(get_ispell_error(parts[0], 4));
                        let suggestions: Vec<_> = parts[1].split(",")
                            .map(|s| s.trim().to_owned())
                            .collect();
                        error.suggestions = suggestions;
                        if first == '&' {
                            output.push(IspellResult::Miss(error));
                        } else {
                            output.push(IspellResult::Guess(error));
                        }
                    },
                    _ => return Err(Error::protocol(format!("unexpected output: {}", line))),
                }
            }
        }
    
        Ok(output)
    }
}
    
                                                  
impl Drop for SpellChecker {
    fn drop(&mut self) {
        // We could do this more nicely 
        self.ispell.kill().unwrap();
    }
}

    
/// Transforms a string looking like "# unkwnown POS' or '& unknown n POS' to an IspellError
fn get_ispell_error(input: &str, n: usize) -> Result<IspellError> {
    let words: Vec<_> = input.split_whitespace().collect();
    if words.len() != n {
        return Err(Error::protocol(format!("unexpected result: {}", input)));
    }
    let mispelled = words[1].to_owned();
    let position:usize = try!(words[n - 1].parse()
                              .map_err(|_| Error::protocol(format!("could not parse '{}' as an int", words[2]))));
    Ok(IspellError {
        mispelled: mispelled,
        position: position,
        suggestions: vec!(),
        })
}
