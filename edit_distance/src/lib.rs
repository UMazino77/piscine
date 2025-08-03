
pub fn edit_distance(source: &str, target: &str) -> usize {
    let (a, b): (Vec<_>, Vec<_>) = (source.chars().collect(), target.chars().collect());
    let (c, d) = (a.len(), b.len());
    let mut res = vec![vec![0; d + 1]; c + 1];
    
    for i in 0..=c { res[i][0] = i; }
    for j in 0..=d { res[0][j] = j; }
    
    for i in 1..=c {
        for j in 1..=d {
            res[i][j] = if a[i-1] == b[j-1] {
                res[i-1][j-1]
            } else {
                1 + [res[i-1][j], res[i][j-1], res[i-1][j-1]].iter().min().unwrap()
            };
        }
    }
    res[c][d]
}



