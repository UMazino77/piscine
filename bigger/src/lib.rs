pub fn bigger(h: HashMap<&str, i32>) -> i32 {
    let a = h.values();
    let max = i32::MIN;
    for i in a {
        if i > max {
            max = i;
        }
    }
    max
}