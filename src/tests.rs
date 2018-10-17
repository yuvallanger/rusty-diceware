extern crate rand;

use rand::{Rng, SeedableRng, StdRng};

use diceware::{BealeWord, ReinholdWord};

fn make_beale_vector() -> Vec<BealeWord> {
    let seed: [u8; 32] = [0; 32];
    let mut rng: StdRng = SeedableRng::from_seed(seed);

    let mut beale_vector: Vec<BealeWord> = vec![];
    for _ in 0..4 {
        let word: BealeWord = rng.gen();
        beale_vector.push(word);
    }
    beale_vector
}

fn make_reinhold_vector() -> Vec<ReinholdWord> {
    let seed: [u8; 32] = [0; 32];
    let mut rng: StdRng = SeedableRng::from_seed(seed);

    let mut reinhold_vector: Vec<ReinholdWord> = vec![];
    for _ in 0..4 {
        let word: ReinholdWord = rng.gen();
        reinhold_vector.push(word);
    }
    reinhold_vector
}

#[test]
fn beale_rng_test() {
    let wanted: Vec<BealeWord> = vec!["dr", "raced", "pvc", "moon"]
        .into_iter()
        .map(BealeWord::new)
        .collect();

    let got = make_beale_vector();

    assert_eq!(got, wanted);
}

#[test]
fn reinhold_rng_test() {
    let wanted: Vec<ReinholdWord> = vec!["douse", "qo", "prune", "moan"]
        .into_iter()
        .map(ReinholdWord::new)
        .collect();

    let got = make_reinhold_vector();

    assert_eq!(got, wanted);
}
