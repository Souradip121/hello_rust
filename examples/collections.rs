use std::{collections::{HashMap, HashSet}, vec};

fn word_frequency(words: &Vec<&str>) -> HashMap<String,u32>
{
    let mut map: HashMap<String,u32> = HashMap::new();
    for word in words {
        let count = map.entry(word.to_string()).or_insert(0);
        *count += 1;
    }
    map

}

fn unique_words(words: &Vec<&str>) -> HashSet<String>{
    let mut set : HashSet<String> = HashSet::new();
    for word in words {
        set.insert(word.to_string());
    }
    set

}

fn main(){
    let words = vec!["rust", "is", "fast", "rust", "is", "rust"];
    println!("{:?}",word_frequency(&words));

    let unique = unique_words(&words);
    println!("Unique words: {}", unique.len());
}