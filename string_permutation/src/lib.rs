use std::collections::HashMap;

pub fn is_permutation(s1: &str, s2: &str) -> bool {
    if s1.len() != s2.len() {
        return false ;
    }
    let mut j = HashMap::new();
    let mut k= HashMap::new();

    let bbb = s1.chars().collect::<Vec<_>>();

    for v in bbb.iter() {
        let kk = j.entry(v).or_insert(0);
        *kk += 1;
    }

    let aaa = s2.chars().collect::<Vec<_>>();

    for v in aaa.iter() {
        let kk = k.entry(v).or_insert(0);
        *kk += 1;
    }

    // println!("{:?} ====> {:?}" ,j, k );

    for val in j.keys() {
        let a = j.get(val).copied().unwrap_or(0);
        let b = k.get(val).copied().unwrap_or(0);
        // println!("{} ===> {}, {}", a, b, val);
        if a != b {
            return false;
        }
    }
    true
}