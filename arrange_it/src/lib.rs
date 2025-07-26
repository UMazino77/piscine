pub fn arrange_phrase(phrase: &str) -> String {
    let b = phrase.to_owned();
    let a: Vec<_> = b.split(" ").collect();

    let s : String :: String.new();
    
    for (ind,val) in a.into_iter().enumerate() {
        println!("{} ===>  {}",ind, val)
    }

    todo!()
}