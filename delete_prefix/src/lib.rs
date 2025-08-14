pub fn delete_prefix<'a>(prefix: &'a str, s: &'a str) -> Option<&'a str> {
    if s[0..prefix.len()] == *prefix {
        return Some(&s[prefix.len()..]) ;
    }

    None
    
}