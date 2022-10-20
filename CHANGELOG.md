# v0.5.8

* Move tests into tests directory.
* Use a reproducible `rand_chacha::ChaCha12Rng` instead of an `rand::prelude::StdRng`. In the current `rand` version, `0.8.5`, `StdRng` is `ChaCha12Rng`, but it may change in the future, as written in <https://docs.rs/rand/0.8.5/rand/rngs/struct.StdRng.html>.

# v0.5.7

Bumping `diceware_wordlists` version in the dependencies.

## v0.5.4 to v0.5.6

* Move wordlists to their own permissively licensed `diceware_wordlists` crate.

## v0.5.3

* Check for legal die digit (not less than 1 not more than 6).

## v0.5.2

* Move code from `/src/bin/diceware.rs`, `/tests/tests.rs`, and `/src/lib.rs` into `/src/main.rs`.

## v0.5.1

* Add physical dice roll input flag (`-r` / `--dicerolls`). Accepts input from standard input.

## v0.5.0

* Add the EFF wordlists as shown in <https://www.eff.org/dice>.
* Make EFF Long Wordlist the default.
* Remove the `--beale` / `--reinhold` / `--minilock` flags.

## v0.4.0

* Add the `-l` / `--wordlist` option.
* Warn that the `--beale` / `--reinhold` / `--minilock` flags would be deprecated in the next minor version.

## v0.3.9

* Move some code from main function to its own function in `src/lib.rs`.
* Remove vestigial naming used when we were stupidly overusing the type system.
* Add test for the MiniLock wordlist.
* Move predefined wordlists to their own `diceware::wordlists` module.
* Add a note about the `--wordlist-file` commandline option.

## v0.3.8

* Switch from a the old and very very weird Struct definition per wordlist to the braindead simple array of `&str`s.
* Add a command line option that takes a filepath to a newline delimited wordlist file so that you could use your own wordlist.

## v0.3.7

* Deduplicate code implementing the `Word` trait using macros. (Thanks @vbrandl!)
* Deduplicate tests code using macros. (Thanks @vbrandl, again!)
* Deduplicate `build.rs` code using a function. How not exciting.

## v0.3.6

* No, really, fix tests. (Added the `Word` trait)

## v0.3.5

* Fix tests. (Was broken due to a changed ThreadRng output)

## v0.3.4

* Deduplicate word printing code by using the trait `Word` and moving that code into a a generic function which uses that trait.
* Move to 2021 edition of Rustlang.  Add a commented out wordlist command line option for a future version.
* Remove some commented out code.
* Use the `Self` keyword in `impl`s instead of the type's concrete name.
* Add this CHANGELOG.
