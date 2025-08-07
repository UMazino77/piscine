pub fn parse_into_boxed(s: String) -> Vec<Box<u32>> {
    let mut a = Vec::new();
    s.split(" ").for_each(|x| {
        let mut ss = 0.0 ;
        if x.contains("k") {
            let jj = x.replace("k", "");
            ss  = jj.parse::<f64>().unwrap()*1000.;
        } else {
            ss = x.parse().unwrap_or(0.);
        }
        // println!("{jj}");

        a.push(Box::new(ss as u32)) ;
    });
    a
}

pub fn into_unboxed(a: Vec<Box<u32>>) -> Vec<u32> {
    let mut aa = Vec::new() ; 
    for i in a {
        aa.push(*i) ;
    }

    aa
}