// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with
// this file, You can obtain one at https://mozilla.org/MPL/2.0/.  

//! This library provides an interface for easily calling the `ispell`
//! or `aspell` command from Rust programs.
//!
//! # Example
//!
//! ```
//! use ispell::SpellLauncher;
//! let mut checker = SpellLauncher::new()
//!                  .aspell()
//!                  .launch()
//!                  .unwrap();
//! let errors = checker.check("Testing iff if it works").unwrap();
//! assert_eq!(&errors[0].misspelled, "iff");
//! ```
//!
//! # The `SpellLauncher`
//!
//! You can set the command that will be called manually:
//!
//! ```
//! # use ispell::SpellLauncher;
//! let result = SpellLauncher::new()
//!              .command("foo")
//!              .launch();
//! ```
//!
//! but the easiest way to set which alternative must be used is with
//! the `aspell` and `hunspell` methods:
//!
//! ```
//! # use ispell::SpellLauncher;
//! let result = SpellLauncher::new()
//!              .hunspell()
//!              .launch();
//! ```
//!
//! # The `SpellChecker`
//!
//! If the command has been launched successfully, it will return a `SpellChecker`.
//!
//! ## Checking words
//!
//! The main usage of this struct is to get the errors
//! (`IspellError`) the spell checker detects with
//! `SpellChecker::check`. The `ispell` API returns the position (in 
//! characters) from the beginning of the line. This means that, if
//! you want to be able do to anything with these numbers, you'll have
//! to call `check` line by line.
//!
//! This method returns a list of `IspellError`s, containing:
//!
//! * the misspelled word;
//! * the position (number of characters since the beginning of the
//! line);
//! * a (possibly empty) list of suggestions.
//!
//! ```
//! # use ispell::SpellLauncher;
//! let mut checker = SpellLauncher::new()
//!                   .launch()
//!                   .unwrap();
//! let errors = checker.check("Does thit message contain any erors?").unwrap();
//! for e in errors {
//!     println!("{} was misspelled at pos {}.", e.misspelled, e.position);
//!     println!("There are {} suggestions for alternatives", e.suggestions.len());
//! }
//! ```
//!
//! `SpellChecker` also provides the `check_raw` method, whose behaviour mimics more closely
//! ispell's output.
//!
//! ## Adding words
//!
//! There are two methods to add words so they are no more detected as errors:
//!
//! * `add_word` adds a word to this current session, but doesn't save it;
//! * `add_word_to_dictionary` adds a word to your personal dictionary, saving it for
//!    next sessions.
//!
//! ```
//! # use ispell::SpellLauncher;
//! let mut checker = SpellLauncher::new()
//!                   .launch()
//!                   .unwrap();
//! checker.add_word("foobar"); // Add a word only to this session
//! checker.add_word_to_dictionary("rustacean"); // Add a word and saves it
//! let errors = checker.check("foobar rustacean").unwrap();
//! assert!(errors.is_empty());
//! ```
//!
//! # Languages
//!
//! `ispell`, `aspell` and `hunspell` all allow you to specify which dictionary must be used,
//! but they don't necessarily use the same naming scheme. `ispell` uses full names:
//!
//! ```
//! # use ispell::SpellLauncher;
//! let result = SpellLauncher::new()
//!              .dictionary("american")
//!              .launch();
//! ```
//!
//! `hunspell` uses unicode language codes:
//! 
//! ```
//! # use ispell::SpellLauncher;
//! let result = SpellLauncher::new()
//!              .hunspell()
//!              .dictionary("en_US")
//!              .launch();
//! ```
//!
//! whereas `aspell` accepts both versions.
//!
//! # Character encoding
//!
//! This library tries to set encoding to `utf-8`, but ispell, hunspell and aspell takes different arguments
//! for that. This is why you should use the `ispell`, `aspell` and `hunspell` methods intead of setting the
//! command to invoke with the `command` method.


mod spell_checker;
mod spell_launcher;
mod error;
mod ispell_result;
mod async_reader;

pub use ispell_result::IspellResult;
pub use ispell_result::IspellError;
pub use spell_checker::SpellChecker;
pub use spell_launcher::SpellLauncher;
pub use error::Error;
pub use error::Result;
