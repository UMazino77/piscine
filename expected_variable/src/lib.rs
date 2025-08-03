use edit_distance::edit_distance;
use convert_case::{Case, Casing};

pub fn expected_variable(str1 : &str , str2 : &str)-> Option<String> {
    
    let aa = str1.to_lowercase() ;
    let bb = str2.to_lowercase() ;

    if !aa.is_case(Case::Camel)  && !aa.is_case(Case::Snake) {
        // println!(<"====>   {}",!normal(str1));
        // println!(">{}    <=====",!normal(str1));

        return None ;
    }

    let a = edit_distance(&aa, &bb);
    let b = 100 - 100*a / str2.len() ;
    if b < 50 {
       return  None ;
    }
    Some(format!("{b}%"))
}

pub fn normal (sss : &str) -> bool {
    !sss.contains('_') && !sss.contains('-') && sss.chars().all(|c| c.is_lowercase() || c.is_numeric())
}