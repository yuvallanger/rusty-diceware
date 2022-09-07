extern crate rand;

use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use rand::SeedableRng;

use diceware::BEALE_WORDLIST;
use diceware::REINHOLD_WORDLIST;

macro_rules! create_test {
    ( $wordlist_name: path, $test_name: ident, $expected: expr ) => {
        #[test]
        fn $test_name() {
            fn make_vektor<'a>() -> Vec<&'a str> {
                let seed: [u8; 32] = [0; 32];
                let mut rng: StdRng = SeedableRng::from_seed(seed);

                let mut vector: Vec<&str> = vec![];
                for _ in 0..4 {
                    let word: &str = $wordlist_name.choose(&mut rng).unwrap();
                    vector.push(word);
                }
                vector
            }
            let wanted: Vec<&str> = $expected.into_iter().collect();

            let got = make_vektor();

            assert_eq!(got, wanted);
        }
    };
}

create_test!(
    BEALE_WORDLIST,
    beale_rng_test,
    vec!["io", "gavel", "beam", "time"]
);

create_test!(
    REINHOLD_WORDLIST,
    reinhold_rng_test,
    vec!["india", "gamma", "bcd", "theme"]
);
