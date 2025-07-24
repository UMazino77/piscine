pub fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f-32.0)/(9.0/5.0)
}

pub fn celsius_to_fahrenheit(c: f64) -> f64 {
    32.0+ c*((9.0/5.0))
}