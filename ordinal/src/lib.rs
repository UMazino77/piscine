pub fn num_to_ordinal(x: u32) -> String {
    
    if (x as i64 -1)%10 == 0 && x%100 != 11 { return format!("{}{}", x, "st")} 
    else if  (x as i64 -2)%10 == 0 && x%100 != 12 {return format!("{}{}", x, "nd")}
    else if (x as i64 -3)%10 == 0 && x%100 != 13 {return format!("{}{}", x, "rd")} 
    
    format!("{}{}", x, "th")
}