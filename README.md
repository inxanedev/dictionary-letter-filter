# dictionary-letter-filter
Console utility allowing for filtering a list of words by only allowing certain letters.

# usage

    Usage: dictionary-letter-filter [options]

    Options:
        -h, --help            displays help page
        -f, --file FILE       dictionary file path
        -l, --letters LETTERS list of acceptable letters
        -u, --url URL         URL to fetch dictionary from
        -o, --output OUTPUT   output file path
        -w, --words           output only the words
        -c, --count           output only the amount of words

# examples

## Filtering a word list for words that only contain the letters abcdef:

    $ dictionary-letter-filter -f wordlist.txt -l abcdef

## Filtering a word list for words that only contain the letters abcdef and output to a file:

    $ dictionary-letter-filter -f wordlist.txt -l abcdef -o output.txt

## Filtering a word list for words that only contain the letters abcdef and output only the amount of words:

    $ dictionary-letter-filter -f wordlist.txt -l abcdef -c

## Filtering a word list from a URL

    $ dictionary-letter-filter -u https://raw.githubusercontent.com/dwyl/english-words/master/words_alpha.txt -l abcdef