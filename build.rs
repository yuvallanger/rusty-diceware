use std::env;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::string::String;

fn make_wordlist(contents: &str) -> Vec<&str> {
    contents
        .split('\n')
        .skip(2)
        .take(7776)
        .map(|s| s.split_once('\t').unwrap().1)
        .collect()
}

fn make_beale_wordlist(contents: &str) -> std::string::String {
    let wordlist = make_wordlist(contents);

    let mut output = std::string::String::new();

    // 7776 words = 6*6*6*6*6; five 6 faced dice throws.
    output.push_str("pub static BEALE_WORDLIST: [&str; 7776] = [\n");
    for word in wordlist {
        output.push_str("    \"");
        for c in word.chars() {
            if c == '"' {
                output.push_str("\\\"");
            } else {
                output.push(c);
            }
        }
        output.push_str("\",\n");
    }
    output.push_str("];\n\n");
    output
}

fn make_reinhold_wordlist(contents: &str) -> std::string::String {
    let wordlist = make_wordlist(contents);

    let mut output = std::string::String::new();

    // 7776 words = 6*6*6*6*6; five 6 faced dice throws.
    output.push_str("pub static REINHOLD_WORDLIST: [&str; 7776] = [\n");
    for word in wordlist {
        output.push_str("    \"");
        for c in word.chars() {
            if c == '"' {
                output.push_str("\\\"");
            } else {
                output.push(c);
            }
        }
        output.push_str("\",\n");
    }
    output.push_str("];\n");
    output
}

fn make_minilock_wordlist(contents: &str) -> std::string::String {
    let wordlist: Vec<&str> = (&contents[1023..543718]).split(',').collect();

    let mut output = std::string::String::new();

    // 58110 words in the MiniLock wordlist.
    output.push_str("pub static MINILOCK_WORDLIST: [&str; 58110] = [\n");
    for word in wordlist {
        output.push_str("    \"");
        for c in word.chars() {
            if c == '"' {
                panic!("Not supposed to have any double quotes.");
            } else {
                output.push(c);
            }
        }
        output.push_str("\",\n");
    }
    output.push_str("];\n");
    output
}

fn build_wordlist(
    wordlist_filename: &str,
    destination_file: &mut File,
    make_wordlist_struct: &dyn Fn(&str) -> String,
) {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    println!("{}", manifest_dir);
    let wordlists_dir = Path::new(&manifest_dir).join("bin").join("wordlists");

    let mut wordlist_file = File::open(&wordlists_dir.join(wordlist_filename)).unwrap();

    let mut wordlist_string = String::new();

    wordlist_file.read_to_string(&mut wordlist_string).unwrap();

    destination_file
        .write_all(make_wordlist_struct(&wordlist_string).as_bytes())
        .unwrap();
}

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("diceware.rs");
    let mut destination_file = File::create(&dest_path).unwrap();

    build_wordlist(
        "beale.wordlist.asc",
        &mut destination_file,
        &make_beale_wordlist,
    );
    build_wordlist(
        "diceware.wordlist.asc",
        &mut destination_file,
        &make_reinhold_wordlist,
    );
    build_wordlist("phrase.js", &mut destination_file, &make_minilock_wordlist);
}
