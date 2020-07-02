use std::fs::File;
use std::io::prelude::*;
use clap::App;

mod args;

fn main() -> std::io::Result<()> {
    // Get command line arguments using clap
    let matches = App::new("word_replacement")
        .version("1.2")
        .author("Timothy Tso <trhksrc@protonmail.ch>")
        .arg("-f, --filename=[FILE] 'Sets the text file'")
        .arg("-w, --words...=[replacing_word][replacement_word] 'Sets the two words to be used to process the text'")
        .get_matches();

    // Get and process individual arguments
    let filename: String = matches.value_of("filename").unwrap().to_string();
    let mut words: Vec<_> = matches.values_of("words").unwrap().collect();

    // Instantiate the args struct containing the three arguments
    let args_list = args::Args {
        filename: filename,
        replacing_word: words.remove(0).to_string(),
        replacement_word: words.remove(0).to_string(),
    };

    // Read text file
    let mut txt_file = File::open(args_list.filename)?;
    let mut txt = String::new();
    txt_file.read_to_string(&mut txt)?;

    // Split the text into a list of the words
    let txt_list: Vec<&str> = txt.split_whitespace().collect();

    // Loop through the text list and replace every replacing word with the replacement word
    let mut proc_txt_list = Vec::new();

    for word in &txt_list {
        if word == &args_list.replacing_word {
            proc_txt_list.push(args_list.replacement_word.to_owned());
        } else {
            proc_txt_list.push(word.to_string());
        }
    }

    // Put the processed words into a single String
    let mut proc_txt = String::new();

    for word in &proc_txt_list {
        proc_txt.push_str(word);
        proc_txt.push_str(" ");
    }

    // println!("{}", &proc_txt);

    // Write the processed contents into a new text file
    let mut replaced_file = File::create("replaced.txt")?;
    replaced_file.write_all(proc_txt.as_bytes())?;

    Ok(())
}
