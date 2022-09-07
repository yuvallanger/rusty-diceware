extern crate rand;

use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;

include!(concat!(env!("OUT_DIR"), "/diceware.rs"));

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
