extern crate rand;

use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;
use rand::Rng;

include!(concat!(env!("OUT_DIR"), "/diceware.rs"));

pub trait Word {
    fn new(word: &'static str) -> Self;

    fn entropy() -> f64;

    fn entropyn(n: u64) -> f64 {
        Self::entropy() * (n as f64)
    }
}

macro_rules! create_generator {
    ( $gen_name:ident, $wordlist: expr ) => {
        #[derive(Debug, Clone, Eq, PartialEq, Copy)]
        pub struct $gen_name(&'static str);
        impl Word for $gen_name {
            fn new(word: &'static str) -> Self {
                $gen_name(word.clone())
            }

            fn entropy() -> f64 {
                ($wordlist.len() as f64).log2()
            }

            fn entropyn(n: u64) -> f64 {
                Self::entropy() * (n as f64)
            }
        }

        impl std::fmt::Display for $gen_name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl rand::distributions::Distribution<$gen_name> for rand::distributions::Standard {
            fn sample<R: rand::Rng + ?Sized>(&self, mut rng: &mut R) -> $gen_name {
                *$wordlist.choose(&mut rng).unwrap()
            }
        }
    };
}

create_generator!(BealeWord, BEALE_WORDLIST);
create_generator!(ReinholdWord, REINHOLD_WORDLIST);
create_generator!(MiniLockWord, MINILOCK_WORDLIST);

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
