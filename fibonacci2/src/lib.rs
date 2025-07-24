pub fn fibonacci(n: u32) -> u32 {
    let mut a:u32 = 0;
    let mut b:u32 = 1;
    let mut c:u32 = 1;
    for _i in 0..n {
        c = a+b ;
        a = b ;
        b = c ;
    }
    a
}