pub fn first_subword(mut s: String) -> String {
    let mut a = s.chars();
    for (index,value) in a.enumerate() {
        if index > 0 && (value == ' ' || value.is_uppercase() || value == '_') {
            //println!("{} ===> {}",index, value );
            // let b :String = a.into_iter().collect();
            //let res = &b[..index];
            let (c, _) = s.split_at(index);
            return c.to_string();
        }
    }
    s
}
 