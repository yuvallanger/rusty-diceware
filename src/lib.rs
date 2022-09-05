extern crate rand;

use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;
use rand::Rng;
use std::fmt;

include!(concat!(env!("OUT_DIR"), "/diceware.rs"));

#[derive(Debug, Clone, Eq, PartialEq, Copy)]
pub struct BealeWord(&'static str);

#[derive(Debug, Clone, Eq, PartialEq, Copy)]
pub struct ReinholdWord(&'static str);

#[derive(Debug, Clone, Eq, PartialEq, Copy)]
pub struct MiniLockWord(&'static str);

pub trait Word {
    fn new(word: &'static str) -> Self;

    fn entropy() -> f64;

    fn entropyn(n: u64) -> f64 {
        Self::entropy() * (n as f64)
    }
}

impl Word for BealeWord {
    fn new(word: &'static str) -> Self {
        Self(word)
    }

    fn entropy() -> f64 {
        (BEALE_WORDLIST.len() as f64).log2()
    }

    fn entropyn(n: u64) -> f64 {
        Self::entropy() * (n as f64)
    }
}

impl Word for ReinholdWord {
    fn new(word: &'static str) -> Self {
        Self(word)
    }

    fn entropy() -> f64 {
        (REINHOLD_WORDLIST.len() as f64).log2()
    }

    fn entropyn(n: u64) -> f64 {
        Self::entropy() * (n as f64)
    }
}

impl Word for MiniLockWord {
    fn new(word: &'static str) -> Self {
        Self(word)
    }

    fn entropy() -> f64 {
        (MINILOCK_WORDLIST.len() as f64).log2()
    }

    fn entropyn(n: u64) -> f64 {
        Self::entropy() * (n as f64)
    }
}

impl rand::distributions::Distribution<BealeWord> for rand::distributions::Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, mut rng: &mut R) -> BealeWord {
        *BEALE_WORDLIST.choose(&mut rng).unwrap()
    }
}

impl fmt::Display for BealeWord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let Self(w) = self;
        write!(f, "{}", w)
    }
}

impl rand::distributions::Distribution<ReinholdWord> for rand::distributions::Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, mut rng: &mut R) -> ReinholdWord {
        *REINHOLD_WORDLIST.choose(&mut rng).unwrap()
    }
}

impl fmt::Display for ReinholdWord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let Self(w) = self;
        write!(f, "{}", w)
    }
}

impl rand::distributions::Distribution<MiniLockWord> for rand::distributions::Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, mut rng: &mut R) -> MiniLockWord {
        *MINILOCK_WORDLIST.choose(&mut rng).unwrap()
    }
}

impl fmt::Display for MiniLockWord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let Self(w) = self;
        write!(f, "{}", w)
    }
}

pub fn print_words<T: Word + std::fmt::Display>(
    word_num: &u64,
    delimiter: &char,
    is_entropy_printed: &bool,
    rng: &mut ThreadRng,
) where
    rand::distributions::Standard: rand::distributions::Distribution<T>,
{
    for _ in 0..(word_num - 1) {
        let word: T = rng.gen();
        print!("{}{}", &word, delimiter);
    }
    let word: T = rng.gen();
    print!("{}", word);

    println!();
    if *is_entropy_printed {
        println!("{}", T::entropyn(*word_num))
    }
}
