pub fn stars(n: u32) -> String {
    let mut s = String::new() ;
    for i in 0.. (2.0_f64).powf(n as f64) as u32 {
        s.push('*') ;
    }
    s
}