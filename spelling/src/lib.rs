pub fn spell(n: u64) -> String {
    let mut m = n ;
    let mut s = String::new() ;
    if n ==0 {
        return String::from("zero");
    }
    while m > 0 {
        if m >= 1000000 {
            s = format!("{s}{}{} million ",spell(m/1000000),to(1000000));
            m = m - (m/1000000 * 1000000) ;
        }
        if m >= 1000 {
            // if m/1000 
            s = format!("{s}{} thousand ",spell(m/1000));
            m = m - (m/1000 * 1000) ;

        }
        if m >= 100 {
            s = format!("{s}{} hundred ",to(m/100));
            m = m - (m/100 * 100) ;
        }
        if m >= 10 {
            // println!("{m}");
            if m - m/10*10 != 0 && m >= 20{
                s = format!("{s}{}-",to(m/10*10));
                m = m - (m/10 * 10) ;
                
            } else if m-m/10*10 == 0{
                s = format!("{s}{} ",to(m));
                m = m - (m/10 * 10) ;
            } else {
                s = format!("{s}{} ",to(m));
                m = 0 ;
            }
            // m = m - (m/10 * 10) ;
        }
        if m == 0 {
            break ;
        }
        s = format!("{s}{} ",to(m%10));
        m/=10 ;
    }
    s.trim().to_owned()
}

pub fn to(n : u64) -> &'static str {
    return match n {
        1 => "one",
        2 => "two",
        3 => "three",
        4 => "four",
        5 => "five",
        6 => "six",
        7 => "seven",
        8 => "eight",
        9 => "nine",
        10 => "ten",
        11 => "eleven",
        12 => "twelve",
        13 => "thirteen",
        14 => "fourteen",
        15 => "fifteen",
        16 => "sixteen",
        17 => "seventeen",
        18 => "eighteen",
        19 => "nineteen",
        20 => "twenty",
        30 => "thirty",
        40 => "forty",
        50 => "fifty",
        60 => "sixty",
        70 => "seventy",
        80 => "eighty",
        90 => "ninety",
        100 => "hundred",
        1000 => "thousand",
        100000 => "million",
        _ => ""
    }
}