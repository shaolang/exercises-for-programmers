use std::io::{self, Write};

fn prompt(question: &str) -> String {
    print!("{} ", question);
    io::stdout().flush().unwrap();
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("Unable to read from stdin");

    s.trim().to_string()
}


fn main() {
    let noun = prompt("Enter a noun:");
    let verb = prompt("Enter a verb:");
    let adjective = prompt("Enter a adjective:");
    let adverb = prompt("Enter a adverb:");

    println!("Do you {} your {} {} {}? That's hilarious!",
             verb, adjective, noun, adverb);
}
