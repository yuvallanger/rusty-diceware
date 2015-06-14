mod diceware;

extern crate rand;
extern crate getopts;

//use std::env;
//use std::io::Read;
// use std::fs::File;

// use getopts::Options;
use rand::Rng;

use diceware::{BealeWord, ReinholdWord, MiniLockWord};
/*
fn make_options() -> Options {
    let mut opts = Options::new();
    opts.optflag("h", "help", "this help message");
    opts.optflag("", "beale", "use the beale wordlist");
    opts.optflag("", "reinhold", "use the standard wordlist");
    opts.optopt("n", "nword", "number of words in a passphrase", "NWORD");
    opts
}

fn print_passphrase(wordlist: Vec<&str>, word_num: u64) -> () {
    let mut rng = rand::OsRng::new().unwrap();
    let mut c = rng.choose(&wordlist);
    for _ in 0..word_num-1 {
        print!("{} ", c.unwrap());
        c = rng.choose(&wordlist);
    }
    println!("{}", c.unwrap());
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}

*/

#[cfg(not(test))]
fn main() {
    /*
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let opts = make_options();

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    let word_num: u64 = matches.opt_str("n").ok() {
        Some(n) => { n.parse::<u64>().err }
        None => {
            print_usage(&program, opts);
            return;
        }
    };
    */

    let mut rng = rand::OsRng::new().unwrap();

    println!("Beale:");
    for _ in 1..8 {
        let word: diceware::BealeWord = rng.gen();
        print!("{} ", word);
    }
    println!("");
    println!("");

    println!("Reinhold:");
    for _ in 1..8 {
        let word: diceware::ReinholdWord = rng.gen();
        print!("{} ", word);
    }
    println!("");
    println!("");

    println!("MiniLock:");
    for _ in 1..8 {
        let word: diceware::MiniLockWord = rng.gen();
        print!("{} ", word);
    }
    println!("");

}

#[cfg(test)]
mod test {
    extern crate rand;

    use rand::{Rng, SeedableRng, StdRng};

    use diceware::{ReinholdWord, BealeWord};

    fn make_beale_word() -> BealeWord {
        let seed: &[_] = &[1, 2, 3, 4];
        let mut rng: StdRng = SeedableRng::from_seed(seed);
        let word = rng.gen();
        word
    }

    fn make_reinhold_word() -> ReinholdWord {
        let seed: &[_] = &[1, 2, 3, 4];
        let mut rng: StdRng = SeedableRng::from_seed(seed);
        let word = rng.gen();
        word
    }

    #[test]
    fn beale_rng_test() {
        let rand_word = make_beale_word();
        assert_eq!(rand_word, BealeWord::new("ladder"))
    }

    #[test]
    fn reinhold_rng_test() {
        let rand_word = make_reinhold_word();
        assert_eq!(rand_word, ReinholdWord::new("ks"))
    }
}
