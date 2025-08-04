pub fn rotate(input: &str, key: i8) -> String {
    let mut a = String::new() ;
    for i in input.chars() {
        if i.is_ascii_alphabetic() {
            if i.is_ascii_uppercase() {
                let k  : u8 = 65 + (((i as u8 - 65) as u8 + ((52 + key )%26)as u8)%26) as u8 ;
                a.push(k as char);
            } else {
                let k : u8= 97 + (((i as u8 - 97) as u8 + ((52 + key )%26)as u8)%26) as u8 ;
                a.push(k as char);
            }
        } else {
            a.push(i) ;
        }
    }
    a
}
