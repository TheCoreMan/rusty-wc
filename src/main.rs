mod testing_resources;

use clap::Parser;
use std::{collections::HashMap, fs};

/// wc impl in rust
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Print the number of lines in each input file
    #[arg(short = 'l')]
    should_lines: bool,

    /// Print the number of bytes in each input file
    #[arg(short = 'c')]
    should_characters: bool,

    /// Print the number of words in each input file
    #[arg(short = 'w')]
    should_words: bool,

    /// Print the top 10 frequent words of each input file
    #[arg(short = 'f')]
    should_frequent: bool,

    /// Paths to input files we want to `wc`. If more than one input file is
    /// specified, a line of cumulative counts for all the files is displayed
    /// on a separate line after the output for the last file.
    paths: Vec<String>,
}

fn main() {
    let parsed_args: Args = Args::parse();
    let should_words: bool;
    let should_lines: bool;
    let should_characters: bool;
    let should_frequent: bool;
    let mut should_exit_with_err: bool = false;
    if !parsed_args.should_frequent {
        if !parsed_args.should_characters && !parsed_args.should_lines && !parsed_args.should_words {
            // Compat with wc behavior, no flags passed means all these should be on.
            should_characters = true;
            should_lines = true;
            should_words = true;
            should_frequent = false;
        } else {
            should_characters = parsed_args.should_characters;
            should_lines = parsed_args.should_lines;
            should_words = parsed_args.should_words;
            should_frequent = false;
        }
    } else {
        should_frequent = parsed_args.should_frequent;
        should_characters = false;
        should_lines = false;
        should_words = false;
    }

    let mut total_words: usize = 0;
    let mut total_lines: usize = 0;
    let mut total_characters: usize = 0;
    let mut frequent_words: HashMap<String, i32> = HashMap::new();
    for path in parsed_args.paths.iter() {
        let file_contents = match read_file(path, &mut should_exit_with_err) {
            Some(value) => value,
            None => continue,
        };
        if should_lines {
            let lines_in_this_content = count_lines_in_content(&file_contents);
            total_lines += lines_in_this_content;
            print!("{:>8}", lines_in_this_content);
        }
        if should_words {
            let words_in_this_content = count_words_in_content(&file_contents);
            total_words += words_in_this_content;
            print!("{:>8}", words_in_this_content);
        }
        if should_characters {
            let characters_in_this_content = count_characters_in_content(&file_contents);
            total_characters += characters_in_this_content;
            print!("{:>8}", characters_in_this_content);
        }
        if should_frequent {
            aggregate_words_in_content(&file_contents, &mut frequent_words);
        }
        println!("path: {}", path);
    }
    if parsed_args.should_frequent {
        let mut hash_vec: Vec<(&String, &i32)> = frequent_words.iter().collect();
            hash_vec.sort_by(|a, b| b.1.cmp(a.1));
            for i in 0..10 {
                println!("{:?}", hash_vec[i])
            }
    }
    // Now if more than 1 path, print total
    if parsed_args.paths.len() > 1 && !parsed_args.should_frequent{
        if should_lines {
            print!("{:>8}", total_lines);
        }
        if should_words {
            print!("{:>8}", total_words);
        }
        if should_characters {
            print!("{:>8}", total_characters);
        }
        println!(" total")
    }
    if should_exit_with_err {
        std::process::exit(0x00000001);
    }
}

fn read_file(path: &String, should_exit_with_err: &mut bool) -> Option<String> {
    let file_contents = match fs::read_to_string(path.clone()) {
        Ok(x) => x,
        Err(e) => {
            eprint!("wc: {}: {}", path, e.to_string());
            *should_exit_with_err = true;
            return None;
        }
    };
    Some(file_contents)
}

fn count_lines_in_content(content: &str) -> usize {
    // My initial implementation
    // content.split('\n').fold(0, |lines: u64, _x| lines + 1)
    // Easier way, still wrong
    // content.split('\n').count()
    // Apparently, wc counts `\n` in content, not lines
    content.match_indices('\n').count()
}

fn count_characters_in_content(content: &str) -> usize {
    content.chars().count()
}

fn count_words_in_content(content: &str) -> usize {
    content.split_ascii_whitespace().count()
}

fn aggregate_words_in_content(file_contents: &str, frequent_words: &mut HashMap<String, i32>) {
    for word in file_contents.split_ascii_whitespace() {
        let mutable_word: &mut String = &mut word.to_string().replace("\n", "");
        let count: &mut i32 = frequent_words.entry(mutable_word.to_string()).or_insert(0);
        *count += 1;
    }
}

#[cfg(test)]
mod tests {
    use crate::testing_resources::EXAMPLE_CONTENT_EMPTY;
    use crate::testing_resources::EXAMPLE_CONTENT_FIVE_WORDS;
    use crate::testing_resources::EXAMPLE_CONTENT_TEN_CHARS;
    use crate::testing_resources::EXAMPLE_CONTENT_WITH_FOUR_LINES;

    use super::*;

    #[test]
    fn test_count_lines_in_content() {
        assert_eq!(4, count_lines_in_content(EXAMPLE_CONTENT_WITH_FOUR_LINES));
        assert_eq!(0, count_lines_in_content(EXAMPLE_CONTENT_EMPTY));
    }

    #[test]
    fn test_count_words_in_content() {
        assert_eq!(5, count_words_in_content(EXAMPLE_CONTENT_FIVE_WORDS));
        assert_eq!(0, count_words_in_content(EXAMPLE_CONTENT_EMPTY));
    }

    #[test]
    fn test_count_characters_in_content() {
        assert_eq!(10, count_characters_in_content(EXAMPLE_CONTENT_TEN_CHARS));
        assert_eq!(0, count_characters_in_content(EXAMPLE_CONTENT_EMPTY));
    }

    #[test]
    fn test_license_words_frequency_in_content() {
        let mut should_exit_with_err: bool = false;
        let file_contents = match read_file(&"LICENSE".to_string(), &mut should_exit_with_err) {
            Some(value) => value,
            None => "".to_string(),
        };
        let mut frequent_words: HashMap<String, i32> = HashMap::new();
        aggregate_words_in_content(&file_contents, &mut frequent_words);
        let expected_license_word_frequency:HashMap<&str, i32> = HashMap::from([
            ("the", 309),
            ("of", 208),
            ("to", 174),
            ("a", 165),
            ("or", 131),
            ("you", 102),
            ("that", 89),
            ("and", 86),
            ("this", 72),
            ("in", 70)
        ]);
        for word in expected_license_word_frequency.keys() {
            assert_eq!(expected_license_word_frequency.get(word), frequent_words.get(word.clone()))
        };
    }

    #[test]
    fn test_license_contributing_words_frequency_in_content() {
        let mut should_exit_with_err: bool = false;
        let mut frequent_words: HashMap<String, i32> = HashMap::new();
        for path in ["LICENSE", "CONTRIBUTING.md"]{
            let file_contents = match read_file(&path.to_string(), &mut should_exit_with_err) {
                Some(value) => value,
                None => continue,
            };
            aggregate_words_in_content(&file_contents, &mut frequent_words);
        }
        let expected_license_word_frequency:HashMap<&str, i32> = HashMap::from([
            ("the", 318),
            ("of", 208),
            ("to", 182),
            ("a", 168),
            ("or", 131),
            ("you", 104),
            ("that", 93),
            ("and", 88),
            ("this", 72),
            ("for", 70)
        ]);
        for word in expected_license_word_frequency.keys() {
            assert_eq!(expected_license_word_frequency.get(word), frequent_words.get(word.clone()))
        };
    }
}
