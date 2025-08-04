pub fn pig_latin(text: &str) -> String {
    let mut a = String::from(text); 
    let vowels = "aeouiAEOUI" ;
    if vowels.contains(&text[0..1]) {
        return format!("{}{}" , text, "ay") ;
    } else if text[1..3].to_lowercase() == "qu" {
        return format!("{}{}{}" , &text[3..],&text[0..3], "ay") ;
    }
    let mut j = 0;
    for i in a.chars() {
        if vowels.contains(i) {
            return format!("{}{}{}",&text[j..],&text[..j],"ay");
        }
        j+=1 ;
    }
    a
}