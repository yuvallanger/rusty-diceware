extern crate rand;

use rand::rngs::StdRng;
use rand::Rng;
use rand::SeedableRng;

use diceware::BealeWord;
use diceware::ReinholdWord;
use diceware::Word;

macro_rules! create_test {
    ( $gen_name: ident, $test_name: ident, $expected: expr ) => {
        #[test]
        fn $test_name() {
            fn make_vektor() -> Vec<$gen_name> {
                let seed: [u8; 32] = [0; 32];
                let mut rng: StdRng = SeedableRng::from_seed(seed);

                let mut vector: Vec<$gen_name> = vec![];
                for _ in 0..4 {
                    let word: $gen_name = rng.gen();
                    vector.push(word);
                }
                vector
            }
            let wanted: Vec<$gen_name> = $expected.into_iter().map($gen_name::new).collect();

            let got = make_vektor();

            assert_eq!(got, wanted);
        }
    };
}

create_test!(
    BealeWord,
    beale_rng_test,
    vec!["io", "gavel", "beam", "time"]
);

create_test!(
    ReinholdWord,
    reinhold_rng_test,
    vec!["india", "gamma", "bcd", "theme"]
);
