// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with
// this file, You can obtain one at https://mozilla.org/MPL/2.0/.

#[derive(Debug)]
pub struct SpellChecker {
    lang: String,
    command: String,
}

/// Spell Checker. Calls `ispell` or one of its variant for you.
/// 
/// # Example
/// 
/// ```
/// use ispell::SpellChecker;
///
/// let mut checker = SpellChecker::new();
/// checker.command("aspell");
/// ```
impl SpellChecker {
    /// Creates a new spell checker with default options
    pub fn new() -> SpellChecker {
        SpellChecker {
            lang: "en".to_owned(),
            command: "ispell".to_owned(),
        }
    }

    /// Set the name of the command to run
    pub fn command<S: Into<String>>(&mut self, command: S) {
        self.command = command.into();
    }
}
