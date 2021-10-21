use std::io::stdin;

fn main() {
    println!("Hello, what's your name?")
    let mut your_name = String::new();

    std()
        .read_line(&mut your_name)
        .expect("Failed to read line.");

    println!("Hello {}", your_name)
}