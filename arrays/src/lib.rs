pub fn sum(a: &[i32]) -> i32 {
    let mut res = 0;
    a.into_iter().for_each(|xx| {res += xx});
    res
}

pub fn thirtytwo_tens() -> [i32; 32] {
    [10;32]
}