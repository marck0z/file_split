use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

/// Will read the file in `path` and write its contents into multiple files
/// each with at most `word_limit` words
///
/// Each file will also contain the last 10 words of the previous file by default
pub fn split_file(path: &Path, word_limit: u32, repeated_words:u32) {

    // Read the file contents into a string
    let mut file: File = File::open(path)
        .expect(&format!("couldn't open file: {}", path.display()));
    let mut contents = String::new();
    match file.read_to_string(&mut contents)
    {
        Err(why) => panic!("couldn't read {}: {}", path.display(), why),
        Ok(bytes) => println!("read {} bytes", bytes),
    }

    //create dir
    let file_name = path.file_stem().expect("invalid file name");
    let absolute_path = fs::canonicalize(path).unwrap();
    let parent_dir = absolute_path.parent().unwrap();
    if !parent_dir.is_dir() {
        panic!("invalid parent directory: {}", parent_dir.display())
    }
    let current_dir = parent_dir.join(file_name);
    fs::create_dir(&current_dir)
        .expect("can't create dir");

    //count words, create files
    let mut i = 0;
    let mut file_number = 1;
    let mut buf = String::with_capacity((word_limit * 4) as usize);
    let mut last_words = String::new();
    for line in contents.lines() {
        let words = line.split_whitespace();
        for word in words {
            buf.push_str(word);
            buf.push(' ');

            //save the last 10 words to use in the next file
            if i > word_limit - repeated_words {
                last_words.push_str(word);
                last_words.push(' ');
            }

            if i < word_limit {
                i += 1;
            } else {
                save_file(&current_dir, file_number, file_name.to_str().unwrap(), &buf);
                buf.clear();
                buf.push_str(&last_words);
                last_words.clear();
                file_number += 1;
                i = 0;
            }
        }
        buf.push('\r');
        buf.push('\n');
    }
}

/// Saves the contents in a file with a custom name
/// # Arguments
///
/// * `dir` - The directory where this file will be saved
/// * `file_number` - The index of the file to be included in its name
/// * `file_name` - The name of the original file
/// * `text` - Text of the new file
fn save_file(dir: &Path, file_number: i32, file_name: &str, text: &String) {
    //create the name of the new file
    let new_file_name = if file_number < 10 {
        format!("0{}-{}", file_number, file_name)
    } else {
        format!("{}-{}", file_number, file_name)
    };
    let new_file = dir.join(new_file_name);

    // Open a file in write-only mode
    let mut file = match File::create(&new_file) {
        Err(why) => panic!("couldn't create {}: {}", new_file.display(), why),
        Ok(file) => file,
    };

    match file.write_all(text.as_bytes()) {
        Err(why) => {
            panic!("couldn't write to {}: {}", new_file.display(), why)
        }
        Ok(_) => println!("successfully wrote to {}", new_file.display()),
    }
}