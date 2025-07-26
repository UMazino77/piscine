pub fn arrange_phrase(phrase: &str) -> String {
    let b = phrase.to_owned();
    let a: Vec<_> = b.split(" ").collect();
    let mut dd: Vec<String> = vec![String::new(); a.len()];

    // let s = String::from("");
    let mut i = 0 ;
    while i < a.len() {
        // println!("{}", a[i]);
        // dd[i] = a[i].to_string();
        let mut cc = 0;
        let mut jj = String::new();
        let mut k:String = String::new();

        for l in a[i].chars() {
            if l.is_ascii_digit() {
                jj.push(l);
                continue
                // a[i].remove(j);
                // j+=1 ;
            }
            k.push(l);
        }
        cc = jj.parse().expect("error");
        dd[cc-1] = k;

        i+=1;
    }
    dd.join(" ")
 }