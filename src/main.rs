mod testing_resources;

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

    /// Print the top 10 most frequent words and their counts
    #[arg(short = 'f')]
    should_frequency: bool,

    /// Paths to input files we want to `wc`. If more than one input file is
    /// specified, a line of cumulative counts for all the files is displayed
    /// on a separate line after the output for the last file.
    paths: Vec<String>,
}

impl Args {
    fn validate(&self) -> Result<(), String> {
        if self.should_frequency
            && (self.should_lines || self.should_characters || self.should_words)
        {
            return Err(String::from("Options -l, -c, -w cannot be used with -f"));
        }
        Ok(())
    }
}

fn main() {
    let parsed_args = Args::parse();
    let should_words: bool;
    let should_lines: bool;
    let should_characters: bool;
    let should_frequency: bool;
    let mut should_exit_with_err: bool = false;
    const TOP_FREQ: usize = 10;

    if let Err(err) = parsed_args.validate() {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }

    should_frequency = parsed_args.should_frequency;
    if !parsed_args.should_frequency
        && !parsed_args.should_characters
        && !parsed_args.should_lines
        && !parsed_args.should_words
    {
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
    let mut total_words_freq: std::collections::HashMap<String, usize> =
        std::collections::HashMap::new();

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
        if should_frequency {
            count_word_frequencies_in_content(&file_contents, &mut total_words_freq);
        }

        println!(" {}", path)
    }
    // Since we want a combined count, we take the frequency count here
    if should_frequency {
        for (count, word) in top_words(total_words_freq, TOP_FREQ) {
            println!(" {:>8} {}", count, word);
        }
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
        if !should_frequency {
            println!(" total")
        }
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

// Case sensitive: different than the common case of "word frequency" but consistent with functionality described in README.md
fn count_word_frequencies_in_content(
    content: &str,
    word_freqs: &mut std::collections::HashMap<String, usize>,
) {
    for word in content.split_ascii_whitespace() {
        *word_freqs.entry(word.to_string()).or_insert(0) += 1;
    }
}

fn top_words(
    total_words_freq: std::collections::HashMap<String, usize>,
    k: usize,
) -> Vec<(usize, String)> {
    let mut word_heap: std::collections::BinaryHeap<(usize, String)> = total_words_freq
        .into_iter()
        .map(|(word, count)| (count, word))
        .collect();
    let mut result = Vec::new();
    for _ in 0..k {
        word_heap.pop().map(|(count, word)| {
            result.push((count, word));
        });
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::testing_resources::EXAMPLE_CONTENT_EMPTY;
    use crate::testing_resources::EXAMPLE_CONTENT_FIVE_WORDS;
    use crate::testing_resources::EXAMPLE_CONTENT_LICENSE;
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
    fn test_word_frequencies_in_content() {
        let mut expected = std::collections::HashMap::new();
        let mut total_words_freq: std::collections::HashMap<String, usize> =
            std::collections::HashMap::new();
        expected.insert("line".to_string(), 4);
        expected.insert("one".to_string(), 1);
        expected.insert("two".to_string(), 1);
        expected.insert("three".to_string(), 1);
        expected.insert("four".to_string(), 1);
        count_word_frequencies_in_content(EXAMPLE_CONTENT_WITH_FOUR_LINES, &mut total_words_freq);
        assert_eq!(expected, total_words_freq);
    }

    #[test]
    fn test_top_words_in_content() {
        let expected = std::vec![
            (29, "the".to_string()),
            (28, "to".to_string()),
            (19, "of".to_string()),
            (18, "you".to_string()),
            (13, "that".to_string()),
            (12, "and".to_string()),
            (10, "for".to_string()),
            (9, "is".to_string()),
            (8, "software".to_string()),
            (8, "it".to_string())
        ];
        let mut total_words_freq: std::collections::HashMap<String, usize> =
            std::collections::HashMap::new();
        count_word_frequencies_in_content(EXAMPLE_CONTENT_LICENSE, &mut total_words_freq);
        assert_eq!(expected, top_words(total_words_freq, 10));
    }
}
