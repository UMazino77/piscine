pub fn score(s : &str) -> u64 {
    let mut a = 0 ;
    for i in s.chars() {
        match i.to_ascii_lowercase() {
            'a' | 'e' | 'i' | 'o' | 'u' | 'l' | 'n' | 'r' | 's' | 't' => a+=1 ,
            'd' | 'g' => a += 2 ,
            'b' | 'c' | 'm' | 'p' => a+= 3 ,
            'f' | 'h' | 'v' | 'w' | 'y' => a+= 4 ,
            'k' => a+= 5,
            'j' | 'x' => a+= 8 ,
            'q' | 'z' => a+= 10,
            _ => a+= 0
        }
    }
    a
}