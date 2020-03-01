use std::io::{self, Write};


fn main() {
    let mut total = 0u32;
    let mut count = 1;

    loop {
        if let Some(n) = prompt(&format!("Enter the price of item {}", count)) {
            let price = n;
            let question = format!("Enter the quantity of item {}", count);

            loop {
                if let Some(n) = prompt(&question) {
                    total += n * price;
                    count += 1;
                    break;
                }
            }
        } else {
            break;
        }
    }

    println!("Subtotal: ${}.00", total);

    let tax = total as f32 * 0.055;
    println!("Tax: ${:.2}", tax);
    println!("Total: ${:.2}", tax + total as f32);
}


fn prompt(question: &str) -> Option<u32> {
    loop {
        let mut input = String::new();

        print!("{}: ", question);
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).expect("Unable to read from stdin");

        let s = input.trim();

        if s.is_empty() {
            break None;
        } else {
            match s.parse() {
                Ok(n)   => break Some(n),
                Err(_)  => println!("Invalid input. Please enter a number."),
            }
        }
    }
}
