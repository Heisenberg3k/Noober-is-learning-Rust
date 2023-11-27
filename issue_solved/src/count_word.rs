use std::io;

pub fn main() {
    let mut input_string = String::new();
    io::stdin().read_line(&mut input_string).expect("Cannot read -> Cook");

    let mut word_count: Vec<(&str, usize)> = Vec::new();

    for word in input_string.split_whitespace() {
        let mut found = false;

        for entry in word_count.iter_mut() {
            if entry.0 == word {
                entry.1 += 1;
                found = true;
                break;
            }
        }

        if !found {
            word_count.push((word, 1));
        }
    }

    println!("Result:");
    for (word, count) in word_count.iter() {
        println!("{}: {}", word, count);
    }
}
