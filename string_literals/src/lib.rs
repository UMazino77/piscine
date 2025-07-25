pub fn is_empty(v: &str) -> bool {
    v.is_empty()
}

pub fn is_ascii(v: &str) -> bool {
    v.is_ascii()
}

pub fn contains(v: &str, pat: &str) -> bool {
    v.contains(pat)
}

pub fn split_at(v: &str, index: usize) -> (&str, &str) {
    let a = v.split_at_checked(index);
    match a{
        Some ((b,c)) => return (b,c),
        None => println!("error") 
    }
    ("","")
    //v.split_at(index)
}
// we can use split_at i prefered to use split_at_checked to avoid panicks 
// if the index if out of range of v 
// i had a brief intro to `option<>` 
// interesting module

pub fn find(v: &str, pat: char) -> usize {
    let a = v.find(pat);
    match a {
        Some(a) => return a ,
        None => println!("error")
    }
    v.len()
}