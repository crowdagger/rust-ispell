// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with
// this file, You can obtain one at https://mozilla.org/MPL/2.0/.

extern crate ispell;
use ispell::SpellLauncher;

fn main() {
    let checker = SpellLauncher::new()
        .command("ispell")
        .launch();
    match checker {
        Ok(mut checker) => {
            let res = checker.check("test of a msitake").unwrap();
            println!("res: {:?}", res);
            let res = checker.check("test without mistake (?)").unwrap();
            println!("res: {:?}", res);
            let res = checker.check("Another test wiht a mistake").unwrap();
            println!("res: {:?}", res);
            
            let res = checker.check("killmaster").unwrap();
            println!("res: {:?}", res);
        },
        Err(err) => println!("Error: {}", err)
    }
}
