extern crate rand;

use rand::{Rng, SeedableRng, StdRng};

use diceware::{ReinholdWord, BealeWord};

fn make_beale_word() -> BealeWord {
    let seed: &[_] = &[1, 2, 3, 4];
    let mut rng: StdRng = SeedableRng::from_seed(seed);
    let word = rng.gen();
    word
}

fn make_reinhold_word() -> ReinholdWord {
    let seed: &[_] = &[1, 2, 3, 4];
    let mut rng: StdRng = SeedableRng::from_seed(seed);
    let word = rng.gen();
    word
}

#[test]
fn beale_rng_test() {
    let rand_word = make_beale_word();
    assert_eq!(rand_word, BealeWord::new("ladder"))
}

#[test]
fn reinhold_rng_test() {
    let rand_word = make_reinhold_word();
    assert_eq!(rand_word, ReinholdWord::new("ks"))
}
