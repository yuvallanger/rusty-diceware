use std::str::FromStr;

include!(concat!(env!("OUT_DIR"), "/diceware.rs"));

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
pub enum Wordlist {
    #[cfg(feature = "beale")]
    Beale,
    #[cfg(feature = "reinhold")]
    Reinhold,
    #[cfg(feature = "minilock")]
    Minilock,
    #[cfg(feature = "efflong")]
    EffLong,
    #[cfg(feature = "effshort1")]
    EffShort1,
    #[cfg(feature = "effshort2")]
    EffShort2,
}

impl Wordlist {
    pub fn get_list(&self) -> &'static [&'static str] {
        match self {
            #[cfg(feature = "beale")]
            Self::Beale => &BEALE_WORDLIST,
            #[cfg(feature = "reinhold")]
            Self::Reinhold => &REINHOLD_WORDLIST,
            #[cfg(feature = "minilock")]
            Self::Minilock => &MINILOCK_WORDLIST,
            #[cfg(feature = "efflong")]
            Self::EffLong => &EFF_LONG_WORDLIST,
            #[cfg(feature = "effshort1")]
            Self::EffShort1 => &EFF_SHORT_WORDLIST_1,
            #[cfg(feature = "effshort2")]
            Self::EffShort2 => &EFF_SHORT_WORDLIST_2_0,
        }
    }
}

impl FromStr for Wordlist {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            #[cfg(feature = "efflong")]
            "efflong" => Ok(Self::EffLong),
            #[cfg(feature = "reinhold")]
            "reinhold" => Ok(Self::Reinhold),
            #[cfg(feature = "beale")]
            "beale" => Ok(Self::Beale),
            #[cfg(feature = "minilock")]
            "minilock" => Ok(Self::Minilock),
            #[cfg(feature = "effshort1")]
            "effshort1" => Ok(Self::EffShort1),
            #[cfg(feature = "effshort2")]
            "effshort2" => Ok(Self::EffShort2),
            _ => Err("Unknown Wordlist"),
        }
    }
}

// XXX If the `efflong` feature is not chosen, which wordlist should be next in line?
// #[cfg(feature = "efflong")]
impl Default for Wordlist {
    fn default() -> Self {
        Self::EffLong
    }
}
