ChangeLog 
=========

unreleased
------------
* Added a `.travis.yml` file.

0.3.0 (2016-09-30)
------------------
* Added a `timeout` method to SpellLauncher. This allows to
  specify a timeout (in milliseconds) when reading from spawned
  process's output, so it no longer blocks your program infinitely if
  it stops outputting text.
* Added the following methods in `SpellChecker` to add words:
  * `add_word`: add a word for the current session;
  * `add_word_to_dictionary`: add a word to your personal directory
    (should be saved between sessions).

**Breaking change**: rename `mispelled` in `IspellError` to
  `misspelled`. Yes, it's quite a shame (but ironical) that the only
  breaking change in this release of a spell checker library comes
  from a spelling issue, but well...

0.2.0 (2016-09-26) 
------------------
* Slightly fix the API:
  * Remove `language` method as it was redundant with `dictionary`;
  * `aspell` method doesn't take a bool anymore;
  * Add `hunspell` method to set `SpellLauncher` to use `hunspell`
* Documentation is now more complete.

0.1.0 (2016-09-26)
------------------
* First release.
