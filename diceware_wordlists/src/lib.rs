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
