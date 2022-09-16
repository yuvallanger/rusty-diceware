# rusty-diceware - a password generator using the diceware word lists
Commandline [Diceware][diceware] ([Wayback Machine mirror][diceware-wayback]),
sans dice, written in [rustlang][rustlang].

Please use [Gitlab][gitlab-mirror] for anything whatsoever. Github is just a
mirror.

[CHANGELOG](/CHANGELOG.md) here.

Inspired by the great passphrase generating solution [Diceware][diceware]
([Wayback Machine mirror][diceware-wayback]) invented by [Arnold G.
Reinhold][arnold] ([Wayback Machine mirror][arnold-wayback]) and by Randall
Monroe’s [xkcd#936][xkcd-936]:

![“Hidden” alt text jokes are a pain in the
ass.](/bin/imgs.xkcd.com/comics/password_strength.png)

## Usage:

```
Usage: diceware [options]

Options:
    -h, --help          this help message
        --minilock      [OBSOLETE] use the MiniLock wordlist (default)
        --reinhold      [OBSOLETE] use the standard wordlist
        --beale         [OBSOLETE] use the beale wordlist
    -e, --entropy       display number of entropy bits
    -n, --nword NWORD   number of words in a passphrase
    -d, --delimiter DELIM
                        the delimiter character used to separate the words
    -f, --wordlist-file FILE
                        path to a wordlist file
    -l, --wordlist WORDLIST
                        Wordlist to use (minilock (default), reinhold, or
                        beale)
```

## Featuring

* The original [Reinhold wordlist][reinhold-wordlist-asc] ([Wayback Machine
  mirror][reinhold-wordlist-asc-wayback]).
* The [Beale wordlist][beale-wordlist-asc] ([Wayback Machine
  mirror][beale-wordlist-asc-wayback]).
* The [MiniLock][minilock] ([github][minilock-github])wordlist. (found in the
  [phrase.js][minilock-phrase-js] file)
* The all new `--wordlist-file` command line option which loads and uses your very own newline delimited wordlist file. Inquire within!

## Mirrors

* [Gitlab][gitlab-mirror] is the main repository.
* [Github][github-mirror] is just a mirror.


[gitlab-mirror]: <https://gitlab.com/yuvallanger/rusty-diceware/>
[github-mirror]: <https://github.com/yuvallanger/rusty-diceware/>

[arnold]: <https://theworld.com/~reinhold/>
[diceware]: <https://theworld.com/~reinhold/diceware.html>

[arnold-wayback]: <https://web.archive.org/web/20220608141106/https://theworld.com/~reinhold/>
[diceware-wayback]: <https://web.archive.org/web/20220817092807/https://theworld.com/~reinhold/diceware.html>

[beale-wordlist-asc]: <https://theworld.com/~reinhold/beale.wordlist.asc>
[reinhold-wordlist-asc]: <https://theworld.com/~reinhold/diceware.wordlist.asc>
[minilock-phrase-js]: <https://github.com/kaepora/miniLock/blob/71dcf431886068c9ec7f563c3e4158153229b202/src/js/lib/phrase.js>

[beale-wordlist-asc-wayback]: <https://web.archive.org/web/20220602072646/https://theworld.com/~reinhold/beale.wordlist.asc>
[reinhold-wordlist-asc-wayback]: <https://web.archive.org/web/20220820102521/https://theworld.com/~reinhold/diceware.wordlist.asc>


[rustlang]: <http://rust-lang.org>

[xkcd-936]: <https://www.explainxkcd.com/wiki/index.php/936>

[minilock]: <http://minilock.io>
[minilock-github]: <https://github.com/kaepora/miniLock/>
