use std::io::{self, Write};

fn is_prime(n: u32) -> bool {
    if n < 2 {
        return false;
    }

    let sqrt_n = (n as f64).sqrt() as u32;
    
    for i in 2..=sqrt_n {
        if n % i == 0 {
            return false;
        }
    }

    true
}

fn main() {
    loop {
        print!("Input: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input);

        let number: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Not number, again: ");
                continue;
            }
        };

        if is_prime(number) {
            println!("{} is a prime.", number);
        } else {
            println!("{} not a prime.", number);
        }
    }
}
