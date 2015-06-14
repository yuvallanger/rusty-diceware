extern crate rand;

use std::fmt;

include!(concat!(env!("OUT_DIR"), "/diceware.rs"));

#[derive(Debug,Clone,Eq,PartialEq)]
pub struct BealeWord(&'static str);

#[derive(Debug,Clone,Eq,PartialEq)]
pub struct ReinholdWord(&'static str);

#[derive(Debug,Clone,Eq,PartialEq)]
pub struct MiniLockWord(&'static str);

impl BealeWord {
    pub fn new(word: &'static str) -> BealeWord {
        BealeWord(word.clone())
    }
}

impl ReinholdWord {
    pub fn new(word: &'static str) -> ReinholdWord {
        ReinholdWord(word.clone())
    }
}

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

impl rand::Rand for MiniLockWord {
    fn rand<R: rand::Rng>(rng: &mut R) -> MiniLockWord {
        rng.choose(&MINILOCK_WORDLIST).unwrap().clone()
    }
}

impl fmt::Display for MiniLockWord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
	    &MiniLockWord(w) => write!(f, "{}", w)
        }
    }
}
