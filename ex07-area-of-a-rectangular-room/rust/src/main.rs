use std::io::{self, Write};

fn prompt_for_integer(question: &str) -> u32 {
    loop {
        print!("{} ", question);
        io::stdout().flush().unwrap();
        let mut s = String::new();

        io::stdin().read_line(&mut s).expect("Unable to read from stdin");
        match s.trim().parse() {
            Ok(n) => break n,
            Err(_) => println!("Please enter a valid number."),
        }
    }
}


fn main() {
    let length = prompt_for_integer("What is the length of the room?");
    let width = prompt_for_integer("What is the length of the room?");
    let area_in_feet = length * width;
    const CONVERSION_FACTOR: f32 = 0.09290304;
    let area_in_meter = CONVERSION_FACTOR * area_in_feet as f32;

    println!("You entered dimensions of {} feet by {} feet.", length, width);
    println!("{} square feet", area_in_feet);
    println!("{:.3} square meters", area_in_meter);
}
