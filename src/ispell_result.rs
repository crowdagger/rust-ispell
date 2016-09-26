// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with
// this file, You can obtain one at https://mozilla.org/MPL/2.0/.


/// An ispell error when, a word is not found in a dictionary
#[derive(Debug, PartialEq)]
pub struct IspellError {
    /// The mispelled word
    pub mispelled: String,

    /// The position of the word
    /// (number of characters since the beginning of the new line)
    pub position: usize,

    /// A list of suggestions
    pub suggestions: Vec<String>,
}

/// A result from ispell, corresponding to a line that is sent back for each word
#[derive(Debug, PartialEq)]
pub enum IspellResult {
    /// The word was found in the dictionnary.
    ///
    /// Corresponds to '*'
    Ok,
    
    /// The word wasn't found, but a root word was found.
    /// 
    /// Corresponds to '+'
    Root(String),

    /// The word wasn't found, but corresponds to the concatenation of two words
    ///
    /// Corresponds to '-'
    Compound,

    /// The word wasn't found, but there are near misses
    ///
    /// Corresponds to '&'
    Miss(IspellError),

    /// The word wasn't found, but could be formed by adding illegal affixes to a known root
    ///
    /// Corresponds to '?'
    Guess(IspellError),

    /// The word wasn't found in the dictionnary and there are no suggestions
    ///
    /// Corresponds to '#'
    None(IspellError),
}
