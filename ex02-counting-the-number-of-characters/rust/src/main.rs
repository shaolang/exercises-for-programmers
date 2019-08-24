use std::io::{self, Write};

fn main() {
    loop {
        print!("What is the input string? ");
        io::stdout().flush().unwrap();

        let mut s = String::new();
        io::stdin().read_line(&mut s).expect("Unable to read from stdin");

        let s = s.trim();

        if s == "" {
            println!("Please enter something.");
        } else {
            println!("{} has {} characters.", s, s.len());
            break;
        }
    }
}
