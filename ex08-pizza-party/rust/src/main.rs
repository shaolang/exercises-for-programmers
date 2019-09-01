use std::io::{self, Write};

fn prompt_for_integer(question: &str) -> u32 {
    loop {
        print!("{} ", question);
        io::stdout().flush().unwrap();
        let mut s = String::new();
        io::stdin().read_line(&mut s).expect("Unable to read from stdin");

        match s.trim().parse() {
            Ok(n) => break n,
            Err(_) => println!("Please enter a number."),
        }
    }
}

fn main() {
    let n_people = prompt_for_integer("How many people?");
    let n_pizzas = prompt_for_integer("How many pizzas do you have?");
    let total_pieces = n_pizzas * 8;
    let pieces_per_pax = total_pieces / n_people;
    let leftovers = total_pieces % n_people;

    println!("{} people with {} pizzas", n_people, n_pizzas);
    println!("Each person gets {} pieces of pizza.", pieces_per_pax);
    println!("There are {} leftover pieces.", leftovers);
}
