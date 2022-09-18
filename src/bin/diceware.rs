extern crate getopts;
extern crate rand;

use std::io::BufRead;
use std::process::exit;

use getopts::Options;
use rand::thread_rng;

use diceware::entropyn;
use diceware::load_wordlist_file;
use diceware::print_words_rng;
use diceware::wordlists::BEALE_WORDLIST;
use diceware::wordlists::EFF_LONG_WORDLIST;
use diceware::wordlists::EFF_SHORT_WORDLIST_1;
use diceware::wordlists::EFF_SHORT_WORDLIST_2_0;
use diceware::wordlists::MINILOCK_WORDLIST;
use diceware::wordlists::REINHOLD_WORDLIST;

fn make_options() -> Options {
    let mut opts = Options::new();
    opts.optflag("h", "help", "This help message.");
    opts.optflag("e", "entropy", "Display number of entropy bits.");
    opts.optflag("r", "dicerolls", "Provide results of physical dice rolls. Word per line, same digit order as in the files, digits between and including 1 and 6.");
    opts.optopt("n", "nword", "Number of words in a passphrase.", "NWORD");
    opts.optopt(
        "d",
        "delimiter",
        "The delimiter character used to separate the words.",
        "DELIM",
    );
    opts.optopt("f", "wordlist-file", "Path to a wordlist file.", "FILE");
    opts.optopt(
        "l",
        "wordlist",
        "Wordlist to use. (efflong (default), effshort1, effshort2, minilock, reinhold, or beale)",
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
        "Unknown wordlist: {}. Available wordlists: efflong (default), effshort1, effshort2, beale, reinhold, or minilock.",
        wordlist_name,
    );
    exit(-1)
}

fn find_number_of_rolls_needed(wordlist_length: u32) -> u32 {
    let mut x = wordlist_length as u64;
    let mut nrolls: u32 = 0;
    while x != 0 {
        x /= 6;
        nrolls += 1;
    }
    nrolls
}

fn rolls_to_word_index(number_of_rolls_needed: u32, rolls: &[u8]) -> usize {
    if number_of_rolls_needed == rolls.len() as u32 {
        let mut word_number = 0;
        for (i, roll) in rolls.iter().rev().enumerate() {
            word_number += (roll
                .checked_sub(1)
                .expect("Must be a die roll result between and including 1 and 6.")
                as usize)
                * 6_usize.pow(i as u32);
        }
        word_number
    } else {
        panic!(
            "Wrong number of die casts: {}. Needs to be {} die casts.",
            rolls.len(),
            number_of_rolls_needed
        )
    }
}

fn print_words_rolls(
    wordlist: &[&str],
    delimiter: &char,
    is_entropy_printed: &bool,
    rolls: &[&[u8]],
) {
    let number_of_rolls_needed = find_number_of_rolls_needed(wordlist.len() as u32);
    for roll_index in 0..(rolls.len() - 1) {
        let roll_sum = rolls_to_word_index(
            number_of_rolls_needed,
            rolls
                .get(roll_index)
                .unwrap_or_else(|| panic!("Bad roll index: {}", roll_index)),
        );
        let word = wordlist
            .get(roll_sum)
            .unwrap_or_else(|| panic!("Wrong word index: {}", roll_sum));
        print!("{}{}", &word, delimiter);
    }
    let roll_sum = rolls_to_word_index(number_of_rolls_needed, rolls.last().unwrap());
    let word = wordlist.get(roll_sum).unwrap();
    print!("{}", &word);

    println!();
    if *is_entropy_printed {
        println!("{}", entropyn(wordlist, rolls.len() as u64))
    }
}

fn read_rolls() -> Vec<Vec<u8>> {
    let stdin = std::io::stdin();
    let mut rolls: Vec<Vec<u8>> = Vec::new();
    let mut last_number_of_dice = None;
    for line in stdin.lock().lines() {
        let line_value = line.unwrap();
        let line_value_trimmed = line_value.trim();
        if line_value_trimmed.is_empty() {
            continue;
        }
        let current_number_of_dice = line_value_trimmed.len();
        if let Some(last_number_of_dice_value) = last_number_of_dice {
            if last_number_of_dice_value != current_number_of_dice {
                panic!("Not all dice rolls were of the same number of die.");
            } else {
                last_number_of_dice = Some(current_number_of_dice);
            }
        }

        rolls.push(
            line_value_trimmed
                .chars()
                .map(|c| {
                    c.to_string()
                        .parse()
                        .unwrap_or_else(|e| panic!("Not a digit: \"{}\". Error: {}", c, e))
                })
                .collect(),
        );
    }
    rolls
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

    let is_physical_rolls = matches.opt_present("r");

    let word_num: u64 = matches
        .opt_str("n")
        .map_or(8, |n_str| n_str.parse::<u64>().ok().unwrap());

    let delimiter: char = matches
        .opt_str("d")
        .map_or(' ', |n_str| n_str.parse::<char>().ok().unwrap());

    let is_entropy_printed = matches.opt_present("entropy");

    let wordlist_name = if let Some(wordlist_option) = matches.opt_str("l") {
        match wordlist_option.to_lowercase().as_ref() {
            z @ ("beale" | "reinhold" | "minilock" | "efflong" | "effshort1" | "effshort2") => z,
            _ => unknown_wordlist(&wordlist_option),
        }
        .to_string()
    } else {
        "efflong".to_string()
    };

    let mut rng = thread_rng();

    if let Some(wordlist_filepath) = matches.opt_str("f") {
        let wordlist_string = load_wordlist_file(&wordlist_filepath);
        let wordlist = wordlist_string
            .split('\n')
            .map(|x| x.trim())
            .filter(|x| x != &"")
            .collect::<Vec<&str>>();

        if is_physical_rolls {
            let rolls = read_rolls();

            print_words_rolls(
                &wordlist,
                &delimiter,
                &is_entropy_printed,
                &rolls.iter().map(|x| x.as_ref()).collect::<Vec<&[u8]>>(),
            );
        } else if word_num != 0 {
            print_words_rng(
                &wordlist,
                &word_num,
                &delimiter,
                &is_entropy_printed,
                &mut rng,
            );
        };
    } else {
        let wordlist = match wordlist_name.as_ref() {
            "efflong" => EFF_LONG_WORDLIST.as_ref(),
            "reinhold" => REINHOLD_WORDLIST.as_ref(),
            "beale" => BEALE_WORDLIST.as_ref(),
            "minilock" => MINILOCK_WORDLIST.as_ref(),
            "effshort1" => EFF_SHORT_WORDLIST_1.as_ref(),
            "effshort2" => EFF_SHORT_WORDLIST_2_0.as_ref(),
            _ => unknown_wordlist(&wordlist_name),
        };

        if is_physical_rolls {
            let rolls = read_rolls();

            print_words_rolls(
                wordlist,
                &delimiter,
                &is_entropy_printed,
                &rolls.iter().map(|x| x.as_ref()).collect::<Vec<&[u8]>>(),
            );
        } else if word_num != 0 {
            print_words_rng(
                wordlist,
                &word_num,
                &delimiter,
                &is_entropy_printed,
                &mut rng,
            );
        };
    };
}
