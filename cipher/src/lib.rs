#[derive(Debug, PartialEq)]
pub struct CipherError {
    pub expected : String,
}

pub fn cipher(original: &str, ciphered: &str) -> Result<(), CipherError> {
    let a = original.chars();
    let mut ab = String::new();
    for i in a {
        if i.is_ascii_lowercase() {
            let c = i as u8 - 97;
            let cc = 122 - c ;
            let d = cc as char;
            ab.push(d);
        } else if i.is_ascii_uppercase() {
            let c = i as u8 - 65;
            let cc = 90 - c ;
            let d = cc as char;
            ab.push(d);
        } else {
            ab.push(i);
        }
    }
    if ab == ciphered.to_owned() {
        Ok(())
    } else {
        Err(CipherError{expected : ab})
    }
}