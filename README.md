rust-ispell 
===========

This library allows to easily use `ispell` or `aspell` from Rust.

Usage
-----

Add this to your `Cargo.toml`

```toml
[dependencies]
ispell = "0.1"
```

Example
-------

```rust
extern crate ispell;
use ispell::SpellLauncher;

fn main() {
    let mut checker = SpellLauncher::new()
        .aspell()
        .dictionary("en_GB")
        .launch()
        .unwrap();
    let errors = checker.check("A simpel test to see if it detetcs typing errors").unwrap();
    for e in errors {
        println!("'{}' (pos: {}) is misspelled!", &e.misspelled, e.position);
        if !e.suggestions.is_empty() {
            println!("Maybe you meant '{}'?", &e.suggestions[0]);
        }
    }
}
```

will display:

```
'simpel' (pos: 2) is misspelled!
Maybe you meant 'simple'?
'detetcs' (pos: 27) is misspelled!
Maybe you meant 'dietetics'?
```

(*Yes*, that is exactly what I meant.)

Documentation
-------------

For more information about using this library, see the
[API documentation on Github.io](https://lise-henry.github.io/rust-ispell/ispell/)
or on [docs.rs](https://docs.rs/releases/search?query=ispell).

ChangeLog
---------

See [ChangeLog.md](ChangeLog.md).

License 
-------

`rust-ispell` is free software, published under the
[Mozilla Public License, version 2.0](https://www.mozilla.org/en-US/MPL/2.0/). 
