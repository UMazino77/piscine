pub fn fibonacci(n: u32) -> u32 {
    let mut a = 0;
    let mut b = 1;
    let mut c = 1;
    for i in 0..n-1 {
        c = a+b ;
        a = b ;
        b = c ;
    }
    b
}