use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

const BEALE_CONTENTS: &'static str = include_str!("bin/wordlists/beale.wordlist.asc");
const REINHOLD_CONTENTS: &'static str = include_str!("bin/wordlists/diceware.wordlist.asc");

fn make_wordlist(contents: &str) -> Vec<&str> {
    contents.split('\n')
        .skip(2)
        .take(7776)
        .map(|s| s.splitn(2, '\t').nth(1).unwrap())
        .collect()
}

fn make_beale_struct(wordlist: Vec<&str>) -> std::string::String {
    let mut output = std::string::String::new();
    output.push_str("const BEALE_WORDLIST: [BealeWord; 7776] = [\n");
    for word in wordlist {
        output.push_str("    BealeWord(\"");
        for c in word.chars() {
            if c == '"' {
                output.push_str("\\\"");
            } else {
                output.push(c);
            }
        }
        output.push_str("\"),\n");
    }
    output.push_str("];\n");
    return output;
}

fn make_reinhold_struct(wordlist: Vec<&str>) -> std::string::String {
    let mut output = std::string::String::new();
    output.push_str("const REINHOLD_WORDLIST: [ReinholdWord; 7776] = [\n");
    for word in wordlist {
        output.push_str("    ReinholdWord(\"");
        for c in word.chars() {
            if c == '"' {
                output.push_str("\\\"");
            } else {
                output.push(c);
            }
        }
        output.push_str("\"),\n");
    }
    output.push_str("];\n");
    return output;
}
fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("diceware.rs");
    let mut f = File::create(&dest_path).unwrap();

    f.write_all(
        make_beale_struct(make_wordlist(BEALE_CONTENTS)).as_bytes()
    ).unwrap();

    f.write_all(
        make_reinhold_struct(make_wordlist(REINHOLD_CONTENTS)).as_bytes()
    ).unwrap();
}
