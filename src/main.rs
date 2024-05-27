mod testing_resources;

use clap::Parser;
use indexmap::IndexMap;
use std::fs;

/// wc impl in rust
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Print the number of lines in each input file
    #[arg(short = 'l', conflicts_with = "should_frequency")]
    should_lines: bool,

    /// Print the number of bytes in each input file
    #[arg(short = 'c', conflicts_with = "should_frequency")]
    should_characters: bool,

    /// Print the number of words in each input file
    #[arg(short = 'w', conflicts_with = "should_frequency")]
    should_words: bool,

    /// Print the top 10 most frequent words in all input files
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
    let should_characters: bool;
    let should_frequency: bool;
    let mut should_exit_with_err: bool = false;

    if !parsed_args.should_characters
        && !parsed_args.should_lines
        && !parsed_args.should_words
        && !parsed_args.should_frequency
    {
        // Compat with wc behavior, no flags passed means all these should be on.
        // Except for `should_frequency`
        should_characters = true;
        should_lines = true;
        should_words = true;
        should_frequency = false;
    } else {
        should_characters = parsed_args.should_characters;
        should_lines = parsed_args.should_lines;
        should_words = parsed_args.should_words;
        should_frequency = parsed_args.should_frequency;
    }

    let mut total_words: usize = 0;
    let mut total_lines: usize = 0;
    let mut total_characters: usize = 0;
    let mut top_frequent_words: IndexMap<String, usize> = IndexMap::new();

    for path in parsed_args.paths.iter() {
        let file_contents = match fs::read_to_string(path) {
            Ok(x) => x,
            Err(e) => {
                eprint!("wc: {}: {}", path, e.to_string());
                should_exit_with_err = true;
                continue;
            }
        };

        if should_frequency {
            for w in (file_contents).split_ascii_whitespace() {
                increment_word_frequency(w, &mut top_frequent_words);
            }
            continue;
        }
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

    if should_frequency {
        // Print top 10 words across all paths
        let top_frequent_words_sorted: Vec<(String, usize)> =
            sort_word_frequency_descending(top_frequent_words);
        for (word, value) in &top_frequent_words_sorted[..10] {
            println!("{:>8} {}", value, word);
        }
    }
    // Now if more than 1 path, print total
    if parsed_args.paths.len() > 1 && !should_frequency {
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
    // In case of an invalid path
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

fn increment_word_frequency(word: &str, top_frequent_words: &mut IndexMap<String, usize>) {
    match top_frequent_words.get(word) {
        Some(count) => {
            top_frequent_words.insert(word.to_string(), count + 1);
        }
        // If a word is being encountered for the very first time
        None => {
            top_frequent_words.insert(word.to_string(), 1);
        }
    }
}

fn sort_word_frequency_descending(
    top_frequent_words: IndexMap<String, usize>,
) -> Vec<(String, usize)> {
    let mut top_frequent_words_sorted: Vec<(String, usize)> =
        top_frequent_words.into_iter().collect();
    top_frequent_words_sorted.sort_by(|a, b| b.1.cmp(&a.1));
    top_frequent_words_sorted
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
}
