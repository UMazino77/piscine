use std::collections::HashMap;

pub fn word_frequency_counter<'a>(words: &[&'a str]) -> HashMap<&'a str, usize> {
    let mut j:HashMap<&'a str, usize> = HashMap::new();
    for i in words {
        let k = j.entry(i).or_insert(0);
        *k+=1;
    }
    j
}

pub fn nb_distinct_words(frequency_count: &HashMap<&str, usize>) -> usize {
    frequency_count.len()
}