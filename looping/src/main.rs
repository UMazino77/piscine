use std::io;

fn main() {
    let mut i = 1;
    loop {
        let mut riddle = String::new();
        println!(
            "I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?"
        );
        io::stdin().read_line(&mut riddle).expect("Failed to read input");
        if riddle.trim() == "The letter e" {
            println!("Number of trials: {i}");
            break;
        }
        i += 1;
    }
}
