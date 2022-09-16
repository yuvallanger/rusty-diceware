extern crate getopts;
extern crate rand;

use std::process::exit;

use getopts::Options;
use rand::thread_rng;

use diceware::load_wordlist_file;
use diceware::print_words;
use diceware::wordlists::BEALE_WORDLIST;
use diceware::wordlists::MINILOCK_WORDLIST;
use diceware::wordlists::REINHOLD_WORDLIST;

fn make_options() -> Options {
    let mut opts = Options::new();
    opts.optflag("h", "help", "this help message");
    opts.optflag(
        "",
        "minilock",
        "[OBSOLETE] use the MiniLock wordlist (default)",
    );
    opts.optflag("", "reinhold", "[OBSOLETE] use the standard wordlist");
    opts.optflag("", "beale", "[OBSOLETE] use the beale wordlist");
    opts.optflag("e", "entropy", "display number of entropy bits");
    opts.optopt("n", "nword", "number of words in a passphrase", "NWORD");
    opts.optopt(
        "d",
        "delimiter",
        "the delimiter character used to separate the words",
        "DELIM",
    );
    opts.optopt("f", "wordlist-file", "path to a wordlist file", "FILE");
    opts.optopt(
        "l",
        "wordlist",
        "Wordlist to use (minilock (default), reinhold, or beale)",
        "WORDLIST",
    );
    opts
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}

fn unknown_wordlist(wordlist_name: &str) -> ! {
    eprintln!(
        "Unknown wordlist: {}. Available wordlists: beale, reinhold, or minilock.",
        wordlist_name,
    );
    exit(-1)
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

    let is_reinhold_flag: bool = matches.opt_present("reinhold");
    let is_beale_flag: bool = matches.opt_present("beale");
    let is_minilock_flag: bool = matches.opt_present("minilock");
    let is_wordlist_flag: bool = is_reinhold_flag | is_beale_flag | is_minilock_flag;

    if is_wordlist_flag {
        eprintln!("WARNING! The --reinhold, --beale and --minilock flags are deprecated and will be removed in the next minor version release. Use the -l or --wordlist flags instead.");
    };

    let wordlist_name = if let Some(wordlist_option) = matches.opt_str("l") {
        if is_wordlist_flag {
            eprintln!("ERROR! The --reinhold, --beale, and --minilock flags cannot be used together with the -l or --wordlist flags.");
            exit(-1);
        }
        match wordlist_option.to_lowercase().as_ref() {
            z @ ("beale" | "reinhold" | "minilock") => z,
            _ => unknown_wordlist(&wordlist_option),
        }
        .to_string()
    } else {
        if is_reinhold_flag {
            "reinhold"
        } else if is_beale_flag {
            "beale"
        } else {
            "minilock"
        }
        .to_string()
    };

    let mut rng = thread_rng();

    if word_num != 0 {
        if let Some(wordlist_filepath) = matches.opt_str("f") {
            let wordlist_string = load_wordlist_file(&wordlist_filepath);

            let wordlist = wordlist_string
                .split('\n')
                .map(|x| x.trim())
                .filter(|x| x != &"")
                .collect::<Vec<&str>>();

            print_words(
                &wordlist,
                &word_num,
                &delimiter,
                &is_entropy_printed,
                &mut rng,
            );
        } else {
            match wordlist_name.as_ref() {
                "reinhold" => {
                    print_words(
                        REINHOLD_WORDLIST.as_ref(),
                        &word_num,
                        &delimiter,
                        &is_entropy_printed,
                        &mut rng,
                    );
                }
                "beale" => {
                    print_words(
                        BEALE_WORDLIST.as_ref(),
                        &word_num,
                        &delimiter,
                        &is_entropy_printed,
                        &mut rng,
                    );
                }
                "minilock" => {
                    print_words(
                        MINILOCK_WORDLIST.as_ref(),
                        &word_num,
                        &delimiter,
                        &is_entropy_printed,
                        &mut rng,
                    );
                }
                _ => unknown_wordlist(&wordlist_name),
            }
        };
    };
}
