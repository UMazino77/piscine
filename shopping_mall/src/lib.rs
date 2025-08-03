pub mod mall ;
pub use mall::* ;
use std::collections::HashMap;


pub fn biggest_store(mall : &Mall) -> Store {
    let mut max_area = 0;
    // let empl = Employee {age: 0, working_hours: (0, 0),  salary: 0.0};
    let aa : HashMap<String, Employee> = Default::default() ;
    let mut strr = Store::new(aa, 0);
    for floor in mall.floors.values(){
        for store in floor.stores.values() {
            if store.square_meters >= max_area {
                max_area = store.square_meters;
                strr = store.clone()  ;
            }
        }
    }

    strr
}

pub fn highest_paid_employee(mall : &Mall) -> Vec<Employee> {
    let mut max = f64::MIN;
    let mut bb : Vec<Employee> = vec![] ;
    for floor in mall.floors.values(){
        for store in floor.stores.values() {
            for empl in store.employees.values() {
                if empl.salary >= max {
                    if empl.salary == max {
                        bb.push(*empl) ;
                    } else {
                        bb.clear() ;
                        bb.push(*empl) ;
                    }
                    max = empl.salary;
                }
            }
        }
    }

    bb
}

pub fn nbr_of_employees(mall : &Mall, ) -> usize {
    let mut a = 0;
    for floor in mall.floors.values(){
        for store in floor.stores.values() {
            a += store.employees.len();
        }
    }
    a + mall.guards.len()
}

pub fn check_for_securities(mall :  &Mall,mut guards :HashMap<String, Guard>) -> usize {
    let nb = guards.len();
    let mut area = 0;
    for floor in mall.floors.values(){
        for store in floor.stores.values() {
            area += store.square_meters ;
        }
    }
    if area / (nb as u64) < 200 {
        guards.insert("mohamed".to_owned(), Guard{age : 22 , years_experience : 1});
        return nb+1 ;
    }
    nb
}

pub fn cut_or_raise( mall : &mut Mall) -> usize {
    for floor in mall.floors.values_mut(){
        for store in floor.stores.values_mut() {
            for empl in store.employees.values_mut() {
                if empl.working_hours.1-empl.working_hours.0 >= 10 {
                    empl.salary = empl.salary + empl.salary/10.0 ;
                } else {
                    empl.salary = empl.salary - empl.salary/10.0 ;
                }
            }
        }
    }
    1
}
