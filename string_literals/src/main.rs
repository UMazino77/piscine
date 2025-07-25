use string_literals::*;

fn main() {
    println!("{}", is_empty(""));
    println!("{}", is_ascii("rust"));
    println!("{}", contains("rust", "ut"));
    println!("{:?}", split_at("rust", 3));
    println!("{}", find("rust", 'u'));
}