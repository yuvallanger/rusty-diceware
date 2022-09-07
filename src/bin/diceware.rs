extern crate getopts;
extern crate rand;

use std::process::exit;

use getopts::Options;
use rand::thread_rng;

fn make_options() -> Options {
    let mut opts = Options::new();
    opts.optflag("h", "help", "this help message");
    opts.optflag("", "minilock", "use the MiniLock wordlist (default)");
    opts.optflag("", "reinhold", "use the standard wordlist");
    opts.optflag("", "beale", "use the beale wordlist");
    opts.optflag("e", "entropy", "display number of entropy bits");
    opts.optopt("n", "nword", "number of words in a passphrase", "NWORD");
    opts.optopt(
        "d",
        "delimiter",
        "the delimiter character used to separate the words",
        "DELIM",
    );
    //opts.optopt("l", "wordlist", "Wordlist to use (minilock (default), reinhold, or beale)", "WORDLIST");
    opts
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let program = &args[0];

    let opts = make_options();

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            println!("{}\n", f);
            print_usage(program, opts);
            exit(-1);
        }
    };

    if matches.opt_present("h") {
        print_usage(program, opts);
        return;
    };

    let word_num: u64 = matches
        .opt_str("n")
        .map_or(8, |n_str| n_str.parse::<u64>().ok().unwrap());

    let delimiter: char = matches
        .opt_str("d")
        .map_or(' ', |n_str| n_str.parse::<char>().ok().unwrap());

    let is_entropy_printed = matches.opt_present("entropy");

    let mut rng = thread_rng();

    if word_num != 0 {
        if matches.opt_present("reinhold") {
            diceware::print_words(
                diceware::REINHOLD_WORDLIST.to_vec(),
                &word_num,
                &delimiter,
                &is_entropy_printed,
                &mut rng,
            );
        } else if matches.opt_present("beale") {
            diceware::print_words(
                diceware::BEALE_WORDLIST.to_vec(),
                &word_num,
                &delimiter,
                &is_entropy_printed,
                &mut rng,
            );
        } else {
            diceware::print_words(
                diceware::MINILOCK_WORDLIST.to_vec(),
                &word_num,
                &delimiter,
                &is_entropy_printed,
                &mut rng,
            );
        }
    }
}
