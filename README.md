# rusty-diceware - a password generator using wordlists

Commandline [Diceware][diceware] ([Wayback Machine mirror][diceware-wayback]), with or without dice, written in [rustlang][rustlang].

Please use [Gitlab][gitlab-mirror] for anything whatsoever. Github is just a mirror.

[CHANGELOG here](/CHANGELOG.md).

Inspired by the great passphrase generating solution [Diceware][diceware] ([Wayback Machine mirror][diceware-wayback]) invented by [Arnold G. Reinhold][arnold] ([Wayback Machine mirror][arnold-wayback]) and by Randall Monroe’s [xkcd#936][xkcd-936]:

![“Hidden” alt text jokes are a pain in the ass.](/bin/imgs.xkcd.com/comics/password_strength.png)

## Usage:

```
Usage: diceware [options]

Options:
    -h, --help          This help message.
    -e, --entropy       Display number of entropy bits.
    -r, --dicerolls     Provide results of physical dice rolls. Word per line,
                        same digit order as in the files, digits between and
                        including 1 and 6.
    -n, --nword NWORD   Number of words in a passphrase.
    -d, --delimiter DELIM
                        The delimiter character used to separate the words.
    -f, --wordlist-file FILE
                        Path to a wordlist file.
    -l, --wordlist WORDLIST
                        Wordlist to use. (efflong (default), effshort1,
                        effshort2, minilock, reinhold, or beale)
```

## Featuring

* The three wordlists mentioned in EFF's [Diceware Guide][eff-diceware-guide]:
    * [EFF Long Wordlist][eff-long-wordlist].
    * [EFF Short Wordlist #1][eff-short-wordlist-1].
    * [EFF Short Wordlist #2][eff-short-wordlist-2-0].
* The original [Reinhold wordlist][reinhold-wordlist-asc] ([Wayback Machine mirror][reinhold-wordlist-asc-wayback]).
* The [Beale wordlist][beale-wordlist-asc] ([Wayback Machine mirror][beale-wordlist-asc-wayback]).
* The [MiniLock][minilock] ([github][minilock-github])wordlist. (found in the [phrase.js][minilock-phrase-js] file)
* The all new `--wordlist-file` command line option which loads and uses your very own newline delimited wordlist file. Inquire within!
* Physical dice roll! You can (don't use echo, it will show up in `ps` and show your rolls to other users):

    ```
    $ cat | diceware -l efflong -r
    111111
    111112
    ^D
    abacus abdomen
    ```
* Wordlists live in their own [diceware_wordlists][diceware_wordlists] crate you can use for your own wicked deeds.

## Mirrors

* [Gitlab][gitlab-mirror] is the main repository.
* [Github][github-mirror] is just a mirror.

## Say hello, chat, and/or lurch, aka "community"

You can say hello and/or rant about how terrible rusty-diceware is in the [#rusty-diceware:matrix.org](https://matrix.to/#/#rusty-diceware:matrix.org) chat room.

## Tips

If you want to tip for this work, I have set up a Liberapay account:

<script src="https://liberapay.com/yuvallanger/widgets/button.js"></script>
<noscript><a href="https://liberapay.com/yuvallanger/donate"><img alt="Donate using Liberapay" src="https://liberapay.com/assets/widgets/donate.svg"></a></noscript>

[gitlab-mirror]: <https://gitlab.com/yuvallanger/rusty-diceware/>
[github-mirror]: <https://github.com/yuvallanger/rusty-diceware/>

[diceware_wordlists]: <https://crates.io/crates/diceware_wordlists>

[eff-diceware-guide]: <https://www.eff.org/dice>
[eff-long-wordlist]: <https://www.eff.org/files/2016/07/18/eff_large_wordlist.txt>
[eff-short-wordlist-1]: <https://www.eff.org/files/2016/09/08/eff_short_wordlist_1.txt>
[eff-short-wordlist-2-0]: <https://www.eff.org/files/2016/09/08/eff_short_wordlist_2_0.txt>

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
