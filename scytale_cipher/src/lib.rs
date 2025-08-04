pub fn scytale_cipher(message: &str, i: usize) -> String {
    if message.is_empty() || i >= message.len() {
        return message.to_owned() ;
    }
    
    let mut ss = String::new() ;
    let mut j = 0;
    
    let ff : Vec<_> = message.chars().collect() ;
    let mut jj = 0 ; 

    let mut bb = false ;
    let mut ccc = 0;
    while jj + ccc < message.len() {
        ss.push(ff[j]) ;
        if j + i < message.len() {
            j = j+i ;
            bb = false;
            ccc += 1 ;
        } else {
            if bb == true {
                ss.push(' ');
            }
            j = jj+1;
            jj+=1 ;
            bb = true
        }
    }
    ss.trim_end().to_owned()
}