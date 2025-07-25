pub fn initials(names: Vec<&str>) -> Vec<String> {
    let mut a :Vec<String> = vec![];
    for name in names {
        let n = name.to_string()[0..1].to_owned() + ("."); 
        a.push(n);
    }
    a
}

// second encounter with vec 
// switching between to_string() and to_owned()
