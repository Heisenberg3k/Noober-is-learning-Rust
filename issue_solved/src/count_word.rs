use std::io;

fn count_words(input_string: &str) -> Vec<(String, u32)> {
    let mut words: Vec<(String, u32)> = Vec::new();
    let mut current_word = String::new();

    for &byte in input_string.as_bytes() {
        if byte.is_ascii_alphanumeric() {
            current_word.push(byte as char);
        } else if !current_word.is_empty() {
            if let Some(index) = words.iter().position(|(word, _)| word == &current_word) {
                words[index].1 += 1;
            } else {
                words.push((current_word.clone(), 1));
            }
            current_word.clear();
        }
    }

    words
}

pub fn main() {
    println!("Input:");
    let mut input_string = String::new();
    io::stdin().read_line(&mut input_string).expect("Cannot read -> Cook");

    let word_count = count_words(&input_string);

    println!("Result:");
    for (word, count) in word_count.iter() {
        println!("{}: {}", word, count);
    }
}