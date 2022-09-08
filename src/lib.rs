extern crate rand;

use std::fs::File;
use std::io::Read;

use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;

pub mod wordlists {
    include!(concat!(env!("OUT_DIR"), "/diceware.rs"));
}

fn entropy(wordlist: &[&str]) -> f64 {
    (wordlist.len() as f64).log2()
}

fn entropyn(wordlist: &[&str], n: u64) -> f64 {
    entropy(wordlist) * (n as f64)
}

pub fn print_words(
    wordlist: Vec<&str>,
    word_num: &u64,
    delimiter: &char,
    is_entropy_printed: &bool,
    rng: &mut ThreadRng,
) {
    for _ in 0..(word_num - 1) {
        let word = wordlist.choose(rng).unwrap();
        print!("{}{}", &word, delimiter);
    }
    let word = wordlist.choose(rng).unwrap();
    print!("{}", word);

    println!();
    if *is_entropy_printed {
        println!("{}", entropyn(&wordlist, *word_num))
    }
}

pub fn load_wordlist_file(filepath: &str) -> String {
    let mut wordlist_file = match File::open(&filepath) {
        Ok(ok) => ok,
        Err(err) => panic!("Unable to open file: {}; due to error: {}", filepath, err),
    };
    let mut wordlist_string = String::new();
    if let Err(err) = wordlist_file.read_to_string(&mut wordlist_string) {
        panic!("Unable to read file: {}; due to error: {}", filepath, err)
    }
    wordlist_string
}
