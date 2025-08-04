pub fn number_logic(num: u32) -> bool {
    let mut num_str = num.to_string();
    let n = num_str.len() ;
    // println!("+++++ {}", num_str);
    let mut res = 0.0 ;
    for i in 0..num_str.len() {
        let r = num_str.pop().unwrap();
        let rr : f64 = r.to_string().parse().unwrap();
        // println!("-----{rr}   {}" , num_str.len());
        res = res + rr.powf(n as f64) ;
    }
    // println!("{res}");

    num == res as u32
}
