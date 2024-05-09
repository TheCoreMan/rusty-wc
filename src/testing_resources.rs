#[cfg(test)]
pub const EXAMPLE_CONTENT_WITH_FOUR_LINES: &str = "line one
line two
line three
line four
";

#[cfg(test)]
pub const EXAMPLE_CONTENT_EMPTY: &str = "";

#[cfg(test)]
pub const EXAMPLE_CONTENT_FIVE_WORDS: &str = "My name is Alexander
Hamilton";

#[cfg(test)]
pub const EXAMPLE_CONTENT_TEN_CHARS: &str = "asdf
asdf!";

#[cfg(test)]
mod tests {
    use crate::update_word_freq;
    use std::collections::HashMap;

    // You might already have similar constants in your testing_resources module
    const SAMPLE_TEXT: &str = "hello world hello world hello test test";
    const SAMPLE_TEXT_MORE: &str = "example example example test test test test hello";

    #[test]
    fn test_update_word_freq() {
        let mut freq_map: HashMap<String, i32> = HashMap::new();
        update_word_freq(SAMPLE_TEXT, &mut freq_map);
        assert_eq!(*freq_map.get("hello").unwrap(), 3);
        assert_eq!(*freq_map.get("world").unwrap(), 2);
        assert_eq!(*freq_map.get("test").unwrap(), 2);
    }

    #[test]
    fn test_print_top_words() {
        let mut freq_map: HashMap<String, i32> = HashMap::new();
        update_word_freq(SAMPLE_TEXT_MORE, &mut freq_map);

        let mut output = Vec::new();
        let mut freq_vec: Vec<_> = freq_map.iter().collect();
        freq_vec.sort_by(|a, b| b.1.cmp(&a.1));

        for (word, &count) in freq_vec.iter().take(3) {
            output.push(format!("{:>4} {}", count, word));
        }

        assert_eq!(output, vec!["   4 test", "   3 example", "   1 hello"]);
    }
}
