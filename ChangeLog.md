ChangeLog 
=========

unreleased
----------
* No longer blocks if the spawned process stops outputting text.

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
