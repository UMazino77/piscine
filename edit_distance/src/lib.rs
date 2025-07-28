use std::collections::HashMap;

pub fn edit_distance(source: &str, target: &str) -> usize {
    rec(source.as_bytes(), target.as_bytes(), 0, 0, &mut HashMap::new())
}

pub fn rec(s: &[u8],t: &[u8],i: usize, j: usize,m: &mut HashMap<(usize, usize), usize>) -> usize {
    if let Some(&r) = m.get(&(i, j)) {
        return r;
    }
    let r ;
    if i == s.len() {
        r = t.len() - j;
    } else if j == t.len() {
        r = s.len() - i;
    } else if s[i] == t[j] {
        r = rec(s, t, i + 1, j + 1, m);
    } else {
        r = 1 +[rec(s, t, i + 1, j, m), rec(s, t, i, j + 1, m), rec(s, t, i + 1, j + 1, m)].iter().min().unwrap()
    };
    r
}
