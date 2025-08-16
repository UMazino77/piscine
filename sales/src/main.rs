use sales::*;

fn main() {
    let store = Store::new(vec![
        (String::from("product A"), 1.23),
        (String::from("product B"), 23.1),
        (String::from("product c"), 3.12),
        (String::from("product d"), 9.75),
        (String::from("product e"), 1.75),
        (String::from("product r"), 23.75),
        (String::from("product f"), 2.75),
        (String::from("product g"), 1.64),
        (String::from("product h"), 15.23),
        
        ]);


    println!("{:?}", store);

    let mut cart = Cart::new();
    cart.insert_item(&store, String::from("product A"));
    cart.insert_item(&store, String::from("product B"));
    cart.insert_item(&store, String::from("product c"));
    cart.insert_item(&store, String::from("product d"));
    cart.insert_item(&store, String::from("product e"));
    cart.insert_item(&store, String::from("product r"));
    cart.insert_item(&store, String::from("product f"));
    cart.insert_item(&store, String::from("product g"));
    cart.insert_item(&store, String::from("product h"));


    println!("{:?}", cart.generate_receipt());

    // println!("{:?}", cart);
}

// [1.23, 23.1, 3.12, 9.75, 1.75, 23.75, 2.75, 1.64, 15.23]