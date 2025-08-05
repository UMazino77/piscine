pub fn get_diamond(c: char) -> Vec<String> {
    let j = c as u8 - 65;
    if j == 0 {
        return vec![String::from("A")];
    }
    let mut a: Vec<String> = vec![String::new(); (2*j+1) as usize];
    for i in 0..2*j+1 {
        let mut s = String::new();
        for ii in 0..2*j+1 {
            let mut dd = 0 ;
            if i <= j {
                dd =  i ;
            } else {
                dd = 2*j - i ;
            }
            // println!("{ii}  +++++ {j} +++ {dd}");
            if ii == j + dd || ii == j - dd {
                s.push((65 + dd) as char);
            } else {
                s.push(' ');
            }
        }
        a[i as usize] = s;
    }
    a
}

pub fn abs(a: i8) -> i8 {
    if a >= 0 {
        return a;
    }
    -1 * a
}