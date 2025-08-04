pub fn talking(text: &str) -> &str {
    if text.trim().is_empty() {
        return "Just say something!";
    }
    if is_all_upper(text) && text.ends_with("?") {
        return "Quiet, I am thinking!"
    }
    if text.ends_with("?") {
        return "Sure."
    }
    if is_all_upper(text) && text.ends_with("!") {
        return "There is no need to yell, calm down!"
    }

    // println!("{text}") ;

    "Interesting"
}

pub fn is_all_upper(text : &str) -> bool {
    for i in text.chars() {
        if i.is_ascii_alphanumeric() && !i.is_ascii_uppercase() {
            return false ;
        }
    }
    return true
}