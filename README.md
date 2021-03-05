# Word Replacement

A simple CLI for replacing all occurences of a word with a different word, using rust.

## Example

```
cd word_example

echo 'the quick brown fox jumps over the lazy dog' > example.txt

cargo run -- --filename example.txt --words the this
```

Inside the `word_example` directory, there should be a file called `replaced.txt` that should contain the contents:

`this quick brown fox jumps over this lazy dog`

## Usage

`word_replacement [OPTIONS]`

Use `-- --help` for help regarding options for how to use the command line arguments.

```
USAGE:
    word_replacement [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -f, --filename <FILE>                              Sets the text file
    -w, --words <replacing_word> <replacement_word>    Sets the two words to be used to process the text
```
