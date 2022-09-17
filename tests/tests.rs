extern crate rand;

use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use rand::SeedableRng;

use diceware::wordlists::BEALE_WORDLIST;
use diceware::wordlists::EFF_LONG_WORDLIST;
use diceware::wordlists::EFF_SHORT_WORDLIST_1;
use diceware::wordlists::EFF_SHORT_WORDLIST_2_0;
use diceware::wordlists::MINILOCK_WORDLIST;
use diceware::wordlists::REINHOLD_WORDLIST;

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

create_test!(
    MINILOCK_WORDLIST,
    minilock_rng_test,
    vec!["hoed", "femininity", "bedsit", "stabbings"]
);

create_test!(
    EFF_LONG_WORDLIST,
    eff_long_rng_test,
    vec!["helpless", "fever", "blooming", "sublease"]
);
create_test!(
    EFF_SHORT_WORDLIST_1,
    eff_short_1_rng_test,
    vec!["five", "boat", "shape", "bony"]
);
create_test!(
    EFF_SHORT_WORDLIST_2_0,
    eff_short_2_0_rng_test,
    vec!["family", "aseptic", "renovator", "atlas"]
);
