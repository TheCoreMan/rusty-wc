use std::collections::HashMap;

pub fn count_frequency_of_words_in_content(content: &str) -> (String, HashMap<String, usize>) {
    let mut word_freq: HashMap<String, usize> = HashMap::new();
    for word in content.split_ascii_whitespace() {
        let count = word_freq.entry(word.into()).or_insert(0);
        *count += 1;
    }

    (frequency_of_words_to_string(&word_freq), word_freq)
}

pub fn frequency_of_words_to_string(word_freq: &HashMap<String, usize>) -> String {
    let mut sorted_word_freq = word_freq.iter().collect::<Vec<_>>();
    sorted_word_freq.sort_by(|a, b| b.1.cmp(a.1).then(a.0.cmp(b.0)));
    sorted_word_freq.truncate(10);

    let mut res = "".to_string();
    for (word, count) in sorted_word_freq.iter() {
        res.push_str(&format!("{} {}\n", count, word));
    }

    res
}
