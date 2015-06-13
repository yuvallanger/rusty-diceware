use std::env;
use std::fs::File;
use std::io::Write;
use std::io::Read;
use std::path::Path;


fn make_wordlist(contents: &std::string::String) -> Vec<&str> {
    contents.split('\n')
        .skip(2)
        .take(7776)
        .map(|s| s.splitn(2, '\t').nth(1).unwrap())
        .collect()
}

fn make_minilock_wordlist(contents: &std::string::String) -> Vec<&str> {
    contents.split(',')
        .skip(3)
        .take(58109)
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
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    println!("{}", manifest_dir);
    let wordlists_dir = Path::new(&manifest_dir).join("bin").join("wordlists");
    let mut beale_wordlist_file = File::open(&wordlists_dir.join("beale.wordlist.asc")).unwrap();
    let mut reinhold_wordlist_file = File::open(&wordlists_dir.join("diceware.wordlist.asc")).unwrap();
    let mut minilock_wordlist_file = File::open(&wordlists_dir.join("phrase.js"));

    let mut beale_wordlist_string = String::new();
    let mut reinhold_wordlist_string = String::new();
    let mut minilock_wordlist_string = String::new();

    beale_wordlist_file.read_to_string(&mut beale_wordlist_string);
    reinhold_wordlist_file.read_to_string(&mut reinhold_wordlist_string);
    minilock_wordlist_file.read_to_string(&mut minilock_wordlist_string);

    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("diceware.rs");
    let mut f = File::create(&dest_path).unwrap();

    f.write_all(
        make_beale_struct(make_wordlist(&beale_wordlist_string)).as_bytes()
    ).unwrap();

    f.write_all(
        make_reinhold_struct(make_wordlist(&reinhold_wordlist_string)).as_bytes()
    ).unwrap();

    f.write_all(
        make_minilock_struct(make_minilock_wordlist(&minilock_wordlist_string)).as_bytes()
    ).unwrap();
}
