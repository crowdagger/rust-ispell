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
//! assert_eq!(&errors[0].mispelled, "iff");
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
//! The main usage of this struct is to get the errors
//! (`IspellError`) the spell checker detects with
//! `SpellChecker::check`. The `ispell` API returns the position (in 
//! characters) from the beginning of the line. This means that, if
//! you want to be able do to anything with these numbers, you'll have
//! to call `check` line by line.
//!
//! This method returns a list of `IspellError`s, containing:
//!
//! * the mispelled word;
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
//!     println!("{} was mispelled at pos {}.", e.mispelled, e.position);
//!     println!("There are {} suggestions for alternatives", e.suggestions.len());
//! }
//! ```
//!
//! `SpellChecker` also provides the `check_raw` method, whose behaviour mimics more closely
//! ispell's output.
//!
//! # Languages and encodings
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
//! Currently, no encoding is specified when runnig i/a/hun/spell, so, depending on the system
//! you are using and the command you use, it is possible you'll encounter problems if you use non-ASCII characters.
//!
//! # Warning
//!
//!  This library hasn't been tested a lot yet. It tries to avoid
//! `panic!`s but, unfortunately, since `Read`s are blocking, there is
//! a risk that it will simply hang up infinitely (particularly if you use
//! `SpellLauncher::command(...)` to launch a process that doesn't comply with
//! the ispell API).

mod spell_checker;
mod spell_launcher;
mod error;
mod ispell_result;

pub use ispell_result::IspellResult;
pub use ispell_result::IspellError;
pub use spell_checker::SpellChecker;
pub use spell_launcher::SpellLauncher;
pub use error::Error;
pub use error::Result;
