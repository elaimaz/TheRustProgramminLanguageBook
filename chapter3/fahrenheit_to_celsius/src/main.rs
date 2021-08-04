// Convert temperatures between Fahrenheit and Celsius.

use std::io;

fn main() {
    println!("Please inform Fahrenheit temperature");

    let mut fahrenheit = String::new();

    io::stdin()
        .read_line(&mut fahrenheit)
        .expect("Failed to read line");
    
    let fahrenheit: f64 = fahrenheit.trim().parse().expect("Please type a number");

    println!("Temperature in Fahrenheit is: {}", fahrenheit);

    let celsius: f64 = ((fahrenheit - 32 as f64) * 5 as f64) / 9 as f64;

    println!("Temperature in Celsius is: {}", celsius);
}
