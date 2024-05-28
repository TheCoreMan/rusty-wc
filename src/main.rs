mod testing_resources;

use clap::Parser;
use std::fs;
use std::collections::HashMap;

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

    /// Print the count frequency of words in the input files, and print the top 10 most frequent words
    #[arg(short = 'f')]
    should_count_word_frequency: bool,

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
    let should_count_word_frequency: bool;

    let mut should_exit_with_err: bool = false;
    if !parsed_args.should_characters && !parsed_args.should_lines && !parsed_args.should_words {
        // Compat with wc behavior, no flags passed means all these should be on.
        should_characters = true;
        should_lines = true;
        should_words = true;
        should_count_word_frequency = true;
    } else {
        should_characters = parsed_args.should_characters;
        should_lines = parsed_args.should_lines;
        should_words = parsed_args.should_words;
        should_count_word_frequency = parsed_args.should_count_word_frequency;
    }

    let mut total_words: usize = 0;
    let mut total_lines: usize = 0;
    let mut total_characters: usize = 0;
    let mut combined_word_frequency_maps_in_all_contents: HashMap<String, usize> = HashMap::new();

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

        if should_count_word_frequency {
            let word_map_frequency_in_this_content = tokenize_words_in_content(&file_contents);
            let word_map_frequency_in_this_content_clone = word_map_frequency_in_this_content.clone();
            let arranged_word_map_frequency_in_this_content = arrange_hashmap(word_map_frequency_in_this_content);
            pretty_print_a_vector(arranged_word_map_frequency_in_this_content);
            combined_word_frequency_maps_in_all_contents = combine_hashmaps(combined_word_frequency_maps_in_all_contents, word_map_frequency_in_this_content_clone);
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
        if should_count_word_frequency {
            let arranged_word_map_frequency_in_all_contents = arrange_hashmap(combined_word_frequency_maps_in_all_contents);
            pretty_print_a_vector(arranged_word_map_frequency_in_all_contents)
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

fn tokenize_words_in_content(content: &str) -> HashMap<String, usize> {
    let mut word_map_frequency: HashMap<String, usize> = HashMap::new();

    for word in content.split_ascii_whitespace() {
        *word_map_frequency.entry(word.to_string()).or_insert(0) += 1;
    }
    word_map_frequency
}

fn combine_hashmaps(map1: HashMap<String, usize>, map2: HashMap<String, usize>) -> HashMap<String, usize> {
    let mut combined_hashmap = map1;
    for (key, value) in map2 {
        let counter = combined_hashmap.entry(key).or_insert(0);
        *counter += value;
        }
    combined_hashmap
}

fn arrange_hashmap(map: HashMap<String, usize>) -> Vec<(String, usize)> {
    let mut sorted_vec: Vec<(String, usize)> = map.iter().map(|(k, v)| (k.clone(), *v)).collect();
    sorted_vec.sort_by(|a, b| b.1.cmp(&a.1));
    sorted_vec.truncate(10);
    sorted_vec
}

fn pretty_print_a_vector(vector: Vec<(String, usize)> ) {
    print!("\n");
    for (key, value) in vector {
        println!("{} {}", key, value);
    }
}

#[cfg(test)]
mod tests {
    use crate::testing_resources::EXAMPLE_CONTENT_EMPTY;
    use crate::testing_resources::EXAMPLE_CONTENT_FIVE_WORDS;
    use crate::testing_resources::EXAMPLE_CONTENT_TEN_CHARS;
    use crate::testing_resources::EXAMPLE_CONTENT_WITH_FOUR_LINES;
    use crate::testing_resources::EXAMPLE_CONTENT_WITH_FOUR_DISTINCT_WORDS;

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
    fn test_tokenize_words_in_content() {
        let tokenaized = tokenize_words_in_content(EXAMPLE_CONTENT_WITH_FOUR_DISTINCT_WORDS);
        assert_eq!(4, tokenaized.keys().len());
        assert_eq!(&3, tokenaized.get("line").unwrap());
        println!("{:?}", tokenaized);
        assert_eq!(0, count_characters_in_content(EXAMPLE_CONTENT_EMPTY));
    }

}
