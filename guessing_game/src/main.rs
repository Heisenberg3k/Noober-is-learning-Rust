use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Hi guys! Welcome to Gacha game");

    let gachaNumber: u32 = rand::thread_rng().gen_range(1..11);
    loop {
        println!("Choose a number from 1 to 10:");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");
        
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You selected: {}", guess);

        match guess.cmp(&gachaNumber){
            Ordering::Less => println!("Smaller"),
            Ordering::Greater => println!("Better"),
            Ordering::Equal => {
                println!("Equal");
                break;
            }
        }
    }
}