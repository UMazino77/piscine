pub mod mall ;
pub use mall::* ;
use std::collections::HashMap;


pub fn biggest_store(mall : &Mall) -> (&str, Store) {
    let mut max_area = 0;
    let mut name = String::new() ;
    let aa : HashMap<String, Employee> = Default::default() ;
    let mut strr = Store::new(aa, 0);
    for floor in mall.floors.values(){
        for (key,store) in &floor.stores {
            if store.square_meters >= max_area {
                max_area = store.square_meters.clone();
                strr = Some(store)  ;
                name = (*key.clone()).to_owned() ;
            }
        }
    }

    (name,strr.unwrap())
}

pub fn highest_paid_employee(mall : &Mall) -> Vec<(&str,Employee)> {
    let mut max = f64::MIN;
    let mut bb : Vec<(String,Employee)> = vec![] ;
     for floor in mall.floors.values() {
        for store in floor.stores.values() {
            for employee in store.employees.values() {
                if employee.salary > max {
                    max = employee.salary;
                }
            }
        }
    }
    
    for floor in mall.floors.values() {
        for store in floor.stores.values() {
            for (name, employee) in &store.employees {
                if employee.salary == max {
                    bb.push((name.as_str(), employee));
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

pub fn check_for_securities(mall :  &mut Mall,guards :HashMap<String, Guard>) -> usize {
    let nb = guards.len();
    let mut area = 0;
    for floor in mall.floors.values(){
        for store in floor.stores.values() {
            area += store.square_meters.clone() ;
        }
    }
    for (name, guard) in guards {
            if mall.guards.len()-1 as u64 > area/200 {
                break;
            }
            mall.hire_guard(name, guard);
        }
    nb
}

pub fn cut_or_raise( mall : &mut Mall) -> usize {
    for floor in mall.floors.values_mut(){
        for store in floor.stores.values_mut() {
            for empl in store.employees.values_mut() {
                if empl.working_hours.1-empl.working_hours.0 >= 10 {
                    empl.raise(empl.salary/10.0) ;
                } else {
                    empl.cut(empl.salary/10.0) ;
                }
            }
        }
    }
    1
}
