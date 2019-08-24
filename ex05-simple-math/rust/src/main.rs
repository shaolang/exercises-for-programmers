use std::io::{self, Write};

fn prompt(question: &str) -> String {
    print!("{} ", question);
    io::stdout().flush().unwrap();
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("Unable to read from stdin");

    s.trim().to_string()
}


fn prompt_for_integer(question: &str) -> i32 {
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
    let m = prompt_for_integer("What is the first number?");
    let n = prompt_for_integer("What is the second number?");

    println!("{m} + {n} = {a}\n{m} - {n} = {b}\n{m} * {n} = {c}\n{m} / {n} = {d}",
             m = m,
             n = n,
             a = m + n,
             b = m - n,
             c = m * n,
             d = m / n);
}
