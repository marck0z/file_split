# Text File Splitter

Command line utility to split large text files into multiple small files by number of words

### Installing

```
cargo build --release
```

## Usage

Running the following command will create a directory with the resulting files inside
```
file_split FILE_TO_SPLIT WORD_COUNT [REPEATED_WORDS]
```

* FILE_TO_SPLIT path to the text file to split
* WORD_COUNT each resulting file will have at most this many words
* REPEATED_WORDS optional, this is the number of the last words from the previous file which will be inserted at the beginning of the next one, defaults to 10
