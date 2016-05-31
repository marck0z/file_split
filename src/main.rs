extern crate file_split;

use std::env;
use std::str::FromStr;

fn main() {
    if env::args().count()!=3{
        panic!("Usage: command FILE_NAME WORD_COUNT");
    }
    let file_path:&str=&env::args().nth(1).expect("invalid FILE_NAME");
    let word_limit:i32=env::args().nth(2).and_then(|x| FromStr::from_str(&x).ok()).expect("invalid WORD_COUNT");
    file_split::util::process_file(file_path,word_limit);
    println!("finished successfully")
}
