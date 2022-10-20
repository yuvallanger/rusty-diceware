use std::str::FromStr;

include!(concat!(env!("OUT_DIR"), "/diceware.rs"));

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
pub enum Wordlist {
    Beale,
    Reinhold,
    Minilock,
    EffLong,
    EffShort1,
    EffShort2,
}

impl Wordlist {
    pub fn get_list(&self) -> &'static [&'static str] {
        match self {
            Self::Beale => &BEALE_WORDLIST,
            Self::Reinhold => &REINHOLD_WORDLIST,
            Self::Minilock => &MINILOCK_WORDLIST,
            Self::EffLong => &EFF_LONG_WORDLIST,
            Self::EffShort1 => &EFF_SHORT_WORDLIST_1,
            Self::EffShort2 => &EFF_SHORT_WORDLIST_2_0,
        }
    }
}

impl FromStr for Wordlist {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "efflong" => Ok(Self::EffLong),
            "reinhold" => Ok(Self::Reinhold),
            "beale" => Ok(Self::Beale),
            "minilock" => Ok(Self::Minilock),
            "effshort1" => Ok(Self::EffShort1),
            "effshort2" => Ok(Self::EffShort2),
            _ => Err("Unknown Wordlist"),
        }
    }
}

impl Default for Wordlist {
    fn default() -> Self {
        Self::EffLong
    }
}

macro_rules! create_test {
    (
        $wordlist_name: ident,
        $test_name: ident,
        $expected_indices: expr,
        $expected_values: expr,
    ) => {
        #[test]
        fn $test_name() {
            use crate::$wordlist_name;

            let wanted: Vec<&str> = $expected_values.into_iter().collect();

            let got: Vec<&str> = $expected_indices
                .into_iter()
                .map(|i| $wordlist_name[i])
                .collect();

            assert_eq!(got, wanted);
        }
    };
}

create_test!(
    BEALE_WORDLIST,
    beale_test,
    vec![
        0,
        BEALE_WORDLIST.len() - 1,
        490,
        811,
        970,
        1187,
        1726,
        2158,
        2492,
        3005,
        3713,
        4143,
        4289,
    ],
    vec![
        "a", "@", "balls", "boor", "bunts", "cherub", "dime", "ezra", "fuzzy", "hire", "leeway",
        "mills", "ms",
    ],
);

create_test!(
    REINHOLD_WORDLIST,
    reinhold_test,
    vec![
        0,
        REINHOLD_WORDLIST.len() - 1,
        490,
        811,
        970,
        1187,
        1726,
        2158,
        2492,
        3005,
        3713,
        4143,
        4289,
    ],
    vec![
        "a", "@", "balk", "bony", "bum", "charm", "dice", "excess", "fuchs", "hg", "le", "mercy",
        "morsel",
    ],
);

create_test!(
    MINILOCK_WORDLIST,
    minilock_test,
    vec![
        0,
        MINILOCK_WORDLIST.len() - 1,
        7625,
        9287,
        18137,
        26351,
        27726,
        32096,
        37667,
        38151,
        46685,
        47351,
        52997,
    ],
    vec![
        "aardvark",
        "zulus",
        "censer",
        "colonels",
        "exonerate",
        "infringe",
        "jaunts",
        "miserliness",
        "platitudes",
        "pontoon",
        "skua",
        "snuggles",
        "tributes",
    ],
);

create_test!(
    EFF_LONG_WORDLIST,
    eff_long_test,
    vec![
        0,
        EFF_LONG_WORDLIST.len() - 1,
        490,
        811,
        970,
        1187,
        1726,
        2158,
        2492,
        3005,
        3713,
        4143,
        4289,
    ],
    vec![
        "abacus",
        "zoom",
        "barbecue",
        "candle",
        "charbroil",
        "commodity",
        "deserving",
        "eleven",
        "fame",
        "grew",
        "limpness",
        "nifty",
        "outer",
    ],
);
create_test!(
    EFF_SHORT_WORDLIST_1,
    eff_short_1_test,
    vec![
        0,
        EFF_SHORT_WORDLIST_1.len() - 1,
        201,
        250,
        371,
        400,
        565,
        606,
        826,
        890,
        909,
        922,
        966,
    ],
    vec![
        "acid", "zoom", "clerk", "crook", "elbow", "faced", "hut", "kilt", "putt", "rover",
        "sandy", "scary", "shove",
    ],
);
create_test!(
    EFF_SHORT_WORDLIST_2_0,
    eff_short_2_0_test,
    vec![
        0,
        EFF_SHORT_WORDLIST_2_0.len() - 1,
        201,
        250,
        371,
        400,
        565,
        606,
        826,
        890,
        909,
        922,
        966,
    ],
    vec![
        "aardvark",
        "zucchini",
        "circle",
        "cytoplasm",
        "enforcer",
        "etiquette",
        "hybrid",
        "jamboree",
        "omnivorous",
        "plywood",
        "pry",
        "puzzle",
        "riches",
    ],
);
