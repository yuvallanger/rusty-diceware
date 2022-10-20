use rand_chacha;

macro_rules! create_test {
    ( $wordlist_name: ident, $test_name: ident, $expected: expr ) => {
        #[test]
        fn $test_name() {
            use diceware_wordlists::$wordlist_name;
            use rand::prelude::SeedableRng;
            use rand::prelude::SliceRandom;
            use rand_chacha::ChaCha12Rng;

            fn make_vektor<'a>() -> Vec<&'a str> {
                let seed: [u8; 32] = [0; 32];
                let mut rng: ChaCha12Rng = SeedableRng::from_seed(seed);

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
