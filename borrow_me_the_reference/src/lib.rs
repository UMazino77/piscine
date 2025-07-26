pub fn delete_and_backspace(s: &mut String) {
    let mut a: Vec<char> = s.chars().collect();
    let mut i: usize =0;
    let mut c = false ;
    while i<a.len() {
       if a[i] == '+' && (i == a.len()-2 || a[i+1] != '+') {
        a.remove(i);
        a.remove(i);
        if c {
            i = 0
        }
       } else if a[i] == '+' && a[i+1] == '+' {
        c = true
       } else if a[i] == '-' {
        a.remove(i-1);
        a.remove(i-1);
        i-=2;
       }
       i+=1;
    }
    *s = a.iter().collect();
    if s.contains('+') {
        delete_and_backspace(s);
    }
}

pub fn do_operations(v: &mut [String]) {

    for (_,val) in v.iter_mut().enumerate() {
        if val.contains("+") {
            let a : Vec<_> = val.split("+").collect() ;
            let b : i32 = a[0].parse().expect("error");
            let c : i32 = a[1].parse().expect("error");
            let d : i32 = b+c;
            *val = d.to_string();
        } else {
            let a : Vec<_> = val.split("-").collect() ;
            let b : i32 = a[0].parse().expect("error");
            let c : i32 = a[1].parse().expect("error");
            let d : i32 = b-c;
            *val = d.to_string();
        }
    }
}

// that was challenging more than i thought 
// learnt more about string manipulation while the var is borrowed 
// and more methods 