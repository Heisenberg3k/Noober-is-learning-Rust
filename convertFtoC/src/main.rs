use std::io;

fn main() {
    println!("Temperature: ");
    let mut str_temp = String::new();
    io::stdin().read_line(&mut str_temp);

    let str_temp: f64 = match str_temp.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Cook");
            return;
        }
    };

    println!("F or C: ");
    let mut str_temp_unit = String::new();
    io::stdin().read_line(&mut str_temp_unit);

    let str_temp_unit = str_temp_unit.trim();

    let result = if str_temp_unit == "F" {
        f_to_c(str_temp)
    } else if str_temp_unit == "C" {
        c_to_f(str_temp)
    } else {
        println!("Cook");
        return;
    };

    println!("Result: {}", result);
}

fn f_to_c(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}

fn c_to_f(celsius: f64) -> f64 {
    celsius * 9.0 / 5.0 + 32.0
}
