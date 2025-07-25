pub fn nbr_function(c: i32) -> (i32, f64, f64) {
    let d = c as f64;
    (c, d.exp(), d.ln())
}

pub fn str_function(a: String) -> (String, String) {
    let c = a.split(" ");
    let mut s = String::from("");
    for cc in c  {
        let mut d: f64 = cc.parse().expect("error");
        d = d.exp();
        s = format!("{} {} ",s,d.to_string());
    }
    (a, s.trim().to_string())
}

pub fn vec_function(b: Vec<i32>) -> (Vec<i32>, Vec<f64>) {
    let mut c : Vec<f64> = vec![];
    for val in &b {
        let d : f64 = *val as f64;
        c.push(d.ln())
    }
    (b,c)
}