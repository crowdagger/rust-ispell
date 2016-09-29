extern crate ispell;
use ispell::SpellLauncher;

fn main() {
    let checker = SpellLauncher::new()
        .command("examples/sleep.sh")
        .timeout(5)
        .launch();
    assert!(checker.is_err());
    if let Err(err) = checker {
        println!("{}", err);
    }
}
