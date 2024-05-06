mod testing_resources;

use clap::Parser;
use std::{fs, collections::HashMap};

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

    /// Print most frequent words in each input file
    #[arg(short = 'f')]
    should_frequency: bool,
    /// Paths to input files we want to `wc`. If more than one input file is
    /// specified, a line of cumulative counts for all the files is displayed
    /// on a separate line after the output for the last file.
    paths: Vec<String>,
}

fn main() {
    let parsed_args = Args::parse();
    let should_words: bool;
    let should_lines: bool;
    let should_frequency: bool;
    let should_characters: bool;
    let mut should_exit_with_err: bool = false;

    // make sure `should_frequency` is exclusive with other flags
    if parsed_args.should_frequency {
        if parsed_args.should_characters || parsed_args.should_lines || parsed_args.should_words {
            eprintln!("wc: -f is exclusive with -l, -w, -c");
            return
        }
    }

    if !parsed_args.should_characters && !parsed_args.should_lines && !parsed_args.should_words {
        // Compat with wc behavior, no flags passed means all these should be on.
        should_characters = true;
        should_lines = true;
        should_words = true;
    } else {
        should_characters = parsed_args.should_characters;
        should_lines = parsed_args.should_lines;
        should_words = parsed_args.should_words;
    }

    let mut total_words: usize = 0;
    let mut total_lines: usize = 0;
    let mut total_characters: usize = 0;
    for path in parsed_args.paths.iter() {
        let file_contents = match fs::read_to_string(path.clone()) {
            Ok(x) => x,
            Err(e) => {
                eprint!("wc: {}: {}", path, e.to_string());
                should_exit_with_err = true;
                continue;
            }
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
        println!(" {}", path)
    }
    // Now if more than 1 path, print total
    if parsed_args.paths.len() > 1 {
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

// solution is not right, it should return just the mapping and do the sorting and limiting in main
fn count_word_frequencies(content: &str) -> String {
    let mut word_freqs: HashMap<String, usize> = HashMap::new();
    for word in content.split_ascii_whitespace() {
        let count = word_freqs.entry(word.to_string()).or_insert(0);
        *count += 1;
    }
    let mut sorted_word_freqs: Vec<(&String, &usize)> = word_freqs.iter().collect();
    sorted_word_freqs.sort_by(|a, b| b.1.cmp(a.1).then(a.0.cmp(b.0)));

    let length = if sorted_word_freqs.len() < 10 {
        sorted_word_freqs.len()
    } else {
        10
    };
    let mut result = String::new();
    for i in 0..length {
        let (word, _) = sorted_word_freqs[i];
        result.push_str(&format!("{}\n", word));
    }
   result
}

#[cfg(test)]
mod tests {
    use crate::testing_resources::EXAMPLE_CONTENT_EMPTY;
    use crate::testing_resources::EXAMPLE_CONTENT_FIVE_WORDS;
    use crate::testing_resources::EXAMPLE_CONTENT_MANY_WORDS;
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
    fn test_word_frequency() {
        assert_eq!(
            "asdf\nasdf!\n",
            count_word_frequencies(EXAMPLE_CONTENT_TEN_CHARS)
        );
        assert_eq!(
            "one\ntwo\n1\n2\n3\n5\n6\n7\n8\n9\n",
            count_word_frequencies(EXAMPLE_CONTENT_MANY_WORDS)
        );
    }
}
