// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with
// this file, You can obtain one at https://mozilla.org/MPL/2.0/.  

//! This library provides an interface for easily calling the `ispell`
//! or `aspell` command from Rust programs.
//!
//! # Example
//!
//! ```
//! # use ispell::SpellLauncher;
//! let mut checker = SpellLauncher::new()
//!                  .aspell(true)
//!                  .launch()
//!                  .unwrap();
//! let errors = checker.check("Testing iff if it works").unwrap();
//! assert_eq!(&errors[0].mispelled, "iff");
//! ```
//!
//! # Warning
//!
//! > This library hasn't been very tested yet. It tries to avoid
//! `panic!`s but, unfortunately, since `Read`s are blocking, there is
//! a risk that it will simply hang up infinitely. 

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
