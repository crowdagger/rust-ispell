extern crate ispell;
use ispell::SpellLauncher;

fn main() {
    let mut checker = SpellLauncher::new()
        .launch()
        .unwrap();
    
    // "foobar" is not a valid word...
    let errors = checker.check("foobar").unwrap();
    println!("errors: {:?}", errors);
    assert_eq!(errors.len(), 1);
    
    // let's add it
    checker.add_word("foobar").unwrap();
    let errors = checker.check("foobar").unwrap();
    println!("errors: {:?}", errors);
    assert!(errors.is_empty());
}
