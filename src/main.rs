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

    // Prints top words freq
    #[arg(short = 'f')]
    should_word_freq: bool,

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
    let mut should_word_freq: bool = false;
    let mut should_exit_with_err: bool = false;
    if !parsed_args.should_characters && !parsed_args.should_lines && !parsed_args.should_words && !parsed_args.should_word_freq {
        // Compat with wc behavior, no flags passed means all these should be on.
        should_characters = true;
        should_lines = true;
        should_words = true;
    } else {
        should_characters = parsed_args.should_characters;
        should_lines = parsed_args.should_lines;
        should_words = parsed_args.should_words;
        should_word_freq = parsed_args.should_word_freq;
    }

    

    let mut total_words: usize = 0;
    let mut total_lines: usize = 0;
    let mut total_characters: usize = 0;
    let mut words_freq: HashMap<String, i32> = HashMap::new();
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
        if should_word_freq {
            update_word_freq(&file_contents, &mut words_freq)
        }
        println!(" {}", path)
    }
    // Now if more than 1 path, print total
    if parsed_args.paths.len() > 1 && !should_word_freq {
        if should_lines {
            print!("{:>8}", total_lines);
        }
        if should_words {
            print!("{:>8}", total_words);
        }
        if should_characters {
            print!("{:>8}", total_characters);
        }
        println!(" total");
    }
    
    if should_word_freq {
        let mut freq_vec: Vec<_> = words_freq.iter().collect();
        
        freq_vec.sort_by(|a, b| b.1.cmp(&a.1));

        for (word, &count) in freq_vec.iter().take(10) {
            println!("{:>4} {}", count, word);
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

fn update_word_freq(content: &str, freq_map: &mut HashMap<String, i32>) {
    for word in content.split_ascii_whitespace(){
        *freq_map.entry(word.to_string()).or_insert(0) += 1
    }
}

#[cfg(test)]
mod tests {
    use crate::testing_resources::EXAMPLE_CONTENT_EMPTY;
    use crate::testing_resources::EXAMPLE_CONTENT_FIVE_WORDS;
    use crate::testing_resources::EXAMPLE_CONTENT_TEN_CHARS;
    use crate::testing_resources::EXAMPLE_CONTENT_WITH_FOUR_LINES;
    use crate::testing_resources::WORD_FREQ_SAMPLE_TEXT_1;
    use crate::testing_resources::WORD_FREQ_SAMPLE_TEXT_2;

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
    fn test_update_word_freq() {
        let mut freq_map: HashMap<String, i32> = HashMap::new();
        update_word_freq(WORD_FREQ_SAMPLE_TEXT_1, &mut freq_map);
        assert_eq!(*freq_map.get("hello").unwrap(), 3);
        assert_eq!(*freq_map.get("world").unwrap(), 2);
        assert_eq!(*freq_map.get("test").unwrap(), 2);
    }

    #[test]
    fn test_print_top_words() {
        let mut freq_map: HashMap<String, i32> = HashMap::new();
        update_word_freq(WORD_FREQ_SAMPLE_TEXT_2, &mut freq_map);

        let mut output = Vec::new();
        let mut freq_vec: Vec<_> = freq_map.iter().collect();
        freq_vec.sort_by(|a, b| b.1.cmp(&a.1));

        for (word, &count) in freq_vec.iter().take(3) {
            output.push(format!("{:>4} {}", count, word));
        }

        assert_eq!(output, vec!["   4 test", "   3 example", "   1 hello"]);
    }
}

