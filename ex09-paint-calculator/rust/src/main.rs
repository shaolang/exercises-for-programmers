use std::io::{self, Write};

const SQUARE_FEET_PER_GALLON: u32 = 350;

fn prompt(question: &str) -> u32 {
    loop {
        print!("{}", question);
        io::stdout().flush().expect("Unable to read from stdin");

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim().parse() {
            Ok(n)   => break n,
            Err(_)  => println!("Please enter a number."),
        }
    }
}


fn calculate_paint_needed(area: u32) -> u32 {
    (area / SQUARE_FEET_PER_GALLON) +
        (if area % SQUARE_FEET_PER_GALLON > 0 { 1 } else { 0 })
}


fn main() {
    let length = prompt("What is the length (in feet)? ");
    let width  = prompt("What is the width (in feet)? ");
    let area = length * width;
    let gallons = calculate_paint_needed(area);

    println!("You will need to purchase {} gallons of", gallons);
    println!("paint to cover {} square feet", length * width);
}
