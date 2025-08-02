pub struct Food {
    pub name : String,
    pub calories: (String, String),
    pub fats: f64,
    pub carbs: f64,
    pub proteins: f64,
    pub nbr_of_portions: f64
}

pub fn calculate_macros(foods: &[Food]) -> json::JsonValue {
    let mut data = json::JsonValue::new_object();
    let mut cals = 0.0;
    let mut carbs = 0.0;
    let mut  proteins = 0.0;
    let mut fats = 0.0;

    for food in foods {
        let cal : f64= food.calories.1.trim_end_matches("kcal").parse().expect("error") ;
        cals += cal * food.nbr_of_portions;
        carbs += food.carbs * food.nbr_of_portions ;
        proteins += food.proteins * food.nbr_of_portions ;
        fats += food.fats * food.nbr_of_portions ;
    }
    data["cals"] = helper(cals).into();
    data["carbs"] = helper(carbs).into();
    data["proteins"] = helper(proteins).into();
    data["fats"] = helper(fats).into();

    data

}

fn helper(num: f64) -> String {
    let aa = format!("{:.2}", num);
    let bb = aa.trim_end_matches('0').trim_end_matches('.');
    if bb.is_empty() { "0".to_string() } else { bb.to_string() }
}