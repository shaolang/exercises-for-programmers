use std::io::{self, Write};

fn main() {
    print!("What is your name? ");
    io::stdout().flush().unwrap();

    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("Unable to read from stdin");

    println!("Hello, {}, nice to meet you!", s.trim());
}
