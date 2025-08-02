pub struct Food {
    name : String,
    calories: (String, String),
    fats: f64,
    carbs: f64,
    proteins: f64,
    nbr_of_portions: f64
}

pub fn calculate_macros(foods: &[Food]) -> json::JsonValue {
    todo!()
}

fn helper(num: f64) -> String {
    let aa = format!("{:.2}", num);
    let bb = aa.trim_end_matches('0').trim_end_matches('.');
    if bb.is_empty() { "0".to_string() } else { bb.to_string() }
}