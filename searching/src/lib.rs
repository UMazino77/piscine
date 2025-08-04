pub fn search(array: &[i32], key: i32) -> Option<usize> {
    let mut ii : i32 = -1 ;
    for i in 0..array.len() {
        if array[i] == key {
            ii = i as i32 ;
        }
    }
    if ii > 0 {
        return Some(ii as usize);
    }
    None
}