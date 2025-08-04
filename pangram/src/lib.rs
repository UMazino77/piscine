use std::collections::HashSet;
pub fn is_pangram(s: &str) -> bool {
    let mut a = HashSet::new() ;
    for i in s.chars() {
        if i.is_ascii_alphabetic() {
            a.insert(i.to_ascii_lowercase());
        }
    }
    // println!("{:?}", a);
    return a.len() == 26
}