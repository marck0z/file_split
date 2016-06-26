use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

/// Takes a file and creates a directory with the same name
/// inside of it creates as many files as needed each, containing only
/// a determined number of words 'word_limit'
pub fn process_file(file_path: &str, word_limit: i32) {
    // Create a path to the desired file
    let path = Path::new(file_path);
    let file_name=path.file_name().and_then(|x| x.to_str()).expect("invalid file name");
    //let file_name=path.file_name().expect("no file name").to_str().unwrap_or("invalid_name");
    let mut file_number=1;
    let dir_name;

    //obtain dir name
    {
        let mut index=0;
        for (i, c) in file_name.chars().enumerate() {
            if c == '.' {
                index=i;
                break;
            }
        }
        if index>0 {
            dir_name = &file_name[0..index];
        }
        else {
            dir_name=&file_name;
        }
    }

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file:File = match File::open(&path) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't open {}: {}", path.display(), why.description()),
        Ok(file) => {println!("opened OK");file},
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut contents = String::new();
    match file.read_to_string(&mut contents)
    {
        Err(why) => panic!("couldn't read {}: {}", path.display(), why.description()),
        Ok(_) => println!("{} read ok", path.display()),
    }

    //create dir
    let parent = path.parent().and_then(|x| x.to_str()).expect("invalid parent dir");
    let dir_path = if parent.char_indices().count() == 1 {
        format!("{}{}", parent, dir_name)
    } else {
        format!("{}/{}", parent, dir_name)
    };
    println!("dir:{}", dir_path);
    if !Path::new(&dir_path).exists() {
        fs::create_dir(&dir_path).unwrap();
    }

    //separate lines, count words
    let mut tmp=String::new();
    let mut word_count=0;
    for line in contents.lines(){
        word_count=word_count+line.split_whitespace().count() as i32;
        tmp.push_str(line);
        //println!("i:{} word:{}",i,word);
        if word_count>=word_limit {
            save_file(&dir_path, file_number,file_name,&tmp);
            file_number=file_number+1;
            word_count=0;
            tmp.clear();
            tmp.push_str(line); //previous last line
        }
        tmp.push('\n');
    }
    if !tmp.is_empty() {
        save_file(&dir_path, file_number,file_name,&tmp);
    }

    // `file` goes out of scope, and the "hello.txt" file gets closed
}

/// Saves the contents in a file with a custom made name
fn save_file(dir: &str, file_number: i32, file_name: &str, new_contents: &String){
    let new_file_name=if file_number<10 {format!("0{}-{}",file_number,file_name)} else {format!("{}-{}",file_number,file_name)};
    let path=format!("{}/{}",dir, new_file_name);

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}",
                           path,
                           why.description()),
        Ok(file) => file,
    };

    match file.write_all(new_contents.as_bytes()) {
        Err(why) => {
            panic!("couldn't write to {}: {}", path, why.description())
        },
        Ok(_) => println!("successfully wrote to {}", path),
    }
}