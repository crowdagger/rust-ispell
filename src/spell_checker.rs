// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with
// this file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::process::{Child, ChildStdin};
use std::io::Write;
use std::time::Duration;
use std::thread;
use std::sync::mpsc::{channel, Receiver, TryRecvError};

use error::{Result, Error};
use ispell_result::{IspellResult, IspellError};
use async_reader::AsyncReader;

/// Spell Checker
///
/// Checks the spelling of a line.
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
    stdin: ChildStdin,
    receiver: Receiver<Result<String>>,
    timeout: Duration,
    _child: thread::JoinHandle<()>,
}

impl SpellChecker {
    /// Creates a new spell checker from a running process
    #[doc(hidden)]
    pub fn new(mut process: Child, timeout: u64) -> Result<SpellChecker> {
        let stdin = if let Some(stdin) = process.stdin.take() {
            stdin
        } else {
            return Err(Error::process("could not access stdin of spawned process"));
        };

        let stdout = if let Some(stdout) = process.stdout.take() {
            stdout
        } else {
            return Err(Error::process("could not access stdin of spawned process"));
        };

        let (sender, receiver) = channel();
        let mut reader = AsyncReader::new(stdout, sender);
        let child = thread::spawn(move || {
            reader.read_loop();
        });
        
        let mut checker = SpellChecker {
            ispell: process,
            stdin: stdin,
            timeout: Duration::from_millis(timeout),
            receiver: receiver,
            _child: child,
        };

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
        match self.receiver.recv_timeout(self.timeout) {
            Ok(result) => result,
            Err(_) => return Err(Error::process("timeout error: spawned process didn't respond in time, aborting")),
        }
    }

    /// Flushes the stdout of the spawned process, so we are sure we start
    /// reading an answer to what we just wrote
    fn flush_stdout(&mut self) -> Result<()> {
        loop {
            match self.receiver.try_recv() {
                Ok(_) => continue,
                Err(TryRecvError::Empty) => break,
                Err(TryRecvError::Disconnected) => return Err(Error::process("spawned process closed its stdout early, aborting")),
            }
        }
        Ok(())
    }

    /// Write to ispell stdin
    fn write_str(&mut self, text: &str) -> Result<()> {
        // First, clear ispell's stdout just in case
        try!(self.flush_stdout());
        
        try!(self.stdin.write_all(b"^"));
        try!(self.stdin.write_all(text.as_bytes()));
        try!(self.stdin.write_all(b"\n"));
        try!(self.stdin.flush());
        Ok(())
    }

    /// Adds a word to your personal dictionary
    ///
    /// The word will be saved to your words file (e.g. `~/.ispell_LANG,` `~/.hunspell_LANG, or `~/.aspell.LANG.pws`),
    /// so it will be memorized next time you use i/a/hun/spell. If you only want to add the word to
    /// this current session, use `add_word`.
    ///
    /// # Returns
    ///
    /// An error if `word` contains spaces or illegal characters
    ///
    /// # Examples
    ///
    /// Adding a valid word
    ///
    /// ```rust,no_run
    /// use ispell::SpellLauncher;
    ///
    /// fn main() {
    ///     let mut checker = SpellLauncher::new()
    ///         .launch()
    ///         .unwrap();
    ///    
    ///     // "rustacean" is not a valid word...
    ///     // (unless you already ran this code example)
    ///     let errors = checker.check("rustacean").unwrap();
    ///     assert_eq!(errors.len(), 1);
    ///
    ///     // let's add it to our personal dictionary
    ///     checker.add_word_to_dictionary("rustacean").unwrap();
    ///
    ///     // now it is a valid word
    ///     let errors = checker.check("rustacean").unwrap();
    ///     assert!(errors.is_empty());
    /// }
    /// ```
    pub fn add_word_to_dictionary(&mut self, word: &str) -> Result<()> {
        if word.contains(|c:char| !c.is_alphabetic()) {
            return Err(Error::invalid_word(format!("word '{}' contains non alphabetic characters",
                                                   word)));
        }
        try!(self.stdin.write_all(b"*"));
        try!(self.stdin.write_all(word.as_bytes()));
        try!(self.stdin.write_all(b"\n"));

        // Save the dictionary
        try!(self.stdin.flush());
        try!(self.stdin.write_all(b"#\n"));
        try!(self.stdin.flush());
        Ok(())
    }

    /// Add a word to current session.
    ///
    /// This word won't be memorized the next time you use i/a/hun/spell. If you want this behaviour,
    /// use `add_word_to_dictionary`.
    ///
    /// # Returns
    ///
    /// An error if `word` contains spaces or illegal characters
    ///
    /// ```rust,no_run
    /// use ispell::SpellLauncher;
    ///
    /// fn main() {
    ///     let mut checker = SpellLauncher::new()
    ///         .launch()
    ///         .unwrap();
    ///    
    ///     // "rustaholic" is not a valid word...
    ///     // (even if you already ran this code example)
    ///     let errors = checker.check("rustaholic").unwrap();
    ///     assert_eq!(errors.len(), 1);
    ///
    ///     // let's add it to this session
    ///     checker.add_word("rustaholic").unwrap();
    ///
    ///     // now it is a valid word
    ///     let errors = checker.check("rustaholic").unwrap();
    ///     assert!(errors.is_empty());
    /// }
    /// ```
    pub fn add_word(&mut self, word: &str) -> Result<()> {
        if word.contains(|c:char| !c.is_alphabetic()) {
            return Err(Error::invalid_word(format!("word '{}' contains non alphabetic characters",
                                                   word)));
        }
        try!(self.stdin.write_all(b"@"));
        try!(self.stdin.write_all(word.as_bytes()));
        try!(self.stdin.write_all(b"\n"));
        try!(self.stdin.flush());
        Ok(())
    }
    

    /// Checks the spelling of a line.
    ///
    /// This method only returns the errors that ispell detects. Since the position returned
    /// in those errors is the number of characters since the beginning of the line, this method
    /// needs to be called line by line and not on a full document.
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
    let misspelled = words[1].to_owned();
    let position:usize = try!(words[n - 1].parse()
                              .map_err(|_| Error::protocol(format!("could not parse '{}' as an int", words[2]))));
    Ok(IspellError {
        misspelled: misspelled,
        position: position - 1, // remove the '^' character we add for escaping
        suggestions: vec!(),
        })
}


