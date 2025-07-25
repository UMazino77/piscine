pub fn initials(names: Vec<&str>) -> Vec<String> {
    let mut a :Vec<String> = vec![];
    for name in names {
        let n: Vec<_> = name.split(' ').collect();
        let p = n[0][0..1].to_owned()+". "+&n[1][0..1].to_owned()+".";
        a.push(p);
    }
    a
}


// second encounter with vec 
// switching between to_string() and to_owned()
