use std::io::{self, Write};
use std::time::SystemTime;

fn get_current_year() -> u16 {
    let now = SystemTime::now();
    let elapsed = now.duration_since(SystemTime::UNIX_EPOCH)
        .expect("Unable to get elapsed time");

    // not entire correct, 'cos doesn't account for leap years
    1970u16 + (elapsed.as_secs() / (60 * 60 * 24 * 365)) as u16
}


fn prompt(question: &str) -> String {
    print!("{} ", question);
    io::stdout().flush().unwrap();
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("Unable to read from stdin");

    s.trim().to_string()
}


fn prompt_for_u16(question: &str) -> u16 {
    loop {
        let s = prompt(question);

        if let Ok(n) = s.parse() {
            break n;
        } else {
            println!("Please enter a number.");
        }
    }
}


fn main() {
    let current_age = prompt_for_u16("What is your age?");
    let retirement_age = prompt_for_u16("What is your retirement age?");
    let current_year = get_current_year();

    let years_left = retirement_age - current_age;
    let retirement_year = current_year + years_left;

    println!("You have {} years left until you can retire.", years_left);
    println!("It's {}, so you can retire in {}.", current_year, retirement_year);
}
