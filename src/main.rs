extern crate rand;
extern crate getopts;

//use std::env;
//use std::io::Read;
// use std::fs::File;

// use getopts::Options;
use rand::Rng;

mod diceware {
    extern crate rand;

    const BEALE_CONTENTS: &'static str = include_str!("../bin/wordlists/beale.wordlist.asc");
    const REINHOLD_CONTENTS: &'static str = include_str!("../bin/wordlists/diceware.wordlist.asc");


    fn make_wordlist(contents: &str) -> Vec<&str> {
        contents.split('\n')
            .skip(2)
            .take(7776)
            .map(|s| s.splitn(2, '\t').nth(1).unwrap())
            .collect()
    }

    #[derive(Debug)]
    pub struct BealeWord (&'static str);

    #[derive(Debug)]
    pub struct ReinholdWord (&'static str);

    impl rand::Rand for BealeWord {
        fn rand<R: rand::Rng>(rng: &mut R) -> BealeWord {
            let wordlist = make_wordlist(BEALE_CONTENTS);
            let c = rng.choose(&wordlist);
            BealeWord(c.unwrap())
        }
    }

    impl rand::Rand for ReinholdWord {
        fn rand<R: rand::Rng>(rng: &mut R) -> ReinholdWord {
            let wordlist = make_wordlist(REINHOLD_CONTENTS);
            let c = rng.choose(&wordlist);
            ReinholdWord(c.unwrap())
        }
    }
}

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
    let word: diceware::BealeWord = rng.gen();
    println!("{:?}", word);
}
