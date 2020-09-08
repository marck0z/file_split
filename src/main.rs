extern crate file_split;

use std::env;
use std::path::Path;
use std::process::exit;

fn main() {
    if env::args().count() < 3 {
        println!("Usage: file_split FILE_TO_SPLIT WORD_COUNT [REPEATED_WORDS]");
        exit(0)
    }

    let mut args = env::args();
    //first argument usually is the executable path, ignored
    args.next();

    //second argument is the file name
    let file_path = args.next().expect("invalid FILE_NAME");
    let file_path = Path::new(&file_path);
    if !file_path.is_file() {
        panic!("can't read file: {}", file_path.display());
    }

    //third argument is the word count
    let word_limit = args.next().expect("missing WORD_COUNT");
    let word_limit: u32 = word_limit.parse().expect("invalid WORD_COUNT");

    //fourth argument is the number of words to repeat at the beginning of the next file
    let repeated_words: u32 = args.next()
        .map(|x| x.parse().expect("invalid REPEATED_WORDS")).unwrap_or(10);

    file_split::util::split_file(file_path, word_limit, repeated_words);
    println!("finished successfully")
}
