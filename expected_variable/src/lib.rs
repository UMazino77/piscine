use edit_distance::edit_distance;
use convert_case::{Case, Casing};

pub fn expected_variable(str1 : &str , str2 : &str)-> Option<String> {

    if (!str1.is_case(Case::Camel) && !str2.is_case(Case::Camel)) && (!str1.is_case(Case::Snake) && !str2.is_case(Case::Snake)) && (!str1.is_case(Case::Pascal) && !str2.is_case(Case::Pascal)) {
        println!("{}", str1);
        return None ;
    }

    let aa = str1.to_lowercase() ;
    let bb = str2.to_lowercase() ;

    let a = edit_distance(&aa, &bb);
    let b = 100 - 100*a / str2.len() ;
    if b < 50 {
       return  None ;
    }
    Some(format!("{b}%"))
}