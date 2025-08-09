use generics_list::*;

fn main() {
    let mut new_list_str = List::new();
    new_list_str.push("String Test 1");
    println!("The size of the list is {}", new_list_str.len());

    new_list_str.push("String Test 2");
    println!("The size of the list is {}", new_list_str.len());

    new_list_str.push("String Test 3");
    println!("The size of the list is {}", new_list_str.len());

    // println!("the new list {:?}", new_list_str);
    new_list_str.pop();
    println!("The size of the list is {}", new_list_str.len());

    let mut a = List::new();

    for i in 0..20 {
        a.push(i);
    }
    let mut aa = a.head.unwrap();
    for i in (1..20).collect::<Vec<i32>>().iter().rev() {
        println!("----- {} ++++ {}", *i,aa.value);
        aa = *aa.next.unwrap();
    }
    // println!("{:?}", new_list_nbr);
}


// need to learn to read the subject before doing anything