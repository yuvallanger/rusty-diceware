extern crate rand;

use std::io::Read;
use rand::Rng;

fn main() {
    let beale_contents = include_str!("../bin/wordlists/beale.wordlist.asc");

    let v: Vec<_> = beale_contents.split('\n')
        .skip(2)
        .take(7776)
        .map(|s| s.splitn(2, '\t').nth(1).unwrap())
        .collect();

    let mut rng = rand::OsRng::new().unwrap();
    for _ in 0..7 {
        let c = rng.choose(&v);
        print!("{} ", c.unwrap());
    }
    println!("");
}
