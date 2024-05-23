mod testing_resources;
mod frequency;

use std::collections::HashMap;
use clap::Parser;
use std::fs;

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

    /// Print the frequency of words in each input file
    #[arg(short = 'f')]
    should_words_frequency: bool,

    /// Paths to input files we want to `wc`. If more than one input file is
    /// specified, a line of cumulative counts for all the files is displayed
    /// on a separate line after the output for the last file.
    paths: Vec<String>,
}

fn main() {
    let parsed_args = Args::parse();
    let should_words: bool;
    let should_words_frequency: bool;
    let should_lines: bool;
    let should_characters: bool;
    let mut should_exit_with_err: bool = false;

    if parsed_args.should_words_frequency {
        should_words_frequency = true;
        should_characters = false;
        should_lines = false;
        should_words = false;
    } else if !parsed_args.should_characters && !parsed_args.should_lines && !parsed_args.should_words {
        // Compat with wc behavior, no flags passed means all these should be on.
        should_characters = true;
        should_lines = true;
        should_words = true;
        should_words_frequency = false;
    } else {
        should_characters = parsed_args.should_characters;
        should_lines = parsed_args.should_lines;
        should_words = parsed_args.should_words;
        should_words_frequency = false;
    }

    let mut total_words: usize = 0;
    let mut total_lines: usize = 0;
    let mut total_characters: usize = 0;
    let mut total_words_frequency: HashMap<String, usize> = HashMap::new();

    for path in parsed_args.paths.iter() {
        let file_contents = match fs::read_to_string(path) {
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
        if should_words_frequency {
            let (word_frequency_in_this_content, word_freq) = frequency::count_frequency_of_words_in_content(&file_contents);
            merge_word_freq(&mut total_words_frequency, &word_freq);
            print!("{}", word_frequency_in_this_content);
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
        if should_words_frequency {
            print!("{}", frequency::frequency_of_words_to_string(&total_words_frequency));
        }
        println!(" total")
    }
    if should_exit_with_err {
        std::process::exit(0x00000001);
    }
}

fn merge_word_freq(total: &mut HashMap<String, usize>, iteration_result: &HashMap<String, usize>) {
    for (word, count) in iteration_result.iter() {
        let total_count = total.entry(word.into()).or_insert(0);
        *total_count += count;
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

#[cfg(test)]
mod tests {
    use crate::frequency::count_frequency_of_words_in_content;
    use crate::testing_resources::{EXAMPLE_CONTENT_EMPTY, EXAMPLE_FREQUENCY_CONTENT_WITH_FOUR_LINES};
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
    fn test_count_frequency_of_words_in_content() {
        let (res, _) = count_frequency_of_words_in_content(EXAMPLE_CONTENT_WITH_FOUR_LINES);
        assert_eq!(EXAMPLE_FREQUENCY_CONTENT_WITH_FOUR_LINES, res);
    }
}
