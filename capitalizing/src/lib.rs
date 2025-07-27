pub fn capitalize_first(input: &str) -> String {
    if input.is_empty() {
        return String::new()
    }
    let mut sss: Vec<_> = input.to_owned().chars().collect();
    sss[0] = sss[0].to_uppercase().nth(0).expect("error");
    sss.into_iter().collect()
}

pub fn title_case(input: &str) -> String {
    if input.is_empty() {
        return String::new()
    }
    let b = input.to_owned();
    let c: Vec<_> = b.split(|c| c == ' ' || c == '\t' ).into_iter().collect();
    let mut cc: Vec<_> = vec![];
    let mut i = 0;
    let mut vect: Vec<_> = vec![];
    for il in b.chars() {
        if il.is_whitespace() {
            vect.push(il.to_string());
        }
    }

    for inp in &c {
        if i >= c.len()-1 {
            cc.push(capitalize_first(inp));
        } else {
            cc.push(capitalize_first(inp)+vect[i].as_str());
        }
        i+=1;
    }
    cc.concat()
}



/*
    split saves extra spaces for exmpl split_whitespace rmoves them
*/

pub fn change_case(input: &str) -> String {
    if input.is_empty() {
        return String::new()
    }
    let a = input.to_owned();
    let b: Vec<_> = a.split_whitespace().into_iter().collect();
    let mut dd: Vec<String> = vec![];
    for inp in b {
        let ccc = inp.to_string();
        let l: Vec<_> = ccc.chars().collect();
        let ff = l.into_iter().map(|c| {
                if c.is_ascii_lowercase() {
                    c.to_ascii_uppercase()
                } else if c.is_ascii_uppercase() {
                    c.to_ascii_lowercase()
                } else {
                    c
                }
            });

        dd.push(ff.collect());
    }
    dd.join(" ")
}
