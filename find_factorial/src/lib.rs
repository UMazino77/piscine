pub fn factorial(num: u64) -> u64 {
    let mut i = 1;
    for a in (2..num+1) {
        i = i*a
    }
    i
}