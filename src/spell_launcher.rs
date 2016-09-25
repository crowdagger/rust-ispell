// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with
// this file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::process::Command;
use std::process::Stdio;

use spell_checker::SpellChecker;
use error::{Result, Error};

#[derive(Debug)]
pub struct SpellLauncher {
    lang: String,
    command: String,
}

/// Spell Launcher wizard (ah, ah).
///
/// Runs `ispell` or one of its variant for you.
///
/// # Example
///
/// ```
/// use ispell::SpellLauncher;
/// let checker = SpellLauncher::new()
///                 .command("aspell") 
///                 .language("en")
///                 .launch()
///                 .unwrap();
/// ```
impl SpellLauncher {
    /// Creates a new spell checker with default options
    pub fn new() -> SpellLauncher {
        SpellLauncher {
            lang: "en".to_owned(),
            command: "ispell".to_owned(),
        }
    }

    /// Set the name of the command to run
    ///
    /// By default, set to "ispell"
    pub fn command<S: Into<String>>(&mut self, command: S) -> &mut SpellLauncher {
        self.command = command.into();
        self
    }

    /// Set the language for spell checking
    ///
    /// By default, set to "en"
    pub fn language<S: Into<String>>(&mut self, lang: S) -> &mut SpellLauncher {
        self.lang = lang.into();
        self
    }

    /// Launch a SpellChecker
    pub fn launch(&self) -> Result<SpellChecker> {
        let res = Command::new(&self.command)
            .arg("-l")
            .arg(&self.lang)
            .arg("-a")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn();

        match res {
            Ok(child) => Ok(SpellChecker::new(child)),
            Err(err) => Err(Error::new(format!("could not successfully spawn '{}' command: {}", self.command, err)))
        }
        
    }
}
