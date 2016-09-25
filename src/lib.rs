// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with
// this file, You can obtain one at https://mozilla.org/MPL/2.0/.  

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
