pub fn scytale_cipher(message: &str, i: usize) -> String {
    if message.is_empty() || i == 0 || i >= message.len() {
        return message.to_owned() ;
    }
    
    let mut ss = String::new() ;
    let mut j = 0;
    
    let ff : Vec<_> = message.chars().collect() ;
    let mut jj = 0 ; 

    let z = (message.len() as f64/i as f64).ceil() as usize;

    // let mut bb = false ;
     while jj < i {
        j = jj;
        let mut ccc = 0;
        while ccc < z {
            if j < message.len() {
                ss.push(ff[j]);
            } else {
                ss.push(' ');
            }
            j += i;
            ccc += 1;
        }
        jj += 1;
    }
    ss.trim_end().to_owned()
}