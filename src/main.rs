mod diceware;

#[cfg(test)]
mod tests;

extern crate getopts;
extern crate rand;

//use std::env;

use getopts::Options;
use rand::rngs::OsRng;
use rand::Rng;
use std::process::exit;

//use diceware::{BealeWord, ReinholdWord, MiniLockWord};

fn make_options() -> Options {
    let mut opts = Options::new();
    opts.optflag("h", "help", "this help message");
    opts.optflag("", "minilock", "use the MiniLock wordlist (default)");
    opts.optflag("", "reinhold", "use the standard wordlist");
    opts.optflag("", "beale", "use the beale wordlist");
    opts.optflag("e", "entropy", "display number of entropy bits");
    opts.optopt("n", "nword", "number of words in a passphrase", "NWORD");
    opts
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}

#[cfg(not(test))]
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let program = &args[0];

    let opts = make_options();

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            println!("{}\n", f.to_string());
            print_usage(&program, opts);
            exit(-1);
        }
    };

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    };

    let word_num: u64 = matches
        .opt_str("n")
        .map_or(8, |n_str| n_str.parse::<u64>().ok().unwrap());

    let mut rng = OsRng::new().unwrap();

    if matches.opt_present("reinhold") {
        for _ in 0..word_num {
            let word: diceware::ReinholdWord = rng.gen();
            print!("{} ", word);
        }
        println!();
        if matches.opt_present("entropy") {
            println!("{}", diceware::ReinholdWord::entropyn(word_num))
        }
        return;
    }

    if matches.opt_present("beale") {
        for _ in 0..word_num {
            let word: diceware::BealeWord = rng.gen();
            print!("{} ", word);
        }
        println!();
        if matches.opt_present("entropy") {
            println!("{}", diceware::BealeWord::entropyn(word_num))
        }
        return;
    }

    for _ in 0..word_num {
        let word: diceware::MiniLockWord = rng.gen();
        print!("{} ", word);
    }
    println!();
    if matches.opt_present("entropy") {
        println!("{}", diceware::MiniLockWord::entropyn(word_num))
    }
}
