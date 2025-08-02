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
        let cal : f64= food.calories.1.trim_end_matches("kcal").parse().unwrap() ;
        cals += cal * food.nbr_of_portions;
        carbs += food.carbs * food.nbr_of_portions ;
        proteins += food.proteins * food.nbr_of_portions ;
        fats += food.fats * food.nbr_of_portions ;
    }

    cals = helper(cals).parse().expect("error");
    carbs = helper(carbs).parse().expect("error");
    proteins = helper(proteins).parse().expect("error");
    fats = helper(fats).parse().expect("error");
    
    
    data["cals"] = cals.into();
    data["carbs"] = carbs.into();
    data["proteins"] = proteins.into();
    data["fats"] = fats.into();

    data

}

fn helper(num: f64) -> String {
    let aa = format!("{:.2}", num);
    let bb = aa.trim_end_matches('0').trim_end_matches('.');
    if bb.is_empty() { "0".to_string() } else { bb.to_owned()}
}