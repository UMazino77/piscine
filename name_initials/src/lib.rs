pub fn initials(names: Vec<&str>) -> Vec<String> {
    let mut a :Vec<String> = vec![];
    for name in names {
        let b = find(name, ' ');
        let (c,d) = split_at(name, b);
        let n = c.to_string()[0..1].to_owned() + (".") + &d.to_string()[0..2].to_owned() + ("."); 
        a.push(n);
    }
    a
}

// second encounter with vec 
// switching between to_string() and to_owned()

pub fn split_at(v: &str, index: usize) -> (&str, &str) {
    let a = v.split_at_checked(index);
    match a{
        Some ((b,c)) => return (b,c),
        None => println!("error") 
    }
    ("","")
}

pub fn find(v: &str, pat: char) -> usize {
    let a = v.find(pat);
    match a {
        Some(a) => return a ,
        None => println!("error")
    }
    v.len()
}
