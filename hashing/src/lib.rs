use std::collections::HashMap;

pub fn mean(list: &[i32]) -> f64 {
    let a : f64 = list.to_owned().iter().sum::<i32>() as f64;
    let b : f64 = list.len() as f64;
    a/b
}

pub fn median(list: &[i32]) -> i32 {
    let mut c = list.to_owned();
    c.sort();
    if c.len()% 2 == 0 {
        return (c[c.len()/2]+c[c.len()/2-1])/2
    }
    c[c.len()/2]
}

pub fn mode(list: &[i32]) -> i32 {
    let mut c = HashMap::new();
    list.iter().for_each(|x| {
        let cc = c.entry(x).or_insert(0);
        *cc+=1;
    });

    // println!(" ===> {:?} " , c);

    let mut ind = 0;
    let mut max = i32::MIN ;    
    for (i,val) in c {
        if val > max {
            max = val;
            ind = *i;
        }
    }
    if max == 1 {
        return list[0];
    }
    ind
}