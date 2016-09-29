extern crate ispell;
use ispell::SpellLauncher;

fn main() {
    let mut checker = SpellLauncher::new()
        .hunspell()
        .timeout(1000)
        .launch()
        .unwrap();
    
    // "rustacean" is not a valid word...
    let errors = checker.check("rustacéan").unwrap();
    println!("errors: {:?}", errors);
    assert_eq!(errors.len(), 1);
    
    // let's add it to our personal dictionary
//    checker.add_word_to_dictionary("rustacéan").unwrap();
    let errors = checker.check("rustacéan").unwrap();
    println!("errors: {:?}", errors);
    assert!(errors.is_empty());
}
