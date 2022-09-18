use std::env;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::string::String;

fn make_diceware_wordlist(array_name: String) -> impl Fn(&str) -> String {
    move |contents: &str| {
        // 7776 words = 6*6*6*6*6; five 6 faced dice throws.
        format!("pub(crate) static {}: [&str; 7776] = [\n", array_name)
            + &contents
                .split('\n')
                .skip(2)
                .take(6 * 6 * 6 * 6 * 6)
                .map(|s| s.split_once('\t').unwrap().1)
                .map(|s| {
                    s.chars()
                        .map(|c| {
                            if c == '"' {
                                "\\\"".to_owned()
                            } else {
                                c.to_string()
                            }
                        })
                        .collect::<String>()
                })
                .map(|s| format!("    \"{}\",\n", s))
                .collect::<String>()
            + "];\n\n"
    }
}

fn make_minilock_wordlist(contents: &str) -> std::string::String {
    // 58110 words in the MiniLock wordlist.
    "pub(crate) static MINILOCK_WORDLIST: [&str; 58110] = [\n".to_owned()
        + &contents[1023..543718]
            .split(',')
            .map(|x| format!("    \"{}\",\n", x))
            .collect::<String>()
        + "];\n"
}

fn make_eff_long_wordlist(contents: &str) -> String {
    // 7776 words = 6*6*6*6*6; five 6 faced dice throws.
    "pub(crate) static EFF_LONG_WORDLIST: [&str; 7776] = [\n".to_owned()
        + &contents
            .split('\n')
            .take(6 * 6 * 6 * 6 * 6)
            .map(|x| x.split('\t').nth(1).unwrap())
            .map(|x| format!("    \"{}\",\n", x))
            .collect::<String>()
        + "];\n"
}

fn make_eff_short_wordlist(array_name: String) -> impl Fn(&str) -> String {
    move |contents: &str| {
        // 1296 words = 6*6*6*6; five 6 faced dice throws.
        format!("pub(crate) static {}: [&str; 1296] = [\n", array_name)
            + &contents
                .split('\n')
                .take(6 * 6 * 6 * 6)
                .map(|x| x.split('\t').nth(1).unwrap())
                .map(|x| format!("    \"{}\",\n", x))
                .collect::<String>()
            + "];\n"
    }
}

fn build_wordlist(
    wordlist_file_path: &str,
    destination_file: &mut File,
    make_wordlist: &dyn Fn(&str) -> String,
) {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    println!("{}", manifest_dir);
    let wordlist_file_path = Path::new(&manifest_dir).join(wordlist_file_path);

    let mut wordlist_file = File::open(&wordlist_file_path).unwrap();

    let mut wordlist_string = String::new();

    wordlist_file.read_to_string(&mut wordlist_string).unwrap();

    destination_file
        .write_all(make_wordlist(&wordlist_string).as_bytes())
        .unwrap();
}

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("diceware.rs");
    let mut destination_file = File::create(&dest_path).unwrap();

    build_wordlist(
        "bin/wordlists/beale.wordlist.asc",
        &mut destination_file,
        &make_diceware_wordlist("BEALE_WORDLIST".to_string()),
    );
    build_wordlist(
        "bin/wordlists/diceware.wordlist.asc",
        &mut destination_file,
        &make_diceware_wordlist("REINHOLD_WORDLIST".to_string()),
    );
    build_wordlist(
        "bin/wordlists/phrase.js",
        &mut destination_file,
        &make_minilock_wordlist,
    );
    build_wordlist(
        // They call it "EFF's Long Wordlist" on https://www.eff.org/dice, not "EFF's Large
        // Wordlist", although the file name is different, so I will call it "long" rather than
        // "large".
        "bin/wordlists/www.eff.org/files/2016/07/18/eff_large_wordlist.txt",
        &mut destination_file,
        &make_eff_long_wordlist,
    );
    build_wordlist(
        "bin/wordlists/www.eff.org/files/2016/09/08/eff_short_wordlist_1.txt",
        &mut destination_file,
        &make_eff_short_wordlist("EFF_SHORT_WORDLIST_1".to_string()),
    );
    build_wordlist(
        "bin/wordlists/www.eff.org/files/2016/09/08/eff_short_wordlist_2_0.txt",
        &mut destination_file,
        &make_eff_short_wordlist("EFF_SHORT_WORDLIST_2_0".to_string()),
    );
}
