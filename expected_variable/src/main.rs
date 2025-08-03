use expected_variable::*;

fn main() {
    println!(
        "{:?} close to it",
        expected_variable("On_Point", "on_point")
    );
    println!(
        "{:?} close to it",
        expected_variable("soClose", "so_close")
    );
    println!(
        "{:?}",
        expected_variable("something", "something_completely_different")
    );
    println!(
        "{:?} close to it",
        expected_variable("BenedictCumberbatch", "BeneficialCucumbersnatch")
    );
}