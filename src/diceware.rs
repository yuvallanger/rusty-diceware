extern crate rand;

use rand::seq::SliceRandom;
use std::fmt;

include!(concat!(env!("OUT_DIR"), "/diceware.rs"));

#[derive(Debug, Clone, Eq, PartialEq, Copy)]
pub struct BealeWord(&'static str);

#[derive(Debug, Clone, Eq, PartialEq, Copy)]
pub struct ReinholdWord(&'static str);

#[derive(Debug, Clone, Eq, PartialEq, Copy)]
pub struct MiniLockWord(&'static str);

impl BealeWord {
    #[cfg(test)]
    pub fn new(word: &'static str) -> BealeWord {
        BealeWord(word.clone())
    }

    pub fn entropy() -> f64 {
        (BEALE_WORDLIST.len() as f64).log2()
    }

    pub fn entropyn(n: u64) -> f64 {
        BealeWord::entropy() * (n as f64)
    }
}

impl ReinholdWord {
    #[cfg(test)]
    pub fn new(word: &'static str) -> ReinholdWord {
        ReinholdWord(word.clone())
    }

    pub fn entropy() -> f64 {
        (REINHOLD_WORDLIST.len() as f64).log2()
    }

    pub fn entropyn(n: u64) -> f64 {
        ReinholdWord::entropy() * (n as f64)
    }
}

impl MiniLockWord {
    #[cfg(test)]
    pub fn new(word: &'static str) -> MiniLockWord {
        MiniLockWord(word.clone())
    }

    pub fn entropy() -> f64 {
        (MINILOCK_WORDLIST.len() as f64).log2()
    }

    pub fn entropyn(n: u64) -> f64 {
        MiniLockWord::entropy() * (n as f64)
    }
}

impl rand::distributions::Distribution<BealeWord> for rand::distributions::Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, mut rng: &mut R) -> BealeWord {
        *BEALE_WORDLIST.choose(&mut rng).unwrap()
    }
}

/*
impl rand::Rand for BealeWord {
    fn rand<R: rand::Rng>(rng: &mut R) -> BealeWord {
        rng.choose(&BEALE_WORDLIST).unwrap().clone()
    }
}
*/

impl fmt::Display for BealeWord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let BealeWord(w) = self;
        write!(f, "{}", w)
    }
}

impl rand::distributions::Distribution<ReinholdWord> for rand::distributions::Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, mut rng: &mut R) -> ReinholdWord {
        *REINHOLD_WORDLIST.choose(&mut rng).unwrap()
    }
}

/*
impl rand::Rand for ReinholdWord {
    fn rand<R: rand::Rng>(rng: &mut R) -> ReinholdWord {
        rng.choose(&REINHOLD_WORDLIST).unwrap().clone()
    }
}
*/

impl fmt::Display for ReinholdWord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ReinholdWord(w) = self;
        write!(f, "{}", w)
    }
}

impl rand::distributions::Distribution<MiniLockWord> for rand::distributions::Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, mut rng: &mut R) -> MiniLockWord {
        *MINILOCK_WORDLIST.choose(&mut rng).unwrap()
    }
}

/*
impl rand::Rand for MiniLockWord {
    fn rand<R: rand::Rng>(rng: &mut R) -> MiniLockWord {
        rng.choose(&MINILOCK_WORDLIST).unwrap().clone()
    }
}
*/

impl fmt::Display for MiniLockWord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let MiniLockWord(w) = self;
        write!(f, "{}", w)
    }
}
