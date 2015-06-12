extern crate rand;
extern crate getopts;


//use std::env;
//use std::io::Read;
// use std::fs::File;

// use getopts::Options;
use rand::Rng;

mod diceware {
    extern crate rand;

    use std::fmt;

    include!(concat!(env!("OUT_DIR"), "/diceware.rs"));

    #[derive(Debug,Clone)]
    pub struct BealeWord(pub &'static str);

    #[derive(Debug,Clone)]
    pub struct ReinholdWord(pub &'static str);

    impl rand::Rand for BealeWord {
        fn rand<R: rand::Rng>(rng: &mut R) -> BealeWord {
            rng.choose(&BEALE_WORDLIST).unwrap().clone()
        }
    }

    impl fmt::Display for BealeWord {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                &BealeWord(w) => write!(f, "{}", w)
            }
        }
    }

    impl rand::Rand for ReinholdWord {
        fn rand<R: rand::Rng>(rng: &mut R) -> ReinholdWord {
            rng.choose(&REINHOLD_WORDLIST).unwrap().clone()
        }
    }

    impl fmt::Display for ReinholdWord {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                &ReinholdWord(w) => write!(f, "{}", w)
            }
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
    for _ in 1..8 {
        let diceware::BealeWord(word) = rng.gen();
        print!("{} ", word);
    }
    println!("");
}
