// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with
// this file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::process::Command;
use std::process::Stdio;

use spell_checker::SpellChecker;
use error::{Result, Error};

#[derive(Debug)]
pub struct SpellLauncher {
    lang: Option<String>,
    dict: Option<String>,
    command: Option<String>,
    aspell: bool,
}

/// Spell Launcher wizard (ah, ah).
///
/// Runs `ispell` or one of its variant for you.
///
/// # Examples
///
/// * Launches `ispell` with `british` dictionary:
///
/// ```
/// use ispell::SpellLauncher;
/// let checker = SpellLauncher::new()
///               .dictionary("british")
///               .launch()
///               .unwrap();
/// ```
///
/// * Launches `aspell` with french (France) language:
/// 
/// ```
/// use ispell::SpellLauncher;
/// let checker = SpellLauncher::new()
///               .aspell(true)
///               .language("fr_FR")
///               .launch()
///               .unwrap();
/// ```
impl SpellLauncher {
    /// Creates a new spell checker with default options
    pub fn new() -> SpellLauncher {
        SpellLauncher {
            lang: None,
            command: None,
            dict: None,
            aspell: false,
        }
    }

    /// If true, sets compatibility mode to aspell instead of ispell.
    pub fn aspell(&mut self, b: bool) -> &mut SpellLauncher {
        self.aspell = b;
        self
    }
    
    /// Set the name of the command to run
    ///
    /// By default, it is "ispell" or "aspell" if the `aspell` flag has been set.
    pub fn command<S: Into<String>>(&mut self, command: S) -> &mut SpellLauncher {
        self.command = Some(command.into());
        self
    }

    /// Set the dictionary that should be used
    /// 
    /// # Example
    ///
    /// ```
    /// use ispell::SpellLauncher;
    /// let checker = SpellLauncher::new()
    ///               .dictionary("american")
    ///               .launch()
    ///               .unwrap();
    /// ```
    pub fn dictionary<S: Into<String>>(&mut self, dict: S) -> &mut SpellLauncher {
        self.dict = Some(dict.into());
        self
    }
    
    /// Set the language for spell checking
    ///
    /// Used only if the `aspell` flag has been set, else use `dictionary`.
    ///
    /// # Example
    ///
    /// ```
    /// use ispell::SpellLauncher;
    /// let checker = SpellLauncher::new()
    ///               .aspell(true)
    ///               .language("en_GB")
    ///               .launch()
    ///               .unwrap();
    /// ```
    pub fn language<S: Into<String>>(&mut self, lang: S) -> &mut SpellLauncher {
        self.lang = Some(lang.into());
        self
    }

    /// Launch a SpellChecker
    pub fn launch(&self) -> Result<SpellChecker> {
        let command_name: &str = if let Some(ref command) = self.command {
            command
        } else {
            if self.aspell {
                "aspell"
            } else {
                "ispell"
            }
        };
        let mut command = Command::new(command_name);
        command
            .arg("-a")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped());
        if self.aspell {
            if let Some(ref lang) = self.lang {
                command.arg("-l")
                    .arg(lang);
            }
        }
        if let Some(ref dict) = self.dict {
            command.arg("-d")
                .arg(dict);
        }
        let res = command.spawn();

        match res {
            Ok(child) => SpellChecker::new(child),
            Err(err) => Err(Error::process(format!("could not successfully spawn process '{}': {}", command_name, err)))
        }
    }
}
