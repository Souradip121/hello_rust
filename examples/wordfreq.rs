use std::collections::HashMap;

fn most_frequent_word(words: Vec<&str>) -> Option<String> {
    if words.is_empty() {
        return None;
    }

    let mut counts: HashMap<&str, usize> = HashMap::new();
    for word in &words {
        *counts.entry(word).or_insert(0) += 1;
    }

    counts
        .into_iter()
        .max_by_key(|(_, count)| *count)
        .map(|(word, _)| word.to_string())
}


//

fn main() {
    let words = vec!["apple", "banana", "apple", "cherry", "banana", "apple"];
    println!("{:?}", most_frequent_word(words)); // Some("apple")

    let empty: Vec<&str> = vec![];
    println!("{:?}", most_frequent_word(empty));  // None
}