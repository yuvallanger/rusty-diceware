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
