use std::io::{self, Write};
use std::ops::Add;

fn prompt(question: &str) -> String {
    print!("{} ", question);
    io::stdout().flush().unwrap();
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("Unable to read from stdin");

    s.trim().to_string()
}


fn main() {
    let quote = prompt("What is the quote?");
    let author = prompt("Who said it?");

    let output = author.add(" says, \"").add(&quote).add("\"");

    println!("{}", output);
}
