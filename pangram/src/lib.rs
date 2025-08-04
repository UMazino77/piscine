use std::collections::HashMap;
pub fn is_pangram(s: &str) -> bool {
    let mut a = HashMap::new() ;
    for i in s.chars() {
        if i.is_alphabetic() {
            a.insert(i.to_ascii_lowercase(), 1);
        }
    }
    // println!("{}", a.len());
    return a.len() == 26
}