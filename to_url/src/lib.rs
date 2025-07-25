pub fn to_url(s: &str) -> String {
    s.replace(" ", "%20").to_string()
}

// this one was easy 
// using the replace method and at the end converting the str to String using to_string()
// no idea if i should handle \t etc...