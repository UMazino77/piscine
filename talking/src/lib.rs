pub fn talking(text: &str) -> &str {
    if text.trim().is_empty() {
        return "Just say something!";
    }
    if is_all_upper(text, false) && text.ends_with("?") {
        return "Quiet, I am thinking!"
    }
    if text.ends_with("?") {
        return "Sure."
    }
    if is_all_upper(text, true) {
        return "There is no need to yell, calm down!"
    }

    // println!("{text}") ;

    "Interesting"
}

pub fn is_all_upper(text : &str, yell : bool ) -> bool {
    for i in text.chars() {
        if i.is_ascii_alphanumeric() && (!i.is_ascii_uppercase() && (!yell || !i.is_ascii_digit())) {
            // println!("{text}");
            // println!("------   {i}  +++++");
            return false ;
        }
    }
    return true
}