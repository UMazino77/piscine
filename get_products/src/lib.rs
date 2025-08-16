pub fn get_products(arr: Vec<usize>) -> Vec<usize> {
    let mut a : Vec<usize> = vec![] ;
    for i in 0..arr.len() {
        let mut res = 1;
        for j in 0 .. arr.len() {
            if arr[i] != arr[j] {
                res *= arr[j];
            }
        }
    a.push(res);
    }
    a
}