pub fn capitalize_first(input: &str) -> String {
    let mut sss: Vec<_> = input.to_owned().chars().collect();
    sss[0] = sss[0].to_uppercase().nth(0).expect("error");
    sss.into_iter().collect()
}

pub fn title_case(input: &str) -> String {
    let b = input.to_owned();
    let c : Vec<_>= b.split(" ").into_iter().collect();
    let mut cc : Vec<_> = vec![];
    for inp in &c {
        // println!("{} ===> {}", inp,capitalize_first(inp));
        cc.push(capitalize_first(inp));
    }
    cc.join(" ")
}

pub fn change_case(_input: &str) -> String {
    let a = input.to_owned();
    let b = a.split(" ").into_iter().collect();
    
    todo!()
}