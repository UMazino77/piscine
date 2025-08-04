pub fn num_to_ordinal(x: u32) -> String {
    if x%100 != 11 && x%100 != 12 x%100 != 13 {
        return format!("{}{}", x, "th");
    } 
    if (x-1)%10 == 0 { return format!("{}{}", x, "st")} 
    else if  (x-2)%10 == 0  {return format!("{}{}", x, "nd")}
    else if (x-3)%10 == 0 {return format!("{}{}", x, "rd")} 
    format!("{}{}", x, "th")
}