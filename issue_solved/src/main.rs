use std::io;

fn main() {
    println!("Temperature: ");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Cook_1");

    let temp_str = &input[..input.len()-1];
    let unit = &input[input.len()-1..];

    let temp = temp_str.parse::<f32>().expect("Cook_2");

    match unit {
        "F" => {
            let celsius = (temp - 32.0) * (5.0 / 9.0);
            println!("{}°F = {}°C", temp, celsius);
        },
        "C" => {
            let fahrenheit = temp * (9.0 / 5.0) + 32.0;
            println!("{}°C = {}°F", temp, fahrenheit);
        },
        _ => println!("Cook_3"),
    }
}