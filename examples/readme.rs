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
        println!("'{}' (pos: {}) is mispelled!", &e.mispelled, e.position);
        if !e.suggestions.is_empty() {
            println!("Maybe you meant '{}'?", &e.suggestions[0]);
        }
    }
}
